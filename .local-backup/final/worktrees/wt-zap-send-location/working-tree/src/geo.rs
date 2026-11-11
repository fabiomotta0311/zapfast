//! Reading a location out of the text a reader pastes.
//!
//! A computer has no position of its own to offer and ZapFast asks no service
//! for one, so the location dialog takes the spot as text: a pair of
//! coordinates, or a link to a spot on a map. Everything here is offline
//! parsing, and anything it cannot read with confidence it refuses, so the
//! dialog can keep the send button dark instead of pinning the wrong place.

/// A spot on the earth, in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spot {
    pub latitude: f64,
    pub longitude: f64,
}

impl Spot {
    /// The coordinates as text, dot decimals and five places (about a metre),
    /// which is how coordinates are written everywhere, links included.
    pub fn text(self) -> String {
        format!("{:.5}, {:.5}", self.latitude, self.longitude)
    }

    /// Whether the pair is a real position, so a typo cannot pin the sea.
    fn is_on_earth(self) -> bool {
        self.latitude.is_finite()
            && self.longitude.is_finite()
            && (-90.0..=90.0).contains(&self.latitude)
            && (-180.0..=180.0).contains(&self.longitude)
    }
}

/// Reads the spot in `text`, or `None` when the text is no location.
pub fn spot(text: &str) -> Option<Spot> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    link(text)
        .or_else(|| pair(text))
        .filter(|spot| spot.is_on_earth())
}

/// The link to open for a spot, the one the location card offers too. The
/// browser fetches it, never ZapFast, so no map is ever loaded in the app.
pub fn map_url(spot: Spot) -> String {
    let (latitude, longitude) = (spot.latitude, spot.longitude);
    format!(
        "https://www.openstreetmap.org/?mlat={latitude}&mlon={longitude}#map=16/{latitude}/{longitude}"
    )
}

/// The map a link names, so each provider's own conventions are read only
/// where that provider writes them and an unrelated address is refused
/// rather than guessed at.
#[derive(Clone, Copy, PartialEq)]
enum Provider {
    Google,
    Apple,
    OpenStreetMap,
}

/// Reads the map a link's host names, or `None` when it names none: the
/// dialog promises supported map links, and a spot pinned out of an
/// unrelated URL would be a guess the reader never sees.
fn provider(url: &reqwest::Url) -> Option<Provider> {
    let host = url.host_str()?.to_ascii_lowercase();
    let host = host.strip_prefix("www.").unwrap_or(&host);
    // Google serves each country from its own domain, so the label is what
    // identifies it, and its links come from `maps.google.com` and short
    // hosts alike.
    if host == "google.com"
        || host.ends_with(".google.com")
        || host.split('.').next() == Some("google")
    {
        Some(Provider::Google)
    } else if host == "apple.com" || host.ends_with(".apple.com") {
        Some(Provider::Apple)
    } else if host == "openstreetmap.org" || host.ends_with(".openstreetmap.org") {
        Some(Provider::OpenStreetMap)
    } else {
        None
    }
}

/// Reads a link to a spot: Google Maps, Apple Maps, OpenStreetMap, and any
/// `geo:` URI. Google's place markers name the exact spot, so they win over the
/// viewport around them.
fn link(text: &str) -> Option<Spot> {
    let url = reqwest::Url::parse(text).ok()?;
    if url.scheme() == "geo" {
        // A `geo:` URI may carry standard parameters after the coordinates,
        // as in `geo:0,0;u=35`, which name no spot of their own.
        let coordinates = url.path().split(';').next().unwrap_or(url.path());
        return two(coordinates);
    }
    if !matches!(url.scheme(), "http" | "https") {
        return None;
    }
    match provider(&url)? {
        Provider::Google => marked_place(text)
            .or_else(|| url.path().split_once("/@").and_then(|(_, rest)| two(rest)))
            .or_else(|| {
                query_spot(
                    &url,
                    &[
                        "q",
                        "ll",
                        "query",
                        "daddr",
                        "destination",
                        "center",
                        "loc",
                        "sll",
                    ],
                )
            }),
        Provider::Apple => query_spot(
            &url,
            &["ll", "q", "sll", "center", "query", "daddr", "destination"],
        ),
        // The `mlat`/`mlon` pair is the marker itself, so it is read before
        // the viewport the fragment describes.
        Provider::OpenStreetMap => osm_marker(&url)
            .or_else(|| url.fragment().and_then(openstreetmap_position))
            .or_else(|| query_spot(&url, &["ll", "q", "sll"])),
    }
}

/// Reads the `!3d<latitude>!4d<longitude>` markers a Google Maps link carries.
fn marked_place(text: &str) -> Option<Spot> {
    let latitude = number(between(text, "!3d", "!")?)?;
    let longitude = number(between(text, "!4d", "!")?)?;
    Some(Spot {
        latitude,
        longitude,
    })
}

/// The text between `start` and the next `end`, or to the end of the text.
fn between<'a>(text: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let (_, rest) = text.split_once(start)?;
    Some(rest.split_once(end).map_or(rest, |(head, _)| head))
}

/// Reads `<latitude>/<longitude>` out of an OpenStreetMap `#map=zoom/lat/long`
/// fragment.
fn openstreetmap_position(fragment: &str) -> Option<Spot> {
    let (_, rest) = fragment.strip_prefix("map=")?.split_once('/')?;
    let rest = rest.split('&').next().unwrap_or(rest);
    let mut parts = rest.split('/');
    joined(parts.next()?, parts.next()?)
}

/// Reads OpenStreetMap's own `mlat` and `mlon` pair, the marker its links
/// carry before the `#map=` viewport.
fn osm_marker(url: &reqwest::Url) -> Option<Spot> {
    let mut latitude = None;
    let mut longitude = None;
    for (key, value) in url.query_pairs() {
        match key.as_ref() {
            "mlat" => latitude = number(&value),
            "mlon" => longitude = number(&value),
            _ => {}
        }
    }
    Some(Spot {
        latitude: latitude?,
        longitude: longitude?,
    })
}

/// Reads a spot out of a link's query, under the keys `keys` names. Each map
/// writes its marker under its own key, and a value that is not a pair of
/// coordinates is a place name, which this refuses.
fn query_spot(url: &reqwest::Url, keys: &[&str]) -> Option<Spot> {
    for (key, value) in url.query_pairs() {
        if keys.contains(&key.as_ref())
            && let Some(spot) = pair(&value)
        {
            return Some(spot);
        }
    }
    None
}

/// Reads the first two comma-separated numbers of a chunk, which is how a
/// link's own `latitude,longitude,zoom` carries them.
fn two(chunk: &str) -> Option<Spot> {
    let mut parts = chunk.split(',');
    let latitude = number(parts.next()?)?;
    let longitude = number(parts.next()?)?;
    Some(Spot {
        latitude,
        longitude,
    })
}

/// Reads two numbers written as coordinates: `-23.55 -46.63`, `-23.55, -46.63`
/// or `-23,5505,-46,6333`. A comma is the decimal mark in much of the world and
/// the separator in the rest, so a text with several commas is read only when
/// one reading alone is possible.
fn pair(text: &str) -> Option<Spot> {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    match tokens.as_slice() {
        [latitude, longitude] => joined(latitude.strip_suffix(',').unwrap_or(latitude), longitude),
        [single] => {
            let mut readings = single
                .char_indices()
                .filter(|(_, character)| *character == ',')
                .filter_map(|(index, _)| joined(&single[..index], &single[index + 1..]));
            let first = readings.next()?;
            readings.all(|other| other == first).then_some(first)
        }
        _ => None,
    }
}

/// Reads two numbers that are already separate apart from their own spacing.
fn joined(latitude: &str, longitude: &str) -> Option<Spot> {
    Some(Spot {
        latitude: number(latitude)?,
        longitude: number(longitude)?,
    })
}

/// Reads one number, with a dot or a comma as the decimal mark. `1.234,56` is
/// read the way it is written in much of the world: the dot groups thousands.
/// A comma after the number is a separator, as in `-23.55, -46.63`.
fn number(token: &str) -> Option<f64> {
    let token = token.trim();
    match token.strip_suffix(',') {
        Some(bare) => parse(bare).or_else(|| parse(token)),
        None => parse(token),
    }
}

fn parse(token: &str) -> Option<f64> {
    if token.is_empty() {
        return None;
    }
    let negative = token.starts_with('-');
    let digits = token.strip_prefix(['-', '+']).unwrap_or(token);
    let normalized = match (digits.contains('.'), digits.contains(',')) {
        (true, true) => digits.replace('.', "").replace(',', "."),
        (false, true) => digits.replace(',', "."),
        _ => digits.to_owned(),
    };
    if normalized.is_empty()
        || normalized.matches('.').count() > 1
        || !normalized
            .chars()
            .all(|character| character.is_ascii_digit() || character == '.')
    {
        return None;
    }
    let value: f64 = normalized.parse().ok()?;
    value
        .is_finite()
        .then_some(if negative { -value } else { value })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spot_of(text: &str) -> (f64, f64) {
        let spot = spot(text).unwrap_or_else(|| panic!("{text} is a location"));
        (spot.latitude, spot.longitude)
    }

    #[test]
    fn coordinates_are_read_with_a_comma_or_a_space_between_them() {
        assert_eq!(spot_of("-23.5505, -46.6333"), (-23.5505, -46.6333));
        assert_eq!(spot_of("-23.5505 -46.6333"), (-23.5505, -46.6333));
        assert_eq!(spot_of("51.5074,  -0.1278"), (51.5074, -0.1278));
        assert_eq!(spot_of(" -23.5505, -46.6333 "), (-23.5505, -46.6333));
        assert_eq!(spot_of("52.5200,13.4050"), (52.52, 13.405));
    }

    /// Much of the world writes the decimal mark as a comma, which collides
    /// with the separator: the reading has to survive both.
    #[test]
    fn decimal_commas_are_read_when_only_one_reading_is_possible() {
        assert_eq!(spot_of("-23,5505,-46,6333"), (-23.5505, -46.6333));
        assert_eq!(spot_of("-23,5505 -46,6333"), (-23.5505, -46.6333));
        // The dot groups thousands into a pair no position can be, so the
        // pair is read and then refused: `number` is where that reading is
        // checked.
        assert_eq!(spot("1.234,5, 8.765,4"), None);
        // Two different readings fit, so neither is taken.
        assert_eq!(spot("12,5,7"), None);
        // One reading fits here, the middle comma separating the pair, and the
        // dialog shows the spot it read before anything goes out.
        assert_eq!(spot_of("1,2,3,4"), (1.2, 3.4));
    }

    #[test]
    fn the_pair_has_to_be_a_position_on_the_earth() {
        assert_eq!(spot("91.0, 0.0"), None);
        assert_eq!(spot("-91.0, 0.0"), None);
        assert_eq!(spot("0.0, 181.0"), None);
        assert!(spot("-0.0, -180.0").is_some());
        assert!(spot("90.0, 180.0").is_some());
        assert_eq!(spot("nan, 12.0"), None);
        assert_eq!(spot("inf, 12.0"), None);
    }

    #[test]
    fn text_that_is_no_pair_of_numbers_is_refused() {
        assert_eq!(spot(""), None);
        assert_eq!(spot("   "), None);
        assert_eq!(spot("somewhere near the office"), None);
        assert_eq!(spot("-23.5505"), None);
        assert_eq!(spot("12 34 56"), None);
        assert_eq!(spot("--23.5505, -46.6333"), None);
    }

    #[test]
    fn google_maps_links_are_read_from_the_place_markers() {
        assert_eq!(
            spot_of(
                "https://www.google.com/maps/place/Sao+Paulo/data=!4m2!3m1!1s0x0!3d-23.5505!4d-46.6333"
            ),
            (-23.5505, -46.6333)
        );
        // The viewport around the marker is the fallback when there is no marker.
        assert_eq!(
            spot_of("https://www.google.com/maps/@-23.5505,-46.6333,15z"),
            (-23.5505, -46.6333)
        );
        assert_eq!(
            spot_of("https://maps.google.com/?q=-23.5505,-46.6333"),
            (-23.5505, -46.6333)
        );
        assert_eq!(
            spot_of("https://www.google.com/maps/dir/?api=1&destination=-23.5505,-46.6333"),
            (-23.5505, -46.6333)
        );
    }

    #[test]
    fn apple_and_openstreetmap_links_are_read() {
        assert_eq!(
            spot_of("https://maps.apple.com/?ll=-23.5505,-46.6333&q=Office"),
            (-23.5505, -46.6333)
        );
        assert_eq!(
            spot_of(
                "https://www.openstreetmap.org/?mlat=-23.5505&mlon=-46.6333#map=16/-23.55/-46.63"
            ),
            (-23.5505, -46.6333)
        );
        assert_eq!(
            spot_of("https://www.openstreetmap.org/#map=15/-23.5505/-46.6333"),
            (-23.5505, -46.6333)
        );
        assert_eq!(spot_of("geo:-23.5505,-46.6333"), (-23.5505, -46.6333));
    }

    #[test]
    fn a_link_without_a_spot_in_it_is_refused() {
        assert_eq!(spot("https://www.google.com/maps/place/Sao+Paulo"), None);
        assert_eq!(spot("https://example.com/"), None);
        assert_eq!(spot("https://www.openstreetmap.org/way/123"), None);
    }

    /// A `geo:` URI may carry standard parameters after the coordinates, as
    /// in `;u=40`. They name no spot, so the coordinate component alone is
    /// read.
    #[test]
    fn a_geo_uri_is_read_before_its_parameters() {
        assert_eq!(spot_of("geo:-23.5505,-46.6333;u=40"), (-23.5505, -46.6333));
        assert_eq!(spot_of("geo:0,0;crs=wgs84;u=35"), (0.0, 0.0));
    }

    /// The dialog promises supported map links, so each map's own syntax is
    /// read on that map and nowhere else: an unrelated URL must not be able to
    /// pin coordinates.
    #[test]
    fn an_unrelated_address_cannot_pin_a_spot() {
        assert_eq!(spot("https://example.com/?q=12,34"), None);
        assert_eq!(spot("https://example.com/?ll=12,34"), None);
        assert_eq!(spot("https://example.com/?mlat=12&mlon=34"), None);
        // Google's markers and viewport are Google's, not everyone's.
        assert_eq!(spot("https://example.com/maps/@12,34,15z"), None);
        assert_eq!(spot("https://example.com/place/X/data=!3d12!4d34"), None);
        // A coordinate pair in an OpenStreetMap fragment is OpenStreetMap's.
        assert_eq!(spot("https://example.com/#map=15/12/34"), None);
    }

    /// Each supported map keeps reading its own links the way it did.
    #[test]
    fn every_supported_map_still_reads_its_own_links() {
        assert_eq!(
            spot_of("https://www.google.com.br/maps/place/X/data=!3d-23.5505!4d-46.6333"),
            (-23.5505, -46.6333)
        );
        assert_eq!(
            spot_of("https://maps.apple.com/?ll=-23.5505,-46.6333"),
            (-23.5505, -46.6333)
        );
        assert_eq!(
            spot_of("https://www.openstreetmap.org/?mlat=-23.5505&mlon=-46.6333"),
            (-23.5505, -46.6333)
        );
    }

    /// The link the card opens is one the reader can paste back in.
    #[test]
    fn the_map_link_round_trips_through_the_reader() {
        let here = Spot {
            latitude: -23.5505,
            longitude: -46.6333,
        };
        assert_eq!(spot(&map_url(here)), Some(here));
        assert_eq!(here.text(), "-23.55050, -46.63330");
    }

    #[test]
    fn one_number_carries_a_sign_and_one_decimal_mark() {
        assert_eq!(number("  -12.5 "), Some(-12.5));
        assert_eq!(number("+12.5"), Some(12.5));
        assert_eq!(number("-12,5"), Some(-12.5));
        assert_eq!(number("-12.5,"), Some(-12.5));
        assert_eq!(number("12,"), Some(12.0));
        assert_eq!(number("12.5.5"), None);
        // A dot groups thousands, as it is written in much of the world.
        assert_eq!(number("1.234,5"), Some(1234.5));
        assert_eq!(number("1.234"), Some(1.234));
        assert_eq!(number("-"), None);
        assert_eq!(number(""), None);
        assert_eq!(number("1e3"), None);
    }
}

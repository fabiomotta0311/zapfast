# Fila de trabalho — ZapFast (`fabiomotta0311` → upstream `crmne/zapfast`)

> **Antes de compilar/testar qualquer coisa:** leia
> `C:/Users/fabio/dev/zapfast-local-environment.md` (premissas do ambiente local,
> como reconstruir o shim de perl e o `make.exe`, e todas as armadilhas já
> exploradas). O resumo é `source /c/Users/fabio/dev/zapfast-localenv.sh`.

Regra da casa: **cada item vira UMA PR pequena**, com passos de reprodução e
evidência de teste. O CI do GitHub é o verificador final, mas desde 23/09 o
build local funciona: `source ~/dev/zapfast-localenv.sh` dá `check`, `clippy -D
warnings`, `test` e `doc` em minutos (o próprio script documenta por que cada
variável existe). Rodar a lista de checks do `AGENTS.md` antes de subir deixou de
ser impossível e passa a ser o padrão.

Disco: com o volume cheio o build falha como `dlltool ... Permission denied` e
`Espaço insuficiente no disco`, que parecem problema de toolchain e não são.
`df -h /c` antes de um build grande; `cargo clean` libera ~5 GB e
`cargo sweep --time 14` rotaciona o cache.

**Rebase:** antes de tocar num PR antigo, `git rebase origin/main` no worktree dele; conflitos
típicos são blocos de teste no fim de `src/app.rs` (manter os dois) e novas seções no `README.md`.
Depois do rebase, rode `cargo fmt --all` **antes** do `git add` (senão o commit sai sem formatação e o
`fmt --check` do CI falha), confira `git status --porcelain` vazio e `git push --force-with-lease fork`.

**Premissa de escrita (trazida da sessão de PRs para a Nous/Hermes):** o corpo da PR
é a porta de entrada, então ele vende o valor de negócio antes do detalhe técnico:
`## Why this is worth having` → `## What this adds` → `## Steps to reproduce` →
`## Tests` → `## Limits and trust model` → `## Design notes` →
`## Evidence on this head` (SHA do head + o que o CI reportou nele). A documentação
que a mudança toca (`docs/_guide/using-zapfast.md`,
`docs/_reference/settings-and-files.md`) recebe o mesmo tratamento. Sem claims não
comprovados e sem citar concorrentes. As 15 PRs abertas foram reescritas nesse
formato em 23/09.
Worktree por item em `C:/Users/fabio/dev/wt-*`, sempre recriado a partir de
`origin/main` recém-baixada. Nunca abrir issue/PR duplicada.

## Resultado de 23/09/2026 (noite) — as duas PRs novas foram aceitas para a 0.16

O `crmne` **terminou as duas na nossa branch** (mesmo padrão de #141/#154/#150):
fez merge do `main`, commitou os ajustes dele e comentou "Thanks, Fabio! Finished
for 0.16 on your branch". As duas estão **8/8 verdes** nos heads dele.

- **#167 open the message a notification announced** — head dele `4a46491`.
  O salto passa a contar como posicionar o divisor de não lidas (antes o frame
  seguinte rolava de volta para o divisor, e o clique podia cair longe da
  mensagem); `OpenMessage` para quando o chat não abre, então chat trancado não
  move o anchor nem carrega histórico; lock envenenado tratado no `on_activated`;
  e **corrigiu uma frase falsa minha no README** (escrevi que o Windows também
  dispensa as notificações do chat ao ler: só o Linux faz).
- **#168 keep playing voice messages one after another** — head dele `17b86c7`.
  A sequência segue **só as vozes consecutivas logo depois da que terminou** e
  **qualquer outra mensagem a encerra** (arquivo de áudio incluído, como no
  WhatsApp); ela pertence à conversa (sair do chat, tocar um vídeo ou escolher
  outro clipe encerra); download que falha encerra; **a pausa da mídia dos outros
  apps sobrevive ao download** do próximo clipe, com prazo de 10 s e `wake_after`,
  em vez de soltar e pausar a música entre clipes; buscar áudio passa a parar o
  vídeo que estivesse tocando.

Lição de processo: **nunca force-push numa branch de PR onde ele já commitou** —
ele empurra commits (e merges do `main`) na nossa branch, e um force-push apagaria
o trabalho dele. Antes de mexer, `git fetch` e `git show <sha>`: os commits dele
são a especificação real da feature.

Ofereci abrir a PR de split view (ver "Pedidos de continuação"); aguardando
resposta dele.

Nenhuma das duas tinha iniciativa prévia (conferido em issues, PRs, histórico do
git e `PRODUCT.md`).

## Resultado de 23/09/2026 — 12 mergeadas, 3 fechadas por política

O `crmne` mergeou 12 das 15 PRs abertas no mesmo dia (0.16). Nenhuma ficou aberta.

| PR | O que é | Resultado |
|----|---------|-----------|
| #141 | Idioma do sistema (6 catálogos) | MERGED — ele terminou na nossa branch (corrigiu o panic de fatiar 3 bytes no nome do mês em alemão/russo) |
| #124 | Relógio 12h/24h | MERGED |
| #140 | Fim do flicker de mídia/animação | MERGED |
| #120 | 1,25x e 1,75x no áudio | MERGED |
| #115 | Prévia de imagem no app | MERGED |
| #149 | Ctrl+F dentro da conversa | MERGED |
| #148 | Rascunhos no archive criptografado | MERGED |
| #116 | Botão de reação no hover | MERGED |
| #151 | Lista colapsada em avatares | MERGED |
| #154 | Etiquetas (+ abas + split view) | MERGED só a parte de etiquetas: viraram chips na linha de filtros com a preferência `label_chips`; **split view ficou de fora** |
| #143 | Live location nativo | MERGED (ele pediu screenshots antes) |
| #150 | Grupos de figurinhas | MERGED como **local packs** com chave por conteúdo (`pack.json` + cópia de cada figurinha nomeada pelo hash), lista ao lado da grade e strings traduzidas |
| #142 | Transcrição por provedor remoto | CLOSED — ZapFast não manda conteúdo de mensagem para terceiros; se for local (Whisper no dispositivo) abrir **issue primeiro** |
| #121 | Mensagens interativas | CLOSED — superseded pelo #113 (cards nativos + envio de resposta) |
| #152 | Recibos de leitura por conversa | CLOSED — o WhatsApp só tem a configuração global, e o ZapFast evita comportamento que os apps oficiais não têm para não parecer cliente modificado |
| #153 | Anti-revoke | CLOSED — "dangerous territory" |

### Pedidos de continuação registrados

- **Split view**: fazer como PR própria em cima do `main`. Defeitos citados: os dois painéis criam composer e lista com os mesmos ids no egui (input pode ir para o painel errado), a lista de conversas desaparece e cada composer fica estreito demais (o placeholder quebra letra por letra). Ele ainda está decidindo se split view cabe no ZapFast.
- **Transcrição local** (Whisper no dispositivo): abrir issue primeiro.

### Lição de processo

Ele pediu **screenshots** antes de revisar (#143 e #154). Gerar com `--demo-page <page>` sobre chats sintéticos e colar no corpo da PR.

## #173 (24/09) — a suíte não pode depender do idioma da máquina

Achado ao rodar a suíte localmente pela primeira vez: **3 testes falhavam por
locale** (a i18n mergeada hoje faz a interface seguir o idioma do sistema, e os
testes comparam strings em inglês). `our_own_chat_is_titled_and_found_by_our_name`
esperava `"You"` e recebia `"Você"`; o toast de recusa vem traduzido; o teste do
demo procura a linha `"Message info"`.

Correção em [#173](https://github.com/crmne/zapfast/pull/173) (branch
`fix/tests-do-not-depend-on-the-machine-locale`, worktree `wt-zap-locale`, head
`19db5e2`, draft): `i18n::detect()` devolve inglês quando `cfg!(test)`, e uma
preferência explícita continua vencendo (builds normais, incluindo `--demo-page`,
seguem lendo o sistema). Fechar na fonte evitou tocar ~20 chamadas de
`App::headless` nos testes.

**Evidência local (máquina pt-BR):** `697 passed; 0 failed; 9 ignored` (antes
`693 passed; 3 failed`), `clippy --all-targets -D warnings` sem avisos em 8m28s,
`fmt --check` limpo.

## #300+ (29/09) — live location recebido não pode ser "seguido" aqui

Pedido do usuário: "adicionar a funcionalidade de compartilhar localização em
tempo real no zapfast; atualmente mesmo que a pessoa compartilhe em tempo real,
vem fixa".

**O que o protocolo permite:** nada, no lado de seguir a posição. O WhatsApp
mantém as posições de um live location no telefone: dispositivos vinculados
recebem um `placeholderMessage` (MASK_LINKED_DEVICES) e só o `mediatype` da
stanza diz que é live location — é o achado que o próprio maintainer registrou
no commit `5849826` (por isso a bolha "Open WhatsApp on your phone to follow
it."). O WhatsApp Desktop oficial diz o mesmo na cara do usuário: "Live
Location is not available on this device. View location on your phone." A FAQ
da Meta fecha: *"Live location isn't supported on companion phones"* e *"You
can only share your live location from your main WhatsApp device"* (localização
**atual**, estática, um vinculado até pode enviar). Ou seja: a posição inicial
chega (é ela que desenha o card parado) e as seguintes não saem do telefone.
Nada de código faz o pin andar; não confundir com bug.

**Feito (branch, PR não aberta):** worktree `wt-zap-live-follow`, branch
`feat/live-location-not-followable-here`, commit `c1eeccb`, no fork. Uma posição
mascarada que continua um share já guardado como **card** passa a marcar o card
(`Content::LiveLocation::newer_on_phone`, serde-additive) em vez de criar uma
segunda bolha embaixo dele; o card passa a dizer "Open WhatsApp on your phone to
follow it."; uma posição legível limpa a marca. `location_card` ganhou um
parâmetro `note`; a amostra `ada-live-now` do demo carrega o estado novo; README
documenta. Nenhuma string i18n nova (reusa o msgid existente). Teste novo:
`a_masked_position_marks_the_share_it_cannot_move`.

**Feito 2 (29-30/09):** worktree `wt-zap-live-follow`, branch
`feat/live-location-not-followable-here`, head `9e3c420` no fork. Gates nativos
no VPS (`/docker/zapfast-builder/gates.sh`, Linux): rodada anterior foi
interrompida no clippy por falta de lock (havia um clippy antigo do mesmo alvo
segurando o diretório); refazer antes de abrir a PR.

**PR ABERTA E VERDE:** https://github.com/crmne/zapfast/pull/302 (head `a07f3ab`,
6 arquivos, +123/-6). Gates nativos no VPS na head final: fmt limpo, clippy
`-D warnings` em todos os alvos, 971 testes passando (966 na lib, 3 e 2 nos
outros alvos), 0 falhas. CI do GitHub: 8/8 checks passando (quality, test em
ubuntu/macos/windows, compile Windows arm64, Nix x86_64/aarch64, advisories).
Screenshots do card (dark + light) anexados no corpo via branch `screens` do
fork; estado MERGEABLE.

## #300+ (29-30/09) — enviar localização (slice 2, FEITO, PR não aberta)

O usuário autorizou implementar o envio de localização **estática** ("se o
zapfast ainda não tem o envio de localização estática, ok pode implementar"),
já que o compartilhamento em movimento não é permitido a um dispositivo
vinculado.

Worktree `wt-zap-send-location`, branch `feat/send-a-location`, head `5131473`
no fork. 22 arquivos, +830/-9:

- Menu "+" → **Send location**; diálogo novo em `src/ui/locations.rs` (mesmo
  padrão de `ui/polls.rs`), com o pin `Icon::MapPin`.
- `src/geo.rs` (novo): lê a posição colada como texto — par de coordenadas
  (vírgula ou espaço; ponto ou vírgula decimal) ou link (Google Maps com
  marcadores `!3d/!4d`, viewport `@`, `q`/`ll`/`daddr`/`destination`,
  Apple Maps, OpenStreetMap `#map=` e `mlat`/`mlon`, `geo:`). Marcador vence
  viewport; texto com mais de uma leitura (`12,5,7`) é recusado; par fora da
  Terra recusado. `geo::map_url` virou a fonte única do link do card.
- Envio: `Command::SendLocation`/`Action::SendLocation` no caminho do
  `SendText`; `Unsent::Location` devolve o spot ao diálogo na recusa; a
  mensagem vai sem nome de lugar e sem flag de live; o card de localização sem
  endereço passa a mostrar as coordenadas.
- Testes novos: geo (10), worker (proto/classificação/citação/recusa offline),
  app (envio fecha o diálogo, recusa reabre), demo (menu com 3 linhas + o
  diálogo no teste de foco/escape). README, guia e 3 msgids i18n nos 7
  catálogos; `--demo-page location`.

Gates nativos no VPS (log `/docker/zapfast-builder/gates-send-location.log`):
`fmt` OK; clippy pegou 2 erros reais em `src/geo.rs` (um `?` a mais em `two()`
e um teste que chamava `spot.text()` em vez de `here.text()`), corrigidos em
`5131473`. Rodada final em andamento.

**PR ABERTA E VERDE:** https://github.com/crmne/zapfast/pull/301 (head `d1ee019`,
22 arquivos, +842/-9). Gates nativos no VPS na head final: fmt limpo, clippy
`-D warnings`, 984 testes passando (979 na lib), 0 falhas. CI do GitHub: 8/8
checks passando. Screenshots do diálogo (dark + light) no corpo, mesmo esquema
da branch `screens`. Estado MERGEABLE. Sem issue upstream para referenciar.

## Fila (próximos itens, em ordem)


### 1. Modo compacto da lista de conversas — FEITO (PR #151)

Implementado: `SidebarDisplayMode` em `src/model.rs`; preferência
`collapse_chat_list` em `src/settings.rs` + toggle nas Configurações;
`App::sidebar_mode()`; `Action::ToggleSidebar` passa a esconder/recolher;
`chats::compact_show` (rail de avatares, badge de não lidas, tooltip, clique
abre, barra de destaque no ativo); `standalone_header` só quando o modo é
`Hidden`; docs no guia. Testes: transições de estado (`src/app.rs`) e rail
estreito + clique abre (`src/ui/chats.rs`).

**Escopo funcional.** Quando o usuário aciona **Hide the chat list** (hoje
`Action::ToggleSidebar`, botão em `src/ui/chats.rs`), em vez de esconder a
lista por completo, recolher a sidebar para uma **barra vertical só de
avatares**.

- **Preferência** nas Configurações (ex.: "Ao ocultar a lista de conversas,
  recolher para barra de ícones"):
  - desativada → comportamento atual (oculta tudo);
  - ativada → o toggle recolhe para a barra compacta.

**Barra compacta.**
- Só avatares (ou iniciais / ícone padrão) dos chats ativos/fixados, em coluna.
- Badge com contador de não lidas no canto inferior direito do avatar, em
  vermelho/destaque; **sem badge quando `unread == 0`** (`Chat::unread: u32`).
- Tooltip nativo no hover com o nome do contato/grupo.
- Clique no ícone abre a conversa na área principal.
- Indicador discreto (ponto/barra) do chat ativo.

**Estados da sidebar** (pedido do usuário, verbatim):

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SidebarDisplayMode {
    Expanded,            // Lista completa com texto, preview e horário
    CollapsedIconsOnly,  // Modo compacto estilo Discord (apenas avatares + badges)
    Hidden,              // Totalmente oculto (se a preferência estiver desmarcada)
}
```

**Pontos de entrada já localizados no código (base `c17b900`):**
- `src/app.rs:238` `pub sidebar_visible: bool` (init em `:442`); toggle em
  `:2465` (`Action::ToggleSidebar`) e `:2479` (reabre ao abrir conversa).
- `src/ui/chats.rs:147` e `:230` — botão "Hide the chat list (Ctrl+B)"/"⌘B"
  que hoje empurra `Action::ToggleSidebar`.
- `src/model.rs:74` `pub unread: u32` em `Chat` (fonte do badge).
- `src/settings.rs` (`struct Settings`) + `src/ui/settings.rs` (diálogo que
  empurra `Action::SettingsChanged`) — onde entra a preferência nova.
- `src/ui/widgets.rs` — helpers de menu/estilo; `src/theme.rs` — ícones.

**Critérios de pronto.**
1. Preferência ligada + toggle → a lista vira a barra de avatares; desligada →
   comportamento atual inalterado.
2. Badge só aparece com não lidas > 0 e reflete `Chat::unread`.
3. Hover mostra o nome; clique abre a conversa; chat ativo destacado.
4. Testes: estado (modo resultante do toggle conforme a preferência) + um
   teste puro de render/entrada da barra compacta; `cargo fmt --all --check`.
5. PR pequena com passos de reprodução; declarar honestamente o que não foi
   executado localmente.

### 2. Recibos de leitura granulares por conversa — "Per-Chat Read Receipts Override"

**Status: IMPLEMENTADO — PR [#152](https://github.com/crmne/zapfast/pull/152)**
(`feat/per-chat-read-receipts`, worktree `wt-zap-receipts`, base `origin/main`
`c17b900`). Tri-state `ReadReceiptPreference::{InheritGlobal, Explicit(bool)}`
em `src/model.rs`; tabela `chat_read_receipts` no **archive criptografado**
(`src/archive/read_receipts.rs`) — linha só existe para chats que diferem do
global; `Command::SetReadReceipts` / `Event::ReadReceipts`; item no menu ⋮ da
conversa ("Read receipts: default (on/off) | always on | always off", clique
cicla); `mark_read` e o recibo de voz usam o valor resolvido. O gate de
privacidade da conta (`receipts_allowed`, vindo do telefone) continua
soberano — "always on" não fura a privacidade da conta. Testes: archive
(round-trip, remoção de linha, clear do chat), app (override vence o global e
chega no `MarkRead`; voltar para default manda `None`).

**Regra de negócio (precedência).** O app já tem a global *Send read receipts*
(`Settings::send_read_receipts`, `src/settings.rs:75`). Cada conversa ganha um
valor opcional que **sobrescreve** a global quando definido; sem valor definido
ela **herda** a global. O ack só é enviado ao protocolo se a resolução final
daquela conversa for `true`.

**Modelo (tri-state), verbatim do pedido:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadReceiptPreference {
    InheritGlobal,  // segue as configurações globais
    Explicit(bool), // true: força envio | false: bloqueia envio
}

impl Default for ReadReceiptPreference {
    fn default() -> Self {
        Self::InheritGlobal
    }
}
```

**UI.** No menu da conversa (três pontinhos `⋮`, `fn header` em
`src/ui/conversation.rs:90`), item "Send read receipts" com switch, indicando o
estado: **Padrão global: Ativado/Desativado**, **Ativado** ou **Desativado**.
Clicar alterna e persiste na hora; o efeito vale já para as mensagens recebidas
depois da mudança, sem reabrir o chat nem reiniciar o app.

**Onde plugar (base `c17b900`).**
- `src/model.rs` — `ReadReceiptPreference` + `Action::SetReadReceipts { chat, preference }`
  (o modelo já tem `Action::MarkRead` etc.).
- Persistência: estado é local, não vem do telefone — tabela própria no archive
  criptografado (padrão de `src/archive/drafts.rs`/`sticker_groups.rs`:
  `chat_read_receipts (chat TEXT PRIMARY KEY, receipts INTEGER)`), com
  `DELETE` no purge/`Archive::clear()`; leitura via `Event` próprio (padrão de
  `Event::Drafts`) para não sobrecarregar `Event::Chats`.
- Resolução: os dois pontos que montam o comando ack hoje usam a global direto —
  `src/app.rs:1699` e `src/app.rs:3091` (`receipts: self.settings.send_read_receipts`).
  Passar a resolver `preference.resolve(global)` antes de enviar
  `Command::MarkRead { chat, receipts }`.
- Worker: `fn mark_read(&mut self, chat, receipts: bool)`
  (`src/backend/worker.rs:3678`) já só chama `send_read_receipts` quando
  `receipts` é `true` (`:3697` → `:3740` `client.mark_as_read`), então **nenhuma
  mudança de protocolo é necessária** — só a fonte do booleano.

**Critérios de pronto.**
1. Conversa em `InheritGlobal` segue a global; `Explicit(true)` envia mesmo com a
   global desligada; `Explicit(false)` não envia mesmo com a global ligada.
2. Voltar de `Explicit(..)` para `InheritGlobal` restaura o comportamento global.
3. Menu mostra o estado corrente (incluindo "Padrão global: …") e persiste.
4. Sobrevive a reinício do app e `Archive::clear()` limpa as linhas.
5. Testes: resolução tri-state (unidade) + um teste de que o comando ack leva o
   booleano resolvido; `cargo fmt --all --check`; PR pequena com passos de
   reprodução.

---

### 3. Anti-Revoke / Proteção contra eliminação de mensagens recebidas

**Status: FECHADA PELO MAINTAINER — PR [#153](https://github.com/crmne/zapfast/pull/153)
está `CLOSED` por `crmne` em 23/09/2026 06:57 UTC, com o comentário "This is threading
on dangerous territory. I'd keep it out."** A implementação ficou pronta e verde no
CI, mas a funcionalidade está fora do escopo upstream: manter mensagens que o
remetente apagou é decisão de produto, não de código. Não reabrir; se o usuário
quiser, o código segue no worktree `wt-zap-anti-revoke` (`feat/anti-revoke`).
(`feat/anti-revoke`, worktree `wt-zap-anti-revoke`, base `origin/main` `c17b900`,
commit `ef0e4b6`). Preferência **"Keep messages the sender deleted"** (default
`false`), coluna `messages.revoked_by_sender` migrada por `MIGRATIONS`, flag em
`Message`, `Command::SetKeepDeleted` enviado no attach e a cada mudança,
`Worker::revoke_for_everyone` nos dois pontos de revogação (REVOKE ao vivo e
lista `revoked` do history sync), rótulo "Deleted by sender" no slot do
"Forwarded" (texto, imagem, áudio, documento, vídeo), mensagem de demo e docs.
Desligar a preferência não apaga o que já foi mantido. Revogação **enviada**
(`Command::Revoke`) continua escrevendo a lápide.

**Regra de negócio.** Configuração global nova nas Configurações — *Keep messages
the sender deleted* (default proposto: `false`, ver nota). Ligada, uma revogação
**recebida** não apaga a mensagem local nem o respetivo ficheiro de mídia; a
mensagem ganha uma flag local e um rótulo discreto no topo da bolha. Desligada,
mantém-se o comportamento atual (o conteúdo vira `Content::Revoked`).

**Onde está hoje (base `c17b900`, recon lido).**
- Revogação **recebida**: `src/backend/worker.rs:5408` monta `revoked: Vec<String>`
  a partir do `ProtocolMessage` (REVOKE) e o conteúdo é substituído — `worker.rs:1662`
  e `worker.rs:2381-2384` chamam `set_content(&chat, &id, &Content::Revoked, false)`.
  Nenhum desses caminhos apaga o ficheiro de mídia: o `content` é sobrescrito e o
  ficheiro fica órfão em disco.
- `src/model.rs:292` `Content::Revoked`; `src/model.rs:403` o rótulo atual
  ("This message was deleted").
- Revogação **enviada** (`Command::Revoke`, `worker.rs:4025`; `app.rs:2137`) fica
  fora do escopo — a feature é só sobre o que o outro apaga.
- Esquema atual: `messages (chat, id, sender, sender_name, from_me, timestamp,
  content, status, quoted, reactions, edited, raw)` — **não existe** coluna de
  revogação.
- Rótulo "forwarded": o slot no topo da bolha vive em `src/ui/conversation.rs`
  (header da bolha). O indicador novo entra exatamente nesse lugar, para todos os
  tipos de conteúdo (texto, imagem, PTT/áudio, documento, vídeo).

**Onde plugar.**
- Coluna nova idempotente em `messages`: `revoked_by_sender INTEGER NOT NULL DEFAULT 0`,
  com migração `ALTER TABLE` guardada por `PRAGMA table_info` (mesmo cuidado do
  `edited`/UPSERT) em `src/archive.rs`.
- `Message` ganha `revoked_by_sender: bool` (`src/model.rs`) e o `content` original
  **nunca** é sobrescrito quando a preferência está ligada.
- Preferência em `src/settings.rs` (default `false`) + toggle em `src/ui/settings.rs`.
- Worker: nos dois pontos de `set_content(..., &Content::Revoked, ...)`, consultar a
  preferência e, quando ligada, escrever só a flag + reemitir a mensagem.
- UI: rótulo secundário/itálico no topo da bolha — "Deleted by sender" (a UI é em
  inglês) — e a flag sobrevive ao history sync que reenvia a lista de revogadas.
- `src/demo.rs` já tem mensagens `Content::Revoked` (`demo.rs:696`, `2320`):
  acrescentar uma com a flag para a feature aparecer no modo demo.
- Docs: `docs/_guide/using-zapfast.md` + `docs/_reference/settings-and-files.md`.

**Conflito a resolver com a auditoria (item 2 da lista abaixo).** "Remover mídia
descriptografada ao excluir/revogar" e o anti-revoke contradizem-se: com a
preferência **ligada** a mídia tem de ficar; **desligada**, segue o comportamento
atual. Ao implementar a limpeza de mídia, pular mensagens com `revoked_by_sender`.

**Nota de design.** O pedido não fixa o default do booleano. Proposta: `false`
(o protocolo manda; ligar é escolha explícita). `true` por padrão muda o
comportamento de todos os usuários sem aviso e é o tipo de coisa que o mantenedor
recusa — confirmar com o usuário antes de abrir a PR.

**Critérios de pronto.**
1. Ligada: revogação recebida mantém conteúdo e arquivo de mídia; a bolha mostra o
   rótulo discreto em texto, imagem, áudio, documento e vídeo.
2. Desligada: comportamento atual (conteúdo vira `Content::Revoked`).
3. A flag sobrevive a reinício e ao history sync.
4. Testes: archive (flag persiste; migração não perde linhas) + app/worker
   (preferência ligada não sobrescreve `content`); `cargo fmt --all --check`.
5. PR pequena, com passos de reprodução e declaração honesta do que não rodou local.

---

### 4. Etiquetas (Labels) + Abas no topo + Split View estilo VS Code — EM PR (#154, draft)

Implementado numa PR única, como pedido: tabelas `labels` + `chat_labels` no
archive (`src/archive/labels.rs`, teto de 20, nome único sem diferenciar
maiúsculas, `#rrggbb` normalizado); `Label`/`ChatPaneState`/`ViewLayoutMode` e
`Chat.labels` no modelo; comandos `CreateLabel`/`UpdateLabel`/`DeleteLabel`/
`SetChatLabels` e `Event::Labels` no backend; estado de painéis, filtro por
etiqueta e contadores em `src/app.rs`; barra de abas + menu + gerenciador em
`src/ui/labels.rs`; split do workspace (dois painéis, cada um com lista e
conversa próprias, divisor arrastável, × no divisor) em `src/ui/mod.rs`;
preferência `labels_as_tabs`; docs. Ver o corpo da PR para o que divergiu do
esboço e por quê.

**Status: NA FILA** — são duas partes (etiquetas primeiro), com três decisões abertas.

#### Parte A — Sistema de etiquetas (pré-requisito do resto)

**Regra de negócio.** CRUD de etiquetas **locais**: nome personalizado, cor em
hexadecimal/RGBA e limite de **20 etiquetas** por conta (paridade com o WhatsApp
Business). Associação **N:N** entre chat e etiqueta (um chat pode ter várias).
Não há sincronização de protocolo: o WhatsApp comum não tem etiquetas no servidor,
então isto é estado **local** do ZapFast — precisa ficar claro na UI/PRs para
ninguém esperar que apareçam no telefone.

**Exibição.** Configuração nova **"Exibir etiquetas como abas no topo da lista de
conversas"** (`labels_as_tabs: bool`, default `false`):
- **desligada** → comportamento padrão: filtro por etiqueta no menu/contexto;
- **ligada** → barra horizontal de abas fixada no topo, com rolagem quando não
  couber: primeira aba fixa **"Todas"**, depois uma aba por etiqueta mostrando a
  cor, o nome e o contador de não lidas `Nome (Qtd)` — o contador soma o `unread`
  apenas dos chats daquela etiqueta e é omitido quando for 0. Aba ativa com
  indicador visual (underline/fundo com a cor, contraste adequado).

**Onde plugar (base `c17b900`).**
- Persistência no **archive criptografado**, no padrão de `drafts`/`sticker_groups`:
  `labels (id PRIMARY KEY, name, color, created_at)` e
  `chat_labels (chat, label, PRIMARY KEY (chat, label))`, com `DELETE` no purge e em
  `Archive::clear()`; carga via `Event` próprio.
- `src/model.rs`: `Label`, `ChatLabelRelation`, `Action::*` (criar/renomear/recolorir/
  excluir etiqueta, associar/desassociar chat) e `Command`/`Event` correspondentes.
- Contadores: `chats.unread` já existe — somar por etiqueta, sem query nova pesada.

#### Parte B — Split View

**Pedido (texto 1).** Dois grupos autônomos lado a lado, cada um com a sua aba
ativa, a sua lista e a sua conversa; `Ctrl+Clique`/`Cmd+Clique` numa aba abre
aquele grupo à direita; `×` no cabeçalho do Grupo B — ou fechar a última aba dele —
volta para Grupo Único; 50/50 por padrão com divisor arrastável; foco visível no
grupo ativo; estado por grupo (`active_label_id`, `selected_chat_id`, `input_draft`,
`scroll_offset`) e `WorkspaceLayout::{Single, HorizontalSplit { left, right, split_ratio }}`.

**Pedido (texto 2).** Clique simples numa aba troca o filtro; `Ctrl+Clique` numa
segunda aba **inativa** entra em Split-View: "Coluna 1: lista de conversas da aba
ativa; Coluna 2: lista de conversas da aba clicada"; clicar simples em qualquer aba
desfaz a divisão; contêiner com 1 ou 2 painéis verticais, 50/50, divisor sutil, cada
coluna com seleção e rolagem próprias.

**Tipos pedidos (verbatim):** `Label { id, name, color_hex, created_at }`,
`ChatLabelRelation { chat_id, label_id }`,
`ViewLayoutMode::{SingleTab { primary_label_id }, SplitTab { primary_label_id, secondary_label_id }}`
(default `SingleTab { primary_label_id: None }` = "Todas"), além do
`ChatPaneState`/`WorkspaceLayout` do texto 1.

#### Decisões abertas (precisam do usuário antes de codar)

1. **O split divide o quê?** O texto 1 exige dois painéis independentes com
   "digitar e ler mensagens simultaneamente em dois chats distintos" → split do
   **workspace inteiro** (lista + conversa + compositor por painel), que é o
   trabalho grande em `App`. O texto 2 diz "Coluna 1/Coluna 2: **lista de
   conversas**" → split só da **lista**, com uma conversa compartilhada, bem menor.
   Recomendação: seguir o texto 1 (é o que "estilo VS Code" significa e o único que
   entrega a leitura simultânea), tratando a frase do texto 2 como imprecisa.
2. **Abas vs chips atuais.** A lista já tem os chips *All / Unread / Private /
   Groups*. As etiquetas como abas **substituem** os chips quando ligadas, ou
   convivem (abas em cima, chips embaixo)? Recomendação: substituem — mesmo papel
   (filtrar a lista) e evita duas fileiras de filtros.
3. **Etiquetas só locais** (sem sync de protocolo) é aceitável? Se o usuário
   esperava ver as etiquetas no telefone, não há caminho sem a Business API.

#### Fatiamento proposto (uma PR por fatia, na ordem)

- **4a. Etiquetas: dados + CRUD + filtro.** Tabelas no archive, `Label`/ações,
  diálogo de gestão (criar/renomear/cor/excluir, teto de 20), associar etiquetas a
  um chat (menu de contexto e/ou diálogo de info do chat) e filtro suspenso na
  lista. Sem abas, sem split. Testes de persistência, teto de 20 e limpeza.
- **4b. Abas no topo + contadores.** `labels_as_tabs` nas Configurações, barra com
  rolagem, aba "Todas", indicador ativo, badge de não lidas por etiqueta.
- **4c. Split View.** `WorkspaceLayout`/`ChatPaneState` (ou `ViewLayoutMode`),
  divisor com clamp, `Ctrl+Clique` na aba, `×` para fechar, foco visual, atalhos
  respeitando o painel focado. É a fatia que refatora o `App` (hoje o estado de
  conversa aberta, rascunho, scroll e filtro é único e global).

#### Notas de UI (não são bugs a corrigir)

- Como no VS Code, só o painel **focado** recebe teclado; os dois têm estado
  independente e o foco troca com um clique. Digitação literalmente simultânea nos
  dois compositores não existe (nem no VS Code).
- A barra de abas vive sobre a lista lateral (não sobre a conversa): o topo da
  janela já é o cabeçalho do chat.

#### Critérios de pronto

1. Etiquetas: criar/renomear/recolorir/excluir, teto de 20 com aviso, N:N por chat,
   filtro funcionando, tudo sobrevivendo ao reinício e limpo por `Archive::clear()`.
2. Abas (com a preferência ligada): "Todas" primeiro, uma aba por etiqueta na ordem
   de criação, cor e contador corretos (`unread` somado só dos chats da etiqueta),
   badge omitido em 0, rolagem quando não couber, aba ativa visível; com a
   preferência desligada, nada muda no layout atual.
3. Split: abre com `Ctrl+Clique`, fecha no `×` e ao fechar a última aba; cada painel
   guarda a sua aba, conversa, rascunho e scroll; divisor redimensiona com clamp e a
   largura sobrevive ao reinício; foco visível e atalhos no painel focado.
4. Testes por fatia (persistência/limpeza; contadores por etiqueta; transições
   `Single` ↔ `Split` preservando os dois estados; `split_ratio` com clamp) e
   `cargo fmt --all --check`. PRs pequenas, com passos de reprodução e declaração
   honesta do que não rodou local.

---

### Pendências de auditoria ainda abertas (runner-up, sem dono)

1. Limitar importação de stickers (tamanho/quantidade).
2. Remover mídia descriptografada ao excluir/revogar mensagem.
3. Modo privado de notificações.
4. Evitar leitura duplicada de áudio não-OGG (`src/audio.rs:478`).
5. Marcar conversa como não lida.
6. Paginar backfill de protobufs.
7. Transações por lote no history sync.
8. Mensagens favoritas.

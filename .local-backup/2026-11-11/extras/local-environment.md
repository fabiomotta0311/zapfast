# Premissas do ambiente local — ZapFast (`crmne/zapfast`) nesta máquina Windows

> **Estado atual (2026-09-28): este procedimento local foi aposentado.** CMake e
> Strawberry Perl foram removidos do Windows host. Compile `test`/`custom` pelo
> builder persistente da VPS: `scripts/build-vps-windows.sh <commit> {test|custom}`
> em `C:/Users/fabio/dev/wt-zap-local-builds`. Windows Sandbox é descartável e fica
> reservado para smoke tests da interface; não instalar nele um toolchain persistente.
> O restante deste documento é histórico, não um procedimento atual.

Documento histórico das premissas que permitiram compilar e validar mudanças do
ZapFast localmente antes da migração do build para a VPS.

Verificado em **23 e 24/09/2026**. Estado histórico: build, testes, clippy e doc
funcionando localmente pela primeira vez (antes só `cargo fmt --all --check` era confiável).

---

## Uso diário

```bash
source /c/Users/fabio/dev/zapfast-localenv.sh
```

Isso é tudo. Depois disso, dentro de qualquer worktree (`~/dev/wt-zap-*` ou o clone
principal `~/zapfast`):

```bash
cargo fmt --all --check                                                 # limpo
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo test --locked --all-targets --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --locked --all-features --no-deps
```

Essa é a lista de *Definition of done* do `AGENTS.md` do repositório. Números de
referência no `main` (24/09): `697 passaram, 0 falharam, 9 ignorados` na libraria e
`2 passaram` no binário; com `--all-features`, `698` na libraria.

Tempo típico do zero: `cargo check --locked --all-targets` ~3 min; clippy ~35 s
quando as dependências já estão compiladas; suíte de testes ~40 s de execução.

---

## O ambiente, variável por variável

| Variável | Valor | Por que |
|---|---|---|
| `PATH` (1º) | `/c/Users/fabio/dev/bin` | Contém o `make.exe` que o `openssl-src` procura (ele chama literalmente `make`; `cmd_make` só usa `gmake` em hosts BSD/Solaris). |
| `PATH` (2º) | `/c/Strawberry/c/bin` | `gcc`, `ar`, `ranlib`, `dlltool`. Tem de vir **antes** do TDM-GCC: o `dlltool` do TDM falha em `getrandom` com `cannot delete bcryptprimitives.dlls00000.o: Permission denied`. |
| `PERL5LIB` | `/c/Users/fabio/dev/perl5local` | Completa o perl do Git for Windows, que é uma build mínima (ver abaixo). **Obrigatoriamente em caminho estilo MSYS**: com `C:/...` o perl cygwin parte o valor no `:` e cai no erro original. |
| `OPENSSL_NO_VENDOR` | `1` | Faz o `openssl-sys` usar um OpenSSL já compilado em vez de construir o 3.6.3 do fonte. É o que derruba o build de ~20 min para ~3 min. |
| `OPENSSL_DIR` / `OPENSSL_LIB_DIR` / `OPENSSL_INCLUDE_DIR` | `C:/Strawberry/c`, `.../c/lib`, `.../c/include` | O OpenSSL **3.6.1** que acompanha o Strawberry (`libssl.a`, `libcrypto.a`, headers). |
| `RUSTUP_TOOLCHAIN` | `1.98.0-x86_64-pc-windows-gnu` | O toolchain MSVC falha: `link.exe` não está instalado. |
| `CARGO_TARGET_DIR` | `C:/Users/fabio/zapfast/target` | Cache **único compartilhado** entre worktrees, como o `AGENTS.md` exige ("a fresh target costs 20 GB or more"). Nunca dar `cargo clean` no diretório errado, e nunca compartilhar entre agentes que compilam ao mesmo tempo por longos períodos (contenção de lock). |

Versões nesta máquina: perl MSYS `5.42.3 x86_64-cygwin`, perl Strawberry
`5.42.3 MSWin32-x64`, OpenSSL `3.6.1` (Strawberry), `rustc 1.98.0` (GNU), egui 0.36.1.

---

## Os três artefatos fora do repositório

Nada disso é versionado. Se `~/dev` for limpo, `~/dev/bin` for apagado ou o
Strawberry for reinstalado, **reconstrua com as receitas abaixo**.

### 1. `~/dev/zapfast-localenv.sh` (o script que você dá `source`)

É o bloco de exportações da tabela acima, com os comentários do porquê. Se sumir,
recrie copiando o bloco da tabela acima; não tem dependência nenhuma além dos dois
diretórios abaixo.

### 2. `~/dev/perl5local` — shim do perl (131 arquivos, 2,2 MB)

O perl que o Git for Windows traz (`/usr/bin/perl`, cygwin) gera caminhos Unix, que é
o que o `Configure` do OpenSSL exige, mas é uma build mínima: **não tem**
`Locale::Maketext::Simple` (exigido por `Params::Check` → `IPC::Cmd`),
`ExtUtils::MakeMaker` nem `Pod::*`. O perl completo do Strawberry tem tudo isso, mas é
`MSWin32` e o `Configure` o recusa ("This perl implementation doesn't produce Unix
like paths").

Solução: copiar os módulos de **perl puro** que faltam do Strawberry para um diretório
próprio e apontá-lo por `PERL5LIB`.

```bash
mkdir -p /c/Users/fabio/dev/perl5local
cd /c/Strawberry/perl/lib
cp -r Locale ExtUtils Pod /c/Users/fabio/dev/perl5local/
# checagem (tem de imprimir "IPC::Cmd ok"):
PERL5LIB=/c/Users/fabio/dev/perl5local perl -MIPC::Cmd -e 'print "IPC::Cmd ok\n"'
```

O `Locale/Maketext/Simple.pm` do Strawberry não tem dependência externa: se o
`Locale::Maketext::Lexicon` não existir, ele cai sozinho no interpolador simples. Não
copie `Config.pm` nem módulos com parte XS: o `%Config` correto é o do perl que está
rodando.

### 3. `~/dev/bin/make.exe` — o `make` que não existe

```bash
mkdir -p /c/Users/fabio/dev/bin
cp /c/Strawberry/c/bin/gmake.exe /c/Users/fabio/dev/bin/make.exe
```

`MAKE=gmake` **não** resolve: o `openssl-src` ignora essa variável e executa `make`.

---

## Como verificar o ambiente em segundos

Em vez de esperar um build de 20 minutos para descobrir se o perl está bom, rode o
`Configure` do OpenSSL, que é onde a cadeia quebra:

```bash
export PATH="/c/Users/fabio/dev/bin:/c/Strawberry/c/bin:$PATH"
export PERL5LIB=/c/Users/fabio/dev/perl5local
perl ./Configure mingw64 no-shared no-tests          # dentro de uma cópia do fonte do OpenSSL
```

Sucesso = "OpenSSL has been successfully configured". Ele roda em segundos e testa o
perl de ponta a ponta sem compilar nada.

---

## Armadilhas conhecidas (todas já custaram tempo)

### Disco cheio se disfarça de bug de toolchain

Sintomas que **parecem** defeito de compilador e são só falta de espaço:

- `dlltool.exe: cannot delete <nome>.dlls00000.o: Permission denied`
- `error: failed to build archive at ...: Espaço insuficiente no disco. (os error 112)`

Confirme com `df -h /c` **antes** de investigar qualquer coisa. O volume aqui vive
perto do cheio (é 456 GB no total). Para liberar:

```bash
cd C:/Users/fabio/zapfast && cargo clean      # ~5 GB, reconstrui em minutos com o atalho do OpenSSL
cargo install cargo-sweep && cargo sweep --time 14   # rotação recomendada pelo AGENTS.md
export CARGO_PROFILE_DEV_DEBUG=0             # deixa o mesmo build várias vezes menor
```

### Por que a rota "vendored" do OpenSSL morre

Sem `OPENSSL_NO_VENDOR`, o `openssl-src` configura e **compila** o OpenSSL inteiro,
mas quebra no `make install_dev`: a função `sanitize_sh`/`change_drive` converte
`C:/...` em `/C/...` para montar o Makefile, e `ar`/`ranlib` nativos não leem essa
forma (`ranlib.exe: '/C/Users/.../libcrypto.a.new': No such file`). Testado: `/C/...`
falha, `C:/...` passa, `/c/...` falha. Não é corrida de paralelismo.

### `cargo` órfão segurando o lock

`Blocking waiting for file lock on build directory` sem fim significa que sobrou um
`cargo` de alguma tentativa anterior (o wrapper do shell saiu, o filho continuou vivo).

```bash
powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"name='cargo.exe'\" | Select-Object ProcessId,CreationDate | Format-Table"
powershell -NoProfile -Command "Stop-Process -Id <pid> -Force"   # mate o MAIS ANTIGO
```

### Log do cargo truncado sem erro

Com a saída redirecionada para arquivo o cargo escreve em bloco: se o processo morrer,
a mensagem de erro se perde e o log termina no meio de um `Compiling`. Rode com pty
(`pty=true` na ferramenta de terminal) ou capture o `exit code` separadamente para não
confundir "morreu" com "compilou".

### Fim de linha (CRLF) nos worktrees

`git add` em worktree recém-criado pode preparar o arquivo inteiro em CRLF (o diff vira
o arquivo todo). Normalize os arquivos tocados antes de commitar e confirme:
`grep -c $'\r' <arquivo>` = 0 e `git diff --stat` com só os seus hunks. Nunca converta
duas vezes (`\r\r\n` deixa CR solto no commit).

### Idioma do sistema e os testes

A interface segue o idioma do sistema, então três testes que comparam strings em
inglês falhavam numa máquina pt-BR (e passavam no CI, em inglês). Corrigido em
[i#173](https://github.com/crmne/zapfast/pull/173): `i18n::detect()` responde inglês
quando `cfg!(test)`. Se você vir falhas de string traduzida num head anterior a esse,
é isso: a suíte é esperada em inglês.

### O que o CI é, e o que ele não é

O GitHub Actions continua sendo o juiz final, e é o único lugar onde Linux, macOS,
Windows arm64 e os pacotes Nix rodam. O ambiente local serve para **chegar no CI com
o trabalho já verificado** — foi assim que a #173 saiu com os seis checks verdes locais
antes de qualquer rodada de CI.

---

## Onde este conhecimento vive

| Lugar | O quê |
|---|---|
| Este documento (`~/dev/zapfast-local-environment.md`) | As premissas completas, incluindo como reconstruir os artefatos. |
| `~/dev/zapfast-localenv.sh` | O bloco de ambiente, comentado, para `source`. |
| Skill `zapfast-pr-maintenance` | As lições de fluxo de PR e o resumo do ambiente (carrega nas sessões de ZapFast). |
| `~/dev/zapfast-work-queue.md` | A fila de trabalho e o resultado das PRs. |
| ai-memory `concepts/zapfast-build-local-windows-destravado.md` | A mesma história, para busca semântica entre sessões. |

**Regra para sessões futuras:** antes de inventar um workaround novo para "o ZapFast
não compila aqui", leia este documento. Todos os becos sem saída já estão listados, e
o caminho que funciona é um `source` de um arquivo.

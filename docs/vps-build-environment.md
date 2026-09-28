# Build remoto das variantes Windows

O ambiente dedicado fica em `/docker/zapfast-builder` no VPS `srv1768354`.
Ele não altera nem monta volumes dos serviços Hermes. Usa Rust 1.98.0,
`cross` 0.2.5 e a imagem cross-rs para `x86_64-pc-windows-gnu` fixada pelo
digest em `Cross.toml`. Há um único worktree de build remoto em
`/docker/zapfast-builder/worktree`, com `target` persistente dentro dele. O
script serializa a troca do commit e a compilação por `flock`; assim o
cache é reaproveitado entre commits sem compartilhar `CARGO_TARGET_DIR` entre
worktrees. O driver limita Cargo a um job e usa release otimizado sem ThinLTO
(`codegen-units=4`), mantendo `opt-level=3` e reduzindo o pico de RAM/tempo de
link em comparação ao perfil padrão. O VPS tem 2 vCPUs, 7,8 GiB de RAM e nenhum
swap; builds nele são sequenciais, mas podem rodar em paralelo ao trabalho no PC.

A versão oficial **não** é compilada. Instale-a pelo instalável Windows
publicado pelo projeto. Para uma PR ou a branch pessoal `custom`, rode na raiz
do worktree limpo:

```bash
bash scripts/build-vps-windows.sh <commit-ou-ref> test
bash scripts/build-vps-windows.sh <commit-ou-ref> custom
```

O script envia `git archive` do commit exato, sem `.git`, arquivos locais ou
alterações não commitadas. A primeira compilação do SHA gera os dois nomes a
partir do mesmo executável (a seleção do perfil é pelo nome em runtime); o
segundo pedido para o mesmo SHA reutiliza esse arquivo e evita recompilar. Os
resultados são devolvidos em `target/vps-artifacts/<commit>/zapfast_test.exe` e
`zapfast_custom.exe`. Um `flock` serializa compilações que compartilham o cache.

O alvo é Windows GNU x86_64, compatível com o toolchain GNU usado neste PC;
não substitui as verificações Windows MSVC do CI oficial. A primeira compilação
completa do ZapFast, com cache frio, terminou em 47m06s e gerou um executável
PE32+ Windows de 79.424.512 bytes. Os testes do repositório ainda não foram
executados nesse ambiente.

Ferramentas instaladas em caminhos isolados:

- Rustup: `/docker/zapfast-builder/rustup`
- Cargo, Rust e Cross: `/docker/zapfast-builder/cargo/bin` e
  `/docker/zapfast-builder/bin`
- Cargo target do worktree remoto: `/docker/zapfast-builder/worktree/target`
- Código e artefatos por SHA: `worktree/source/` e `artifacts/`
- Tarballs temporários de transferência: `incoming/`

# Build remoto das variantes Windows

O ambiente dedicado fica em `/docker/zapfast-builder` no VPS `srv1768354`.
Ele não altera nem monta volumes dos serviços Hermes. Usa Rust 1.98.0,
`cross` 0.2.5 e a imagem cross-rs para `x86_64-pc-windows-gnu` fixada pelo
digest em `Cross.toml`. O Cargo target é persistente nesse diretório para
reaproveitar dependências entre commits. O driver limita Cargo a um job e usa
release otimizado sem ThinLTO (`codegen-units=4`): mantém `opt-level=3` e reduz
o pico de RAM/tempo de link em comparação ao perfil padrão. O VPS tem 2 vCPUs,
7,8 GiB de RAM e nenhum swap; builds devem ficar sequenciais lá, mas podem rodar
em paralelo aos builds/trabalho no PC.

A versão oficial **não** é compilada. Instale-a pelo instalável Windows
publicado pelo projeto. Para uma PR ou a branch pessoal `custom`, rode na raiz
do worktree limpo:

```bash
bash scripts/build-vps-windows.sh <commit-ou-ref> test
bash scripts/build-vps-windows.sh <commit-ou-ref> custom
```

O script envia `git archive` do commit exato, sem `.git`, arquivos locais ou
alterações não commitadas. O resultado volta para
`target/vps-artifacts/<commit>/zapfast_test.exe` ou
`zapfast_custom.exe`; o nome final seleciona o perfil isolado em runtime. Um
`flock` serializa compilações que compartilham o cache.

O alvo é Windows GNU x86_64, compatível com o toolchain GNU usado neste PC;
não substitui as verificações Windows MSVC do CI oficial. O smoke test de
ambiente compilou um executável PE32+ mínimo no VPS. A compilação do ZapFast
completo e os testes do repositório precisam ser confirmados antes de tratar a
pipeline como pronta para uso diário.

Ferramentas instaladas em caminhos isolados:

- Rustup: `/docker/zapfast-builder/rustup`
- Cargo, Rust e Cross: `/docker/zapfast-builder/cargo/bin` e
  `/docker/zapfast-builder/bin`
- Cache de compilação: `/docker/zapfast-builder/target`
- Workspaces e artefatos por SHA: `workspaces/` e `artifacts/`

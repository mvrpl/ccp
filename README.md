[![build and release](https://github.com/mvrpl/ccp/actions/workflows/builds.yaml/badge.svg)](https://github.com/mvrpl/ccp/actions/workflows/builds.yaml)
[![dependency status](https://deps.rs/repo/github/mvrpl/ccp/status.svg)](https://deps.rs/repo/github/mvrpl/ccp)
[![MIT licensed][gnu-badge]][gnu-url]

[gnu-badge]: https://img.shields.io/badge/license-GNU3.0-blue.svg
[gnu-url]: https://github.com/mvrpl/ccp/blob/main/LICENSE

# Chat Copy Protocol

Copy files to chat systems like Telegram, WhatsApp.

## Funcionalidades

- Copie arquivos do file system nativo (Windows, Unix, Linux)
- Envie bytes ou textos do standard input (stdin)
- Multi-OS (Windows, Mac e Linux)


## Como usar

#### Windows x86|ARM 64 bits

```powershell
scoop bucket add mvrpl https://github.com/mvrpl/windows-apps
scoop install mvrpl/ccp
scoop install mvrpl/ssclient
New-Item -Path '~\.ccp' -Type Directory
ssclient create ~\.ccp\secrets.json
ssclient -s ~\.ccp\secrets.json set telegram_token
ssclient -s ~\.ccp\secrets.json set graph_bearer_token
$env.CCP_VAULT_PASS='<SENHA secrets.json>'
ccp cp <caminho_arquivo> [telegram|whatsapp]://...
```

#### Linux x86|ARM 64 bits - MacOS x86|ARM 64 bits

```bash
brew tap mvrpl/unix-apps https://github.com/mvrpl/unix-apps
brew install mvrpl/unix-apps/ccp
cargo install ssclient
mkdir ~/.ccp
ssclient create ~/.ccp/secrets.json
ssclient -s ~/.ccp/secrets.json set telegram_token
ssclient -s ~/.ccp/secrets.json set graph_bearer_token
export CCP_VAULT_PASS='<SENHA secrets.json>'
ccp cp <caminho_arquivo> [telegram|whatsapp]://...
```
## Autor

- [@mvrpl](https://www.github.com/mvrpl)


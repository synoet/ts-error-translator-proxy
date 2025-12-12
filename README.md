# ts-error-translator-proxy

A lightweight LSP proxy that translates cryptic TypeScript error messages into human-readable explanations.

## Why?

TypeScript errors are notoriously unhelpful. Messages like `Type 'X' is not assignable to type 'Y'` tell you *what* failed but not *why* or *how* to fix it.

Tools like [ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim) solve this for Neovim users, but if you use Helix, Zed, or any other LSP-compatible editor, you're out of luck.

This proxy sits between your editor and any TypeScript LSP, intercepting diagnostics and appending (or replacing) error messages with plain-English explanations. No editor plugins required.

## Features

- **Editor-agnostic** - Works with any editor that supports LSP
- **LSP-agnostic** - Works with vtsls, typescript-language-server, or any TypeScript LSP
- **Zero editor configuration** - Just wrap your LSP command
- **70+ error translations** - Covers the most common TypeScript errors
- **Two modes** - Append explanations or replace the original message entirely
- **Fast** - Async Rust, minimal overhead

## Installation

### From source

```bash
git clone https://github.com/yourusername/ts-error-translator-proxy
cd ts-error-translator-proxy
cargo build --release
cp target/release/ts-error-translator-proxy ~/.local/bin/
```

### Cargo

```bash
cargo install ts-error-translator-proxy
```

## Usage

```bash
ts-error-translator-proxy [OPTIONS] [LSP_COMMAND] [LSP_ARGS...]
```

### Options

| Option | Description |
|--------|-------------|
| `--replace` | Replace original error message instead of appending the translation |
| `--help` | Show help |

### Examples

```bash
# Default: wraps vtsls with appended translations
ts-error-translator-proxy

# Wrap typescript-language-server instead
ts-error-translator-proxy typescript-language-server --stdio

# Replace mode - only show the translation
ts-error-translator-proxy --replace vtsls --stdio
```

## Editor Configuration

### Helix

Edit `~/.config/helix/languages.toml`:

```toml
[language-server.vtsls-translated]
command = "ts-error-translator-proxy"
args = ["vtsls", "--stdio"]

[[language]]
name = "typescript"
language-servers = ["vtsls-translated"]

[[language]]
name = "tsx"
language-servers = ["vtsls-translated"]

[[language]]
name = "javascript"
language-servers = ["vtsls-translated"]

[[language]]
name = "jsx"
language-servers = ["vtsls-translated"]
```

Or with `typescript-language-server`:

```toml
[language-server.tsserver-translated]
command = "ts-error-translator-proxy"
args = ["typescript-language-server", "--stdio"]

[[language]]
name = "typescript"
language-servers = ["tsserver-translated"]
```

### Zed

Edit `~/.config/zed/settings.json`:

```json
{
  "lsp": {
    "vtsls": {
      "binary": {
        "path": "ts-error-translator-proxy",
        "arguments": ["vtsls", "--stdio"]
      }
    }
  }
}
```

Or to use replace mode:

```json
{
  "lsp": {
    "vtsls": {
      "binary": {
        "path": "ts-error-translator-proxy",
        "arguments": ["--replace", "vtsls", "--stdio"]
      }
    }
  }
}
```

### Neovim (nvim-lspconfig)

While Neovim users can use [ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim), this proxy works too:

```lua
local lspconfig = require('lspconfig')

lspconfig.vtsls.setup({
  cmd = { "ts-error-translator-proxy", "vtsls", "--stdio" },
})
```

Or with typescript-language-server:

```lua
lspconfig.ts_ls.setup({
  cmd = { "ts-error-translator-proxy", "typescript-language-server", "--stdio" },
})
```

### VS Code

Create a wrapper script (`~/.local/bin/vtsls-translated`):

```bash
#!/bin/bash
exec ts-error-translator-proxy vtsls --stdio
```

Then in `.vscode/settings.json`:

```json
{
  "typescript.tsserver.pluginPaths": [],
  "vtsls.autoUseWorkspaceTsdk": true,
  "vtsls.serverPath": "~/.local/bin/vtsls-translated"
}
```

Note: VS Code has native TypeScript support, so this setup requires disabling the built-in TypeScript extension and using vtsls via a generic LSP extension.

### Sublime Text (LSP package)

Edit `LSP.sublime-settings`:

```json
{
  "clients": {
    "typescript": {
      "enabled": true,
      "command": ["ts-error-translator-proxy", "typescript-language-server", "--stdio"],
      "selector": "source.ts, source.tsx, source.js, source.jsx"
    }
  }
}
```

### Kakoune (kak-lsp)

Edit `~/.config/kak-lsp/kak-lsp.toml`:

```toml
[language.typescript]
filetypes = ["typescript", "javascript"]
roots = ["package.json", "tsconfig.json"]
command = "ts-error-translator-proxy"
args = ["typescript-language-server", "--stdio"]
```

## How It Works

```
┌────────┐     stdin/stdout      ┌─────────────────────┐     stdin/stdout     ┌─────┐
│ Editor │ ◄──────────────────► │ ts-error-translator │ ◄──────────────────► │ LSP │
└────────┘                       │       -proxy        │                      └─────┘
                                 └─────────────────────┘
                                           │
                                           ▼
                                  Intercepts diagnostics,
                                  appends translations
```

The proxy:

1. Spawns the actual LSP as a child process
2. Forwards all messages from editor → LSP unchanged
3. Intercepts `textDocument/publishDiagnostics` responses from LSP → editor
4. Looks up error codes and appends human-readable translations
5. Forwards all other LSP → editor messages unchanged

## Supported Errors

Currently translates 70+ TypeScript errors including:

- **Type errors** (2322, 2339, 2345, etc.) - Assignment and property access issues
- **Import/export errors** (2304, 2305, 2307, etc.) - Module resolution problems
- **Syntax errors** (1002, 1005, 1109, etc.) - Parsing and structure issues
- **Strict mode errors** (7006, 7053, etc.) - Implicit any and index signature problems

## Acknowledgments

Error translations inspired by [Matt Pocock's ts-error-translator](https://github.com/mattpocock/ts-error-translator) VSCode extension and [dmmulroy's ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim).

## License

MIT

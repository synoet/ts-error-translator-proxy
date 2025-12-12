# ts-error-translator-proxy

An LSP proxy that translates TypeScript errors into human-readable explanations.

Based on [Matt Pocock's ts-error-translator](https://github.com/mattpocock/ts-error-translator). Inspired by [ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim).

```
Property 'foo' does not exist on type 'Bar'.

● This property doesn't exist on the type - check for typos or add the property to the type definition.
```

## Why a proxy?

[ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim) works great for Neovim. If you use Helix, Zed, or another editor, you need a different approach.

This proxy wraps any TypeScript LSP and intercepts diagnostics. No editor plugins required.

## Installation

```bash
cargo install ts-error-translator-proxy
```

Or from source:

```bash
git clone https://github.com/yourusername/ts-error-translator-proxy
cd ts-error-translator-proxy
cargo build --release
```

## Usage

```bash
ts-error-translator-proxy [OPTIONS] [LSP_COMMAND] [LSP_ARGS...]
```

| Option | Description |
|--------|-------------|
| `--replace` | Replace original error instead of appending translation |

```bash
# Wrap vtsls (default)
ts-error-translator-proxy

# Wrap typescript-language-server
ts-error-translator-proxy typescript-language-server --stdio

# Replace mode
ts-error-translator-proxy --replace vtsls --stdio
```

## Editor Configuration

### Helix

`~/.config/helix/languages.toml`:

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
```

### Zed

`~/.config/zed/settings.json`:

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

### Neovim

Neovim users should use [ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim) instead.

## License

MIT

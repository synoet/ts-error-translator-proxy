# ts-error-translator-proxy

An LSP proxy that translates TypeScript errors into human-readable explanations. Works with any editor.

Based on [Matt Pocock's ts-error-translator](https://github.com/mattpocock/ts-error-translator). Inspired by [ts-error-translator.nvim](https://github.com/dmmulroy/ts-error-translator.nvim).

*Zed*
<img width="1502" height="526" alt="CleanShot 2025-12-11 at 22 43 31@2x" src="https://github.com/user-attachments/assets/7f6ea8fa-660f-44ae-84d8-a25368a8d17a" />
*Helix*
<img width="1976" height="394" alt="CleanShot 2025-12-11 at 22 41 37@2x" src="https://github.com/user-attachments/assets/dc95c1a2-85d7-4463-a18b-8d61141f5b58" />

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
| `--append` | Append translation to original message instead of replacing |

```bash
# Wrap vtsls (default - replaces original error)
ts-error-translator-proxy

# Wrap typescript-language-server
ts-error-translator-proxy typescript-language-server --stdio

# Append mode (keeps original error + adds translation)
ts-error-translator-proxy --append vtsls --stdio
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

### Other Editors

This should work with any editor that supports LSP. PRs with examples welcome.

## License

MIT

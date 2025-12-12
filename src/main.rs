mod errors;
mod jsonrpc;
mod proxy;
mod translator;

use std::process::Stdio;
use tokio::process::Command;

#[derive(Clone, Copy)]
pub enum TranslationMode {
    Append,
    Replace,
}

fn print_usage() {
    eprintln!("Usage: ts-error-translator-proxy [OPTIONS] [LSP_COMMAND] [LSP_ARGS...]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --replace    Replace original message instead of appending");
    eprintln!("  --help       Show this help");
    eprintln!();
    eprintln!("Default LSP: vtsls --stdio");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut mode = TranslationMode::Append;
    let mut lsp_args: Vec<String> = Vec::new();

    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            "--replace" => mode = TranslationMode::Replace,
            _ => lsp_args.push(arg.clone()),
        }
    }

    let (cmd, cmd_args) = if lsp_args.is_empty() {
        ("vtsls".to_string(), vec!["--stdio".to_string()])
    } else {
        (lsp_args[0].clone(), lsp_args[1..].to_vec())
    };

    let mut child = Command::new(&cmd)
        .args(&cmd_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let lsp_stdin = child.stdin.take().expect("Failed to open LSP stdin");
    let lsp_stdout = child.stdout.take().expect("Failed to open LSP stdout");

    let editor_stdin = tokio::io::stdin();
    let editor_stdout = tokio::io::stdout();

    proxy::run_proxy(editor_stdin, editor_stdout, lsp_stdout, lsp_stdin, mode).await?;

    child.wait().await?;
    Ok(())
}

use crate::jsonrpc::{read_message, write_message};
use crate::translator::translate_message;
use crate::TranslationMode;
use serde_json::Value;
use std::borrow::Cow;
use tokio::io::{AsyncRead, AsyncWrite, BufReader};

pub async fn run_proxy<R1, W1, R2, W2>(
    editor_reader: R1,
    mut editor_writer: W1,
    lsp_reader: R2,
    mut lsp_writer: W2,
    mode: TranslationMode,
) -> std::io::Result<()>
where
    R1: AsyncRead + Unpin + Send + 'static,
    W1: AsyncWrite + Unpin + Send + 'static,
    R2: AsyncRead + Unpin + Send + 'static,
    W2: AsyncWrite + Unpin + Send + 'static,
{
    let mut editor_reader = BufReader::new(editor_reader);
    let mut lsp_reader = BufReader::new(lsp_reader);

    let editor_to_lsp = async {
        loop {
            let Some(msg) = read_message(&mut editor_reader).await? else {
                break;
            };
            write_message(&mut lsp_writer, &msg).await?;
        }
        Ok::<_, std::io::Error>(())
    };

    let lsp_to_editor = async {
        loop {
            let Some(msg) = read_message(&mut lsp_reader).await? else {
                break;
            };
            let transformed = transform_if_diagnostics(&msg, mode);
            write_message(&mut editor_writer, transformed.as_ref()).await?;
        }
        Ok::<_, std::io::Error>(())
    };

    tokio::select! {
        r = editor_to_lsp => r?,
        r = lsp_to_editor => r?,
    }

    Ok(())
}

fn is_publish_diagnostics(json: &Value) -> bool {
    json.get("method").and_then(Value::as_str) == Some("textDocument/publishDiagnostics")
}

fn transform_if_diagnostics(msg: &[u8], mode: TranslationMode) -> Cow<'_, [u8]> {
    let Ok(mut json) = serde_json::from_slice::<Value>(msg) else {
        return Cow::Borrowed(msg);
    };

    if !is_publish_diagnostics(&json) {
        return Cow::Borrowed(msg);
    }

    let Some(diagnostics) = json
        .get_mut("params")
        .and_then(|p| p.get_mut("diagnostics"))
        .and_then(Value::as_array_mut)
    else {
        return Cow::Borrowed(msg);
    };

    for diagnostic in diagnostics {
        transform_diagnostic(diagnostic, mode);
    }

    Cow::Owned(serde_json::to_vec(&json).unwrap_or_else(|_| msg.to_vec()))
}

fn transform_diagnostic(diagnostic: &mut Value, mode: TranslationMode) {
    let code = diagnostic.get("code").and_then(Value::as_i64);

    let Some(message) = diagnostic.get_mut("message") else {
        return;
    };
    let Some(msg_str) = message.as_str() else {
        return;
    };

    let translated = translate_message(msg_str, code, mode);
    if let Cow::Owned(s) = translated {
        *message = Value::String(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn diagnostic_msg(code: u32, message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "method": "textDocument/publishDiagnostics",
            "params": {
                "uri": "file:///test.ts",
                "diagnostics": [{"message": message, "code": code}]
            }
        })
    }

    fn extract_message(output: &[u8]) -> String {
        let v: Value = serde_json::from_slice(output).unwrap();
        v["params"]["diagnostics"][0]["message"]
            .as_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_translates_known_error() {
        let input = serde_json::to_vec(&diagnostic_msg(2339, "Property 'x' does not exist")).unwrap();
        let output = transform_if_diagnostics(&input, TranslationMode::Append);
        let msg = extract_message(&output);

        assert!(msg.contains("Property 'x' does not exist"));
        assert!(msg.contains("●"));
    }

    #[test]
    fn test_unknown_error_passthrough() {
        let input = serde_json::to_vec(&diagnostic_msg(99999, "Unknown error")).unwrap();
        let output = transform_if_diagnostics(&input, TranslationMode::Append);

        assert_eq!(extract_message(&output), "Unknown error");
    }

    #[test]
    fn test_non_diagnostic_passthrough() {
        let input = br#"{"jsonrpc":"2.0","method":"initialize","params":{}}"#;
        let output = transform_if_diagnostics(input, TranslationMode::Append);

        assert_eq!(input.as_slice(), output.as_ref());
    }
}

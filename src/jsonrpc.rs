use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

pub async fn read_message<R: tokio::io::AsyncRead + Unpin>(
    reader: &mut BufReader<R>,
) -> std::io::Result<Option<Vec<u8>>> {
    let mut content_length: Option<usize> = None;
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            return Ok(None);
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        if let Some(len_str) = trimmed.strip_prefix("Content-Length: ") {
            content_length = Some(
                len_str
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            );
        }
    }

    let length = content_length.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing Content-Length")
    })?;

    let mut body = vec![0u8; length];
    reader.read_exact(&mut body).await?;

    Ok(Some(body))
}

pub async fn write_message<W: tokio::io::AsyncWrite + Unpin>(
    writer: &mut W,
    body: &[u8],
) -> std::io::Result<()> {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    writer.write_all(header.as_bytes()).await?;
    writer.write_all(body).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_roundtrip() {
        let original = b"{\"jsonrpc\":\"2.0\",\"id\":1}";

        let mut wire = Vec::new();
        write_message(&mut wire, original).await.unwrap();

        let mut reader = BufReader::new(Cursor::new(wire));
        let decoded = read_message(&mut reader).await.unwrap().unwrap();

        assert_eq!(decoded, original);
    }

    #[tokio::test]
    async fn test_eof_returns_none() {
        let mut reader = BufReader::new(Cursor::new(Vec::new()));
        assert!(read_message(&mut reader).await.unwrap().is_none());
    }
}

use crate::errors::ERRORS;
use crate::TranslationMode;
use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

static TS_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bts(\d+)\b").unwrap());

pub fn extract_error_code(message: &str) -> Option<u32> {
    TS_CODE_REGEX
        .captures(message)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

pub fn lookup_translation(code: u32) -> Option<&'static str> {
    ERRORS.get(&code).map(|info| info.translation)
}

pub fn translate_message<'a>(original: &'a str, code: Option<i64>, mode: TranslationMode) -> Cow<'a, str> {
    let error_code = code
        .map(|c| c as u32)
        .or_else(|| extract_error_code(original));

    let Some(error_code) = error_code else {
        return Cow::Borrowed(original);
    };

    let Some(translation) = lookup_translation(error_code) else {
        return Cow::Borrowed(original);
    };

    Cow::Owned(match mode {
        TranslationMode::Append => format!("{}\n\n● {}", original, translation),
        TranslationMode::Replace => format!("● {}", translation),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_append_preserves_original() {
        let msg = "Property 'foo' does not exist on type 'Bar'.";
        let result = translate_message(msg, Some(2339), TranslationMode::Append);
        assert!(result.contains(msg));
        assert!(result.contains("●"));
    }

    #[test]
    fn test_translate_replace_removes_original() {
        let msg = "Property 'foo' does not exist on type 'Bar'.";
        let result = translate_message(msg, Some(2339), TranslationMode::Replace);
        assert!(!result.contains(msg));
        assert!(result.contains("●"));
    }

    #[test]
    fn test_unknown_code_unchanged() {
        let msg = "Some unknown error";
        let result = translate_message(msg, Some(99999), TranslationMode::Append);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_extracts_code_from_message_text() {
        let msg = "error TS2339: Property 'x' does not exist";
        let result = translate_message(msg, None, TranslationMode::Append);
        assert!(result.contains("●"));
    }
}

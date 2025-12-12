use crate::errors::{extract_params, substitute_params, ERRORS};
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

pub fn translate_message<'a>(original: &'a str, code: Option<i64>, mode: TranslationMode) -> Cow<'a, str> {
    let error_code = code
        .map(|c| c as u32)
        .or_else(|| extract_error_code(original));

    let Some(error_code) = error_code else {
        return Cow::Borrowed(original);
    };

    let Some(info) = ERRORS.get(&error_code) else {
        return Cow::Borrowed(original);
    };

    // Extract parameters from the original message and substitute into improved message
    let translation = match extract_params(&info.pattern, original) {
        Some(params) => substitute_params(info.message, &params),
        None => info.message.to_string(),
    };

    Cow::Owned(match mode {
        TranslationMode::Append => format!("{}  ● {}", original, translation),
        TranslationMode::Replace => format!("● {}", translation),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_with_params() {
        let msg = "Property 'foo' does not exist on type 'Bar'.";
        let result = translate_message(msg, Some(2339), TranslationMode::Append);
        assert!(result.contains(msg));
        assert!(result.contains("You're trying to access 'foo' on an object that doesn't contain it."));
    }

    #[test]
    fn test_translate_replace_with_params() {
        let msg = "Property 'foo' does not exist on type 'Bar'.";
        let result = translate_message(msg, Some(2339), TranslationMode::Replace);
        assert!(!result.contains(msg));
        assert!(result.contains("You're trying to access 'foo' on an object that doesn't contain it."));
    }

    #[test]
    fn test_translate_type_mismatch() {
        let msg = "Type 'string' is not assignable to type 'number'.";
        let result = translate_message(msg, Some(2322), TranslationMode::Append);
        assert!(result.contains("I was expecting a type matching 'number' but instead you passed 'string'."));
    }

    #[test]
    fn test_translate_argument_count() {
        let msg = "Expected 2 arguments, but got 3.";
        let result = translate_message(msg, Some(2554), TranslationMode::Append);
        assert!(result.contains("This function needs 2 argument(s), but you're passing 3."));
    }

    #[test]
    fn test_unknown_code_unchanged() {
        let msg = "Some unknown error";
        let result = translate_message(msg, Some(99999), TranslationMode::Append);
        assert_eq!(result, msg);
    }

    #[test]
    fn test_extracts_code_from_message_text() {
        let msg = "error TS2339: Property 'x' does not exist on type 'Y'.";
        let result = translate_message(msg, None, TranslationMode::Append);
        assert!(result.contains("●"));
    }

    #[test]
    fn test_cannot_find_name() {
        let msg = "Cannot find name 'myVariable'.";
        let result = translate_message(msg, Some(2304), TranslationMode::Append);
        assert!(result.contains("I can't find 'myVariable' - it might not be imported or defined."));
    }

    #[test]
    fn test_module_no_export() {
        let msg = "Module './utils' has no exported member 'helper'.";
        let result = translate_message(msg, Some(2305), TranslationMode::Append);
        assert!(result.contains("'helper' is not exported from './utils'."));
    }
}

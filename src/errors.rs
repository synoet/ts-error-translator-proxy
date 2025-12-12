use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct ErrorInfo {
    /// Regex pattern to match the original error and extract parameters
    pub pattern: Regex,
    /// Human-readable message with {0}, {1}, etc. placeholders
    pub message: &'static str,
}

impl ErrorInfo {
    fn new(pattern: &str, message: &'static str) -> Self {
        Self {
            pattern: pattern_to_regex(pattern),
            message,
        }
    }
}

/// Convert a pattern like "Property '{0}' does not exist on type '{1}'."
/// into a regex that captures the placeholder values
fn pattern_to_regex(pattern: &str) -> Regex {
    let mut regex_str = String::with_capacity(pattern.len() * 2);
    regex_str.push('^');

    let mut last_end = 0;
    let mut i = 0;

    let bytes = pattern.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'{' {
            // Escape everything before this placeholder
            regex_str.push_str(&regex::escape(&pattern[last_end..i]));

            // Find the closing brace
            while i < bytes.len() && bytes[i] != b'}' {
                i += 1;
            }
            // Add a capture group for this placeholder
            regex_str.push_str("(.+?)");
            last_end = i + 1;
        }
        i += 1;
    }

    // Escape any remaining literal text
    if last_end < pattern.len() {
        regex_str.push_str(&regex::escape(&pattern[last_end..]));
    }

    regex_str.push('$');
    Regex::new(&regex_str).unwrap()
}

/// Extract parameters from an error message using the pattern
pub fn extract_params<'a>(pattern: &Regex, message: &'a str) -> Option<Vec<&'a str>> {
    pattern.captures(message).map(|caps| {
        caps.iter()
            .skip(1)
            .filter_map(|m| m.map(|m| m.as_str()))
            .collect()
    })
}

/// Substitute parameters into a message template
pub fn substitute_params(template: &str, params: &[&str]) -> String {
    let mut result = template.to_string();
    for (i, param) in params.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, param);
    }
    result
}

// Error patterns and messages sourced from:
// https://github.com/mattpocock/ts-error-translator
// https://github.com/dmmulroy/ts-error-translator.nvim
pub static ERRORS: LazyLock<HashMap<u32, ErrorInfo>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // 1000-series: Syntax and parsing errors
    m.insert(1002, ErrorInfo::new(
        "Unterminated string literal.",
        "You've started a string but haven't ended it.",
    ));
    m.insert(1003, ErrorInfo::new(
        "Identifier expected.",
        "I was expecting a name but none was provided.",
    ));
    m.insert(1005, ErrorInfo::new(
        "'{0}' expected.",
        "'{0}' is expected here.",
    ));
    m.insert(1006, ErrorInfo::new(
        "A file cannot end inside a template literal.",
        "A file cannot end inside a template literal.",
    ));
    m.insert(1009, ErrorInfo::new(
        "Trailing comma not allowed.",
        "You've added a trailing comma when you're not supposed to.",
    ));
    m.insert(1014, ErrorInfo::new(
        "A rest parameter must be last in a parameter list.",
        "A parameter that starts with `...` must be the last one in the list.",
    ));
    m.insert(1015, ErrorInfo::new(
        "Parameter cannot have question mark and initializer.",
        "A parameter cannot use both a question mark and a default value - choose one or the other.",
    ));
    m.insert(1091, ErrorInfo::new(
        "Only a single variable declaration is allowed in a 'for...in' statement.",
        "You can only create a single variable in a 'for...in' statement.",
    ));
    m.insert(1109, ErrorInfo::new(
        "Expression expected.",
        "I was expecting some code that gives me a value.",
    ));
    m.insert(1117, ErrorInfo::new(
        "An object literal cannot have multiple properties with the same name.",
        "You can't add the same property multiple times to an object.",
    ));
    m.insert(1155, ErrorInfo::new(
        "A 'const' assertions can only be applied to references to enum members, or string, number, boolean, array, or object literals.",
        "A `const` must be given a value when you declare it.",
    ));
    m.insert(1163, ErrorInfo::new(
        "A computed property name must be of type '{0}'.",
        "A computed property name must be of type '{0}'.",
    ));
    m.insert(1208, ErrorInfo::new(
        "'{0}' cannot be compiled under '--isolatedModules' because it is considered a global script file. Add an import, export, or an empty 'export {}' statement to make it a module.",
        "'{0}' is being treated as a script, not a module. Add an import, export, or an empty 'export {{}}' statement to make it a module.",
    ));
    m.insert(1240, ErrorInfo::new(
        "Unable to resolve signature of class decorator when called as an expression.",
        "I can't resolve the signature of this class decorator.",
    ));
    m.insert(1254, ErrorInfo::new(
        "A 'const' assertion can only be applied to a string, number, boolean, array, or object literal.",
        "A 'const' assertion can only be applied to a string, number, boolean, array, or object literal.",
    ));
    m.insert(1268, ErrorInfo::new(
        "'await' expressions are only allowed at the top level of a file when that file is a module, but this file has no imports or exports. Consider adding an empty 'export {}' to make this file a module.",
        "'await' expressions are only allowed at the top level of a file when that file is a module. Add an empty 'export {{}}' to make this file a module.",
    ));
    m.insert(1313, ErrorInfo::new(
        "A class may only extend another class.",
        "A class may only extend another class.",
    ));
    m.insert(1434, ErrorInfo::new(
        "Top-level 'await' expressions are only allowed when the 'module' option is set to 'es2022', 'esnext', 'system', 'node16', 'nodenext', or 'preserve', and the 'target' option is set to 'es2017' or higher.",
        "You need to enable top-level await in your tsconfig.",
    ));

    // 2300-series: Name resolution and declarations
    m.insert(2304, ErrorInfo::new(
        "Cannot find name '{0}'.",
        "I can't find '{0}' - it might not be imported or defined.",
    ));
    m.insert(2305, ErrorInfo::new(
        "Module '{0}' has no exported member '{1}'.",
        "'{1}' is not exported from '{0}'.",
    ));
    m.insert(2307, ErrorInfo::new(
        "Cannot find module '{0}' or its corresponding type declarations.",
        "This could be one of two things - either '{0}' doesn't exist on your file system, or I can't find any type declarations for it.",
    ));
    m.insert(2312, ErrorInfo::new(
        "An interface can only extend an object type or intersection of object types with statically known members.",
        "An interface can only extend an object type or another interface.",
    ));
    m.insert(2314, ErrorInfo::new(
        "Generic type '{0}' requires {1} type argument(s).",
        "'{0}' requires {1} type argument(s) - you need to pass them via a generic.",
    ));
    m.insert(2322, ErrorInfo::new(
        "Type '{0}' is not assignable to type '{1}'.",
        "I was expecting a type matching '{1}' but instead you passed '{0}'.",
    ));
    m.insert(2324, ErrorInfo::new(
        "Property '{0}' is missing in type '{1}'.",
        "Property '{0}' is missing in type '{1}'.",
    ));
    m.insert(2326, ErrorInfo::new(
        "Types of property '{0}' are incompatible.",
        "Types of property '{0}' are incompatible.",
    ));
    m.insert(2327, ErrorInfo::new(
        "Index signature is missing in type '{0}'.",
        "Index signature is missing in type '{0}'.",
    ));
    m.insert(2339, ErrorInfo::new(
        "Property '{0}' does not exist on type '{1}'.",
        "You're trying to access '{0}' on an object that doesn't contain it.",
    ));
    m.insert(2344, ErrorInfo::new(
        "Type '{0}' does not satisfy the constraint '{1}'.",
        "Type '{0}' doesn't satisfy the constraint '{1}'.",
    ));
    m.insert(2345, ErrorInfo::new(
        "Argument of type '{0}' is not assignable to parameter of type '{1}'.",
        "I was expecting '{1}' but you passed '{0}'.",
    ));
    m.insert(2349, ErrorInfo::new(
        "This expression is not callable.",
        "You're trying to call something that isn't a function.",
    ));
    m.insert(2352, ErrorInfo::new(
        "Conversion of type '{0}' to type '{1}' may be a mistake because neither type sufficiently overlaps with the other.",
        "Converting '{0}' to '{1}' may be a mistake - these types don't overlap.",
    ));
    m.insert(2353, ErrorInfo::new(
        "Object literal may only specify known properties, and '{0}' does not exist in type '{1}'.",
        "You can't pass property '{0}' to type '{1}'.",
    ));
    m.insert(2355, ErrorInfo::new(
        "A function whose declared type is neither 'void' nor 'any' must return a value.",
        "This function says it returns something, but it doesn't return anything.",
    ));
    m.insert(2365, ErrorInfo::new(
        "Operator '{0}' cannot be applied to types '{1}' and '{2}'.",
        "Operator '{0}' cannot be applied to types '{1}' and '{2}'.",
    ));
    m.insert(2393, ErrorInfo::new(
        "Duplicate function implementation.",
        "You've got a duplicate function implementation.",
    ));
    m.insert(2414, ErrorInfo::new(
        "Class name cannot be '{0}'.",
        "Class name cannot be '{0}'.",
    ));
    m.insert(2451, ErrorInfo::new(
        "Cannot redeclare block-scoped variable '{0}'.",
        "'{0}' has already been declared - you can't declare it again.",
    ));
    m.insert(2488, ErrorInfo::new(
        "Type '{0}' must have a '[Symbol.iterator]()' method that returns an iterator.",
        "Type '{0}' must have a '[Symbol.iterator]()' method to use for-of.",
    ));
    m.insert(2551, ErrorInfo::new(
        "Property '{0}' does not exist on type '{1}'. Did you mean '{2}'?",
        "You're trying to access '{0}' on an object that doesn't contain it. Did you mean '{2}'?",
    ));
    m.insert(2552, ErrorInfo::new(
        "Cannot find name '{0}'. Did you mean '{1}'?",
        "Cannot find name '{0}'. Did you mean '{1}'?",
    ));
    m.insert(2554, ErrorInfo::new(
        "Expected {0} arguments, but got {1}.",
        "This function needs {0} argument(s), but you're passing {1}.",
    ));
    m.insert(2556, ErrorInfo::new(
        "A spread argument must either have a tuple type or be passed to a rest parameter.",
        "A spread argument must be from a tuple or passed to a rest parameter.",
    ));
    m.insert(2571, ErrorInfo::new(
        "Object is of type 'unknown'.",
        "I don't know what type this object is, so I've defaulted it to 'unknown'.",
    ));
    m.insert(2590, ErrorInfo::new(
        "Expression produces a union type that is too complex to represent.",
        "This expression produces a type that's too complex for me to represent.",
    ));
    m.insert(2604, ErrorInfo::new(
        "JSX element type '{0}' does not have any construct or call signatures.",
        "JSX element type '{0}' doesn't have any construct or call signatures.",
    ));
    m.insert(2614, ErrorInfo::new(
        "Module '{0}' has no default export.",
        "Module '{0}' has no default export.",
    ));
    m.insert(2686, ErrorInfo::new(
        "'{0}' refers to a UMD global, but the current file is a module. Consider adding an import instead.",
        "'{0}' refers to a UMD global, but this file is a module. Consider adding an import instead.",
    ));
    m.insert(2722, ErrorInfo::new(
        "Cannot invoke an object which is possibly 'undefined'.",
        "This value might be undefined - check that it exists before using it.",
    ));
    m.insert(2739, ErrorInfo::new(
        "Type '{0}' is missing the following properties from type '{1}': {2}",
        "'{0}' is missing some required properties from type '{1}': {2}",
    ));
    m.insert(2741, ErrorInfo::new(
        "Property '{0}' is missing in type '{1}' but required in type '{2}'.",
        "You haven't passed all the required properties to '{2}' - '{1}' is missing the '{0}' property.",
    ));
    m.insert(2749, ErrorInfo::new(
        "'{0}' refers to a value, but is being used as a type here. Did you mean 'typeof {0}'?",
        "'{0}' is a value, not a type. Did you mean 'typeof {0}'?",
    ));
    m.insert(2761, ErrorInfo::new(
        "Type import '{0}' cannot be used as a value because it was exported using 'export type'.",
        "'{0}' is a type-only import and can't be used as a value.",
    ));
    m.insert(2775, ErrorInfo::new(
        "Assertions require every name in the call target to be declared with an explicit type annotation.",
        "Assertions require every name to be declared with an explicit type annotation.",
    ));
    m.insert(2783, ErrorInfo::new(
        "'{0}' is specified more than once, so this usage will be overwritten.",
        "'{0}' is specified more than once - the later value will overwrite earlier ones.",
    ));

    // 5000-series
    m.insert(5075, ErrorInfo::new(
        "'{0}' is a type and cannot be imported in JavaScript files. Use '{1}' in a JSDoc type annotation.",
        "'{0}' is a type and can't be imported in JavaScript files.",
    ));

    // 6000-series
    m.insert(6133, ErrorInfo::new(
        "'{0}' is declared but its value is never read.",
        "'{0}' is declared but never used.",
    ));
    m.insert(6142, ErrorInfo::new(
        "Module '{0}' was resolved to '{1}', but '--resolveJsonModule' is not used.",
        "Module '{0}' is imported but '--resolveJsonModule' is not enabled in your tsconfig.",
    ));
    m.insert(6196, ErrorInfo::new(
        "'{0}' is declared but never used.",
        "'{0}' is declared but never used.",
    ));
    m.insert(6244, ErrorInfo::new(
        "Module '{0}' was resolved to '{1}', but '--jsx' is not set.",
        "Module '{0}' was resolved but '--jsx' is not set in your tsconfig.",
    ));

    // 7000-series: Strict mode errors
    m.insert(7006, ErrorInfo::new(
        "Parameter '{0}' implicitly has an '{1}' type.",
        "I don't know what type '{0}' is supposed to be, so I've defaulted it to '{1}'. Your tsconfig says I should throw an error here.",
    ));
    m.insert(7017, ErrorInfo::new(
        "Element implicitly has an 'any' type because type '{0}' has no index signature.",
        "Type '{0}' has no index signature, so element access gives an implicit 'any' type.",
    ));
    m.insert(7026, ErrorInfo::new(
        "JSX element implicitly has type 'any' because no interface 'JSX.IntrinsicElements' exists.",
        "JSX element has an implicit 'any' type because 'JSX.IntrinsicElements' doesn't exist.",
    ));
    m.insert(7053, ErrorInfo::new(
        "Element implicitly has an 'any' type because expression of type '{0}' can't be used to index type '{1}'.",
        "Expression of type '{0}' can't be used to index type '{1}'.",
    ));
    m.insert(7057, ErrorInfo::new(
        "'yield' expression implicitly results in an 'any' type because its containing generator lacks a return-type annotation.",
        "'yield' needs a type annotation on the containing generator.",
    ));
    m.insert(7061, ErrorInfo::new(
        "A mapped type may not declare properties or methods.",
        "A mapped type may not declare properties or methods.",
    ));

    // 8000-series
    m.insert(8016, ErrorInfo::new(
        "Type arguments can only be used in TypeScript files.",
        "Type arguments can only be used in TypeScript files.",
    ));

    // 17000-series
    m.insert(17004, ErrorInfo::new(
        "Cannot use JSX unless the '--jsx' flag is provided.",
        "Add 'jsx' to your tsconfig.json to use JSX.",
    ));
    m.insert(18004, ErrorInfo::new(
        "No value exists in scope for the shorthand property '{0}'. Either declare one or provide an initializer.",
        "No value exists for shorthand property '{0}'. Either declare one or provide an initializer.",
    ));

    // 95000-series
    m.insert(95050, ErrorInfo::new(
        "Convert function to an async function",
        "Consider converting this function to an async function.",
    ));

    m
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_to_regex_simple() {
        let pattern = "Property '{0}' does not exist on type '{1}'.";
        let regex = pattern_to_regex(pattern);
        let caps = regex.captures("Property 'foo' does not exist on type 'Bar'.").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "foo");
        assert_eq!(caps.get(2).unwrap().as_str(), "Bar");
    }

    #[test]
    fn test_pattern_to_regex_three_params() {
        let pattern = "Expected {0} arguments, but got {1}.";
        let regex = pattern_to_regex(pattern);
        let caps = regex.captures("Expected 2 arguments, but got 3.").unwrap();
        assert_eq!(caps.get(1).unwrap().as_str(), "2");
        assert_eq!(caps.get(2).unwrap().as_str(), "3");
    }

    #[test]
    fn test_substitute_params() {
        let template = "You're trying to access '{0}' on an object that doesn't contain it.";
        let params = vec!["foo", "Bar"];
        let result = substitute_params(template, &params);
        assert_eq!(result, "You're trying to access 'foo' on an object that doesn't contain it.");
    }

    #[test]
    fn test_extract_params() {
        let info = ERRORS.get(&2339).unwrap();
        let params = extract_params(&info.pattern, "Property 'foo' does not exist on type 'Bar'.").unwrap();
        assert_eq!(params, vec!["foo", "Bar"]);
    }
}

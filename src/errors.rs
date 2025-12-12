use std::collections::HashMap;
use std::sync::LazyLock;

pub struct ErrorInfo {
    pub translation: &'static str,
}

pub static ERRORS: LazyLock<HashMap<u32, ErrorInfo>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // 1000-series: Syntax and parsing errors
    m.insert(
        1002,
        ErrorInfo {
            translation: "You started a string with a quote but didn't close it before the line ended.",
        },
    );
    m.insert(
        1003,
        ErrorInfo {
            translation: "I was expecting a name here, but none was provided.",
        },
    );
    m.insert(
        1005,
        ErrorInfo {
            translation: "A closing delimiter is missing - check your brackets, parentheses, or braces.",
        },
    );
    m.insert(
        1006,
        ErrorInfo {
            translation: "A file cannot end in the middle of something - check for unclosed blocks or strings.",
        },
    );
    m.insert(
        1009,
        ErrorInfo {
            translation: "Trailing commas after the last element aren't allowed in this context.",
        },
    );
    m.insert(
        1014,
        ErrorInfo {
            translation: "A rest parameter (...args) must be the last parameter in a function.",
        },
    );
    m.insert(
        1015,
        ErrorInfo {
            translation: "Parameters after a rest parameter aren't allowed - rest must come last.",
        },
    );
    m.insert(
        1091,
        ErrorInfo {
            translation: "You can only use 'await' inside an async function.",
        },
    );
    m.insert(
        1109,
        ErrorInfo {
            translation: "There's a syntax issue here - check for typos or missing punctuation.",
        },
    );
    m.insert(
        1117,
        ErrorInfo {
            translation: "An object can't have two properties with the same name.",
        },
    );
    m.insert(
        1155,
        ErrorInfo {
            translation: "'const' assertions can only be applied to certain expressions.",
        },
    );
    m.insert(
        1163,
        ErrorInfo {
            translation: "A computed property name must be a simple expression type.",
        },
    );
    m.insert(
        1208,
        ErrorInfo {
            translation: "This file is being treated as a script, not a module. Add an import or export to make it a module.",
        },
    );
    m.insert(
        1240,
        ErrorInfo {
            translation: "You can't specify a type in a destructuring pattern - the type should go after the full pattern.",
        },
    );
    m.insert(
        1254,
        ErrorInfo {
            translation: "A 'const' assertion isn't allowed on this kind of type reference.",
        },
    );
    m.insert(
        1268,
        ErrorInfo {
            translation: "An 'await' expression is only valid inside an async function or at the top level of a module.",
        },
    );
    m.insert(
        1313,
        ErrorInfo {
            translation: "A class definition can't directly return a value - use methods instead.",
        },
    );
    m.insert(
        1434,
        ErrorInfo {
            translation: "You need to enable top-level await in your tsconfig to use 'await' at the module level.",
        },
    );

    // 2300-series: Name resolution and declarations
    m.insert(
        2304,
        ErrorInfo {
            translation: "I can't find this variable or type - it might not be imported or defined.",
        },
    );
    m.insert(
        2305,
        ErrorInfo {
            translation: "This isn't exported from that module - check the module's exports or your import statement.",
        },
    );
    m.insert(
        2307,
        ErrorInfo {
            translation: "The module doesn't exist or TypeScript can't find its type declarations.",
        },
    );
    m.insert(
        2312,
        ErrorInfo {
            translation: "An interface can only extend a class or another interface, not other types.",
        },
    );
    m.insert(
        2314,
        ErrorInfo {
            translation: "This generic type needs type arguments - specify them in angle brackets.",
        },
    );
    m.insert(
        2322,
        ErrorInfo {
            translation: "The type you're providing doesn't match what's expected here.",
        },
    );
    m.insert(
        2324,
        ErrorInfo {
            translation: "A required property is missing from your object.",
        },
    );
    m.insert(
        2326,
        ErrorInfo {
            translation: "The types in this assignment are incompatible.",
        },
    );
    m.insert(
        2327,
        ErrorInfo {
            translation: "An index signature is missing - the object doesn't support arbitrary string/number keys.",
        },
    );
    m.insert(
        2339,
        ErrorInfo {
            translation: "This property doesn't exist on the type - check for typos or add the property to the type definition.",
        },
    );
    m.insert(
        2344,
        ErrorInfo {
            translation: "The type argument doesn't satisfy the constraint required by this generic.",
        },
    );
    m.insert(
        2345,
        ErrorInfo {
            translation: "The argument type doesn't match what this function expects.",
        },
    );
    m.insert(
        2349,
        ErrorInfo {
            translation: "You're trying to call something that isn't a function.",
        },
    );
    m.insert(
        2352,
        ErrorInfo {
            translation: "These types can't be converted to each other - the cast isn't valid.",
        },
    );
    m.insert(
        2353,
        ErrorInfo {
            translation: "This object literal has properties that aren't in the target type.",
        },
    );
    m.insert(
        2355,
        ErrorInfo {
            translation: "This function declares a return type but doesn't return anything.",
        },
    );
    m.insert(
        2365,
        ErrorInfo {
            translation: "This operator can't be used with these types.",
        },
    );
    m.insert(
        2393,
        ErrorInfo {
            translation: "A function implementation is duplicated - each overload signature needs exactly one implementation.",
        },
    );
    m.insert(
        2414,
        ErrorInfo {
            translation: "A class name can't match a type parameter name in the same scope.",
        },
    );
    m.insert(
        2451,
        ErrorInfo {
            translation: "This variable name is already declared in this scope.",
        },
    );
    m.insert(
        2488,
        ErrorInfo {
            translation: "You need an iterator method to use for-of on this value.",
        },
    );
    m.insert(
        2551,
        ErrorInfo {
            translation: "This property doesn't exist - did you mean the suggested alternative?",
        },
    );
    m.insert(
        2552,
        ErrorInfo {
            translation: "This name doesn't exist in scope - did you mean the suggested alternative?",
        },
    );
    m.insert(
        2554,
        ErrorInfo {
            translation: "You're passing a different number of arguments than the function expects.",
        },
    );
    m.insert(
        2556,
        ErrorInfo {
            translation: "A spread argument must be from a tuple or array with known length.",
        },
    );
    m.insert(
        2571,
        ErrorInfo {
            translation: "This value has type 'unknown' - you need to narrow it before using it.",
        },
    );
    m.insert(
        2590,
        ErrorInfo {
            translation: "This union or intersection type is too complex for TypeScript to process.",
        },
    );
    m.insert(
        2604,
        ErrorInfo {
            translation: "JSX element type doesn't have a construct or call signature.",
        },
    );
    m.insert(
        2614,
        ErrorInfo {
            translation: "The module has a default export, but you're trying to use a named import.",
        },
    );
    m.insert(
        2686,
        ErrorInfo {
            translation: "This refers to a global UMD module, but you're in a module file. Add an import instead.",
        },
    );
    m.insert(
        2722,
        ErrorInfo {
            translation: "This value might be undefined - check that it exists before using it.",
        },
    );
    m.insert(
        2739,
        ErrorInfo {
            translation: "This object is missing required properties from the target type.",
        },
    );
    m.insert(
        2741,
        ErrorInfo {
            translation: "A required property is missing - this property must be provided.",
        },
    );
    m.insert(
        2749,
        ErrorInfo {
            translation: "You're using a type where a value is expected - these are different things.",
        },
    );
    m.insert(
        2761,
        ErrorInfo {
            translation: "Type-only imports can't be used as values - use a regular import.",
        },
    );
    m.insert(
        2775,
        ErrorInfo {
            translation: "Assertions require every name to be declared as a valid export.",
        },
    );
    m.insert(
        2783,
        ErrorInfo {
            translation: "A getter can't also be marked as optional - it's part of the interface.",
        },
    );

    // 5000-series
    m.insert(
        5075,
        ErrorInfo {
            translation: "'isolatedModules' requires each file to be independently compilable.",
        },
    );

    // 6000-series
    m.insert(
        6133,
        ErrorInfo {
            translation: "This variable is declared but never used - consider removing it or prefixing with underscore.",
        },
    );
    m.insert(
        6142,
        ErrorInfo {
            translation: "You're importing a module but not using anything from it.",
        },
    );
    m.insert(
        6244,
        ErrorInfo {
            translation: "The types from these two modules are incompatible.",
        },
    );

    // 7000-series: Strict mode errors
    m.insert(
        7006,
        ErrorInfo {
            translation: "This parameter needs a type annotation - TypeScript can't infer it.",
        },
    );
    m.insert(
        7017,
        ErrorInfo {
            translation: "Element access with string keys isn't allowed on this type.",
        },
    );
    m.insert(
        7026,
        ErrorInfo {
            translation: "JSX element has an implicit 'any' type - add type annotations.",
        },
    );
    m.insert(
        7053,
        ErrorInfo {
            translation: "This index expression isn't valid for this type - use a proper key type.",
        },
    );
    m.insert(
        7057,
        ErrorInfo {
            translation: "'yield' expressions need a type when the generator return type isn't inferred.",
        },
    );
    m.insert(
        7061,
        ErrorInfo {
            translation: "A mapped type must not declare properties or methods directly.",
        },
    );

    // 8000-series
    m.insert(
        8016,
        ErrorInfo {
            translation: "Type arguments can only be used in TypeScript files.",
        },
    );

    // 17000-series
    m.insert(
        17004,
        ErrorInfo {
            translation: "You can't use JSX unless you configure it in your tsconfig.",
        },
    );
    m.insert(
        18004,
        ErrorInfo {
            translation: "No value exists at this runtime position - it's only a type.",
        },
    );

    // 95000-series
    m.insert(
        95050,
        ErrorInfo {
            translation: "Consider converting this function to an async function.",
        },
    );

    m
});


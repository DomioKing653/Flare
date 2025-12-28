use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ERROR_EXPLAIN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        "E0001",
        "Unknown macro used.\n\
         Example:\n\
        ```\n\
         notExistingMacro!(); // macro notExistingMacro!() doesnt exits\n\
        ```\n\
         Fix: Ensure that the macro exists.",
    );

    m.insert(
        "E0002",
        "Cannot infer type for variable.\n\
         Example:\n\
        ```\n\
         let x; // no type, no initial value so flare cannot infer the type\n\
        ```\n\
         Fix: Specify type explicitly or assign a value.",
    );

    m.insert(
        "E0003",
        "Undefined type used.\n\
         Example:\n\
        ```\n\
         let x: MyType; // MyType not declared\n\
        ```\n\
         Fix: Define the type before use.",
    );

    m.insert(
        "E0004",
        "Type mismatch.\n\
         Example: let x: i32 = true; // i32 vs bool\n\
         Fix: Ensure the assigned value matches expected type.",
    );

    m.insert(
        "E0005",
        "Invalid binary operation.\n\
         Example: true + 5\n\
         Fix: Only use operators allowed for operand types.",
    );

    m.insert(
        "E0006",
        "Undefined variable.\n\
         Example: print(x); // x not declared\n\
         Fix: Declare variable before using it.",
    );

    m.insert(
        "E0007",
        "Variable already exists.\n\
         Example:\n\
         ```\n\
          let x = 5;\n\
          let x = 6;\n\
         ```\n\
         Fix: Use a different name.",
    );

    m.insert(
        "E0008",
        "Constant without value.\n\
         Example:\n\
         ```\n\
         const PI: number;\n\
         ```\n\
         Fix: Assign a value when declaring a constant.",
    );

    m.insert(
        "E0009",
        "Cannot reassign constant.\n\
         Example:\n\
         ```\n\
          const X = 5;
          X = 6;\n\
         ```\n\
         Fix: Constants are immutable, use a variable if reassignment is needed.",
    );

    m.insert(
        "E0010",
        "Wrong macro argument count.\n\
         Example: my_macro!(1, 2, 3); // expected 2 args\n\
         Fix: Call macro with correct number of arguments.",
    );
    m
});

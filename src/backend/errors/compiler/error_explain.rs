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
         var x; // no type, no initial value so flare cannot infer the type\n\
        ```\n\
         Fix: Specify type explicitly or assign a value.",
    );

    m.insert(
        "E0003",
        "Undefined type used.\n\
         Example:\n\
        ```\n\
         var x: MyType; // MyType not declared\n\
        ```\n\
         Fix: Use only built-in types.",
    );

    m.insert(
        "E0004",
        "Type mismatch.\n\
         Example:\n\
        ```\n\
         var foo: bool = true;
         writeLn!(foo); // bool vs string\n\
        ```\n\
         Fix: Ensure that the used value matches expected type.",
    );

    m.insert(
        "E0005",
        "Invalid binary operation.\n\
         Example:\n\
        ```\n\
         true + 5\n\
        ```\n\
         Fix: '+' is invalid operator for bool.",
    );

    m.insert(
        "E0006",
        "Undefined variable.\n\
         Example:\n\
        ```\n\
         writeLn!(x); // x not declared\n\
        ```\n\
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
         const PI: numb;\n\
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
         Example:
        ```\n\
         my_macro!(1, 2, 3); // expected 2 args\n\
        ```\n\
         Fix: Call macro with correct number of arguments.",
    );
    m.insert(
        "E0011",
        "Expected printable but found ... \n\
        Compiler expects something that can be printed; something like number and string but bool for example isnt printable\n\
        Example:\n\
        ```
        const b:bool = true;
        writeLn!(b)
        ```


        ");
    m
});

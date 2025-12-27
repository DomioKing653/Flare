use crate::compiler::instructions::Instructions;

pub fn constant_folding(code: Vec<Instructions>) -> Vec<Instructions> {
    let mut out = Vec::new();
    let mut i = 0;

    while i < code.len() {
        match (code.get(i), code.get(i + 1), code.get(i + 2)) {
            (
                Some(Instructions::PushNumber(a)),
                Some(Instructions::PushNumber(b)),
                Some(Instructions::Add),
            ) => {
                out.push(Instructions::PushNumber(a + b));
                i += 3;
            }

            _ => {
                out.push(code[i].clone());
                i += 1;
            }
        }
    }

    out
}

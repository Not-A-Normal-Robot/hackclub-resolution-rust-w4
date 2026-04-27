use core::{cell::RefCell, fmt::Write as _};

use luminia_runner::{
    interpreter::{Function, FunctionBody, Interpreter, Value},
    parser::ProgramParser,
};

static HEADER: &str = "# expected result: ";
static SCRIPTS: [&str; 1] = [include_str!("./scripts/hello.lum")];

thread_local! {
    static OUTPUT_BUF: RefCell<String> = const { RefCell::new(String::new()) };
}

#[test]
fn main() {
    for script in SCRIPTS {
        run(script);
    }
}

fn run(script: &str) {
    let first_line = script
        .lines()
        .next()
        .expect("script should not be empty")
        .trim();

    if !first_line.starts_with(HEADER) {
        panic!("script should start with `{HEADER}`:\n{script}");
    }
    let exp_out_hex = first_line.trim_start_matches(HEADER);

    let expected_bytes = match hex::decode(exp_out_hex) {
        Ok(b) => b,
        Err(e) => {
            panic!("Invalid hex: {e}\nIn script: \n{script}");
        }
    };

    let parsed = match ProgramParser::new().parse(script) {
        Ok(stmts) => stmts,
        Err(e) => {
            panic!("Failed to parse script: {e}\nIn script: \n{script}")
        }
    };

    let mut int = get_hooked_interpreter();

    OUTPUT_BUF.with_borrow_mut(|s| s.clear());

    if let Err(e) = int.run(parsed) {
        panic!("Runtime error: {e:?}\nIn script: \n{script}");
    }

    OUTPUT_BUF.with_borrow(|s| {
        assert_eq!(
            s.as_bytes(),
            &expected_bytes,
            r#"Output mismatch in script:\n"""\n{script}\n""""#
        )
    });
}

fn get_hooked_interpreter() -> Interpreter {
    let mut int = Interpreter::new();

    int.override_function(
        "print".to_string(),
        Function {
            params: vec![String::new()],
            body: FunctionBody::Native(Box::new(|vals| {
                if let Some(val) = vals.first() {
                    OUTPUT_BUF
                        .with_borrow_mut(|s| write!(s, "{val}"))
                        .expect("failed to write to buffer");
                }
                Ok(Value::Nil)
            })),
        },
    );
    int.override_function(
        "eprint".to_string(),
        Function {
            params: vec![String::new()],
            body: FunctionBody::Native(Box::new(|vals| {
                if let Some(val) = vals.first() {
                    OUTPUT_BUF
                        .with_borrow_mut(|s| write!(s, "{val}"))
                        .expect("failed to write to buffer");
                }
                Ok(Value::Nil)
            })),
        },
    );
    int.override_function(
        "println".to_string(),
        Function {
            params: vec![String::new()],
            body: FunctionBody::Native(Box::new(|vals| {
                if let Some(val) = vals.first() {
                    OUTPUT_BUF
                        .with_borrow_mut(|s| writeln!(s, "{val}"))
                        .expect("failed to write to buffer");
                }
                Ok(Value::Nil)
            })),
        },
    );
    int.override_function(
        "eprintln".to_string(),
        Function {
            params: vec![String::new()],
            body: FunctionBody::Native(Box::new(|vals| {
                if let Some(val) = vals.first() {
                    OUTPUT_BUF
                        .with_borrow_mut(|s| writeln!(s, "{val}"))
                        .expect("failed to write to buffer");
                }
                Ok(Value::Nil)
            })),
        },
    );

    int
}

use sheeplang::{compiler::compile_and_run, parser::smart::SheepParser};

fn main() {
    let code = r#"
    let id = \x.x;
    let two = \f.\x.f (f x);
    let three = SUCC two;

    ADD two three PEEK_NUM
"#;

    let program = SheepParser::parse_program(code).unwrap();
    println!("Parsed AST: {:#?}", program);

    compile_and_run(code).unwrap();
}

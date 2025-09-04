use sheeplang::compiler::compile_and_run;

fn main() {
    let code = r#"
    let id = \x.x;
    let two = \f.\x.f (f x);
    let three = SUCC two;
    ADD two three AS_NAT
"#;

    compile_and_run(code).unwrap();
}

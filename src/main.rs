use sheeplang::{parser::smart::SheepParser, vm::{Instruction, VM}};

fn main() {
    let code = "
let id = \\x.x;
let add = \\m.\\n.\\f.\\x.m f (n f x);
let two = \\f.\\x.f (f x);
let three = \\f.\\x.f (f (f x));

add two three AS_NAT";

    let parser = SheepParser::parse_program(code);

    dbg!(parser);
}

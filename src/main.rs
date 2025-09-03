use sheeplang::vm::{Instruction, VM};

fn main() {
    println!("Lambda Calculus VM");

    // Church One: λf. λx. f x
    let church_one = Instruction::lam(vec![
        Instruction::lam(vec![
            Instruction::var(1),  // f
            Instruction::var(0),  // x
            Instruction::app()    // f x
        ])
    ]);

    // Church Two: λf. λx. f (f x)
    let church_two = Instruction::lam(vec![
        Instruction::lam(vec![
            Instruction::var(1),  // f
            Instruction::var(1),  // f
            Instruction::var(0),  // x
            Instruction::app(),   // f x
            Instruction::app()    // f (f x)
        ])
    ]);

    // Addition: λm. λn. λf. λx. m f (n f x)
    let add = Instruction::lam(vec![
        Instruction::lam(vec![
            Instruction::lam(vec![
                Instruction::lam(vec![
                    // m f (n f x)
                    Instruction::var(3),  // m
                    Instruction::var(1),  // f
                    Instruction::app(),   // m f
                    Instruction::var(2),  // n
                    Instruction::var(1),  // f
                    Instruction::app(),   // n f
                    Instruction::var(0),  // x
                    Instruction::app(),   // n f x
                    Instruction::app()    // m f (n f x)
                ])
            ])
        ])
    ]);

    let program = vec![
        add,
        church_one,
        Instruction::app(),
        church_two,
        Instruction::app()
    ];

    let mut vm = VM::new();
    let result = vm.run(&program).unwrap();

    println!("{:}", result);
}

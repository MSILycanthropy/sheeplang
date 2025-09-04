#[derive(Debug, Clone)]
pub enum Instruction {
    Var(usize),
    Lam(Vec<Instruction>),
    App,
    Peek(PeekAs),
}

impl Instruction {
    pub fn var(idx: usize) -> Self {
        Instruction::Var(idx)
    }

    pub fn lam(body: Vec<Instruction>) -> Self {
        Instruction::Lam(body)
    }

    pub fn app() -> Self {
        Instruction::App
    }
}

#[derive(Debug, Clone)]
pub enum PeekAs {
    Number,
    Bool,
    List,
}

impl From<&str> for PeekAs {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "peek_num" => PeekAs::Number,
            "peek_bool" => PeekAs::Bool,
            "peek_list" => PeekAs::List,
            _ => PeekAs::Number,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Closure {
    body: Vec<Instruction>,
    env: Env,
}

impl std::fmt::Display for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

type Env = Vec<Closure>;
type EvalResult = Result<Closure, String>;

#[derive(Default)]
pub struct VM {
    stack: Env,
}

impl VM {
    pub fn eval(&mut self, instructions: &[Instruction], env: &Env) -> EvalResult {
        for instr in instructions {
            match instr {
                Instruction::Var(idx) => {
                    if *idx >= env.len() {
                        return Err(format!("Unbound Variable: {}", idx));
                    }

                    self.stack.push(env[env.len() - 1 - idx].clone());
                }
                Instruction::Lam(body) => {
                    let closure = Closure {
                        body: body.clone(),
                        env: env.clone(),
                    };

                    self.stack.push(closure)
                }
                Instruction::App => {
                    if self.stack.len() < 2 {
                        return Err(
                            "Not enough arguments to invoke Application on stack".to_string()
                        );
                    }

                    let arg = self.stack.pop().ok_or("No argument found".to_string())?;
                    let func = self.stack.pop().ok_or("No closure found".to_string())?;

                    let mut env = func.env;
                    let body = func.body;

                    env.push(arg);
                    let result = self.eval(&body, &env)?;
                    self.stack.push(result);
                }
                Instruction::Peek(peek_as) => {
                    let closure = self.stack.pop().ok_or("No closure to observe")?;

                    self.try_peek(&closure, peek_as)
                }
            }
        }

        if self.stack.is_empty() {
            return Err("No result on stack".to_string());
        }

        Ok(self.stack.pop().expect("how we get here"))
    }

    pub fn run(&mut self, program: &[Instruction]) -> EvalResult {
        let env = vec![];

        self.eval(program, &env)
    }

    fn try_peek(&mut self, closure: &Closure, peek_as: &PeekAs) {
        match peek_as {
            PeekAs::Number => match self.try_as_church_numeral(closure) {
                Some(n) => println!("{}", n),
                None => println!("<not a number>"),
            },
            PeekAs::Bool => match self.try_as_church_boolean(closure) {
                Some(true) => println!("true"),
                Some(false) => println!("false"),
                None => println!("<not a boolean>"),
            },
            _ => println!("<unsupported peek>"),
        }
    }

    fn try_as_church_numeral(&mut self, closure: &Closure) -> Option<i32> {
        let body = &closure.body;

        Some(8)
    }

    fn try_as_church_boolean(&self, closure: &Closure) -> Option<bool> {
        let body = &closure.body;

        Some(false)
    }
}
    
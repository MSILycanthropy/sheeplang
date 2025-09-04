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

        if let [Instruction::Lam(outer_body)] = body.as_slice() {
            if let [Instruction::Lam(inner_body)] = outer_body.as_slice() {
                return self.count_applications(inner_body);
            }

            return Some(self.count_applications_env(outer_body));
        }

        None
    }

    fn try_as_church_boolean(&self, closure: &Closure) -> Option<bool> {
        let body = &closure.body;

        // Church boolean patterns:
        // TRUE = λx.λy.x → Lam([Lam([Var(1)])])
        // FALSE = λx.λy.y → Lam([Lam([Var(0)])])

        match body.as_slice() {
            [Instruction::Lam(outer_body)] => {
                match outer_body.as_slice() {
                    [Instruction::Lam(inner_body)] => {
                        match inner_body.as_slice() {
                            [Instruction::Var(1)] => Some(true),  // TRUE - return first arg
                            [Instruction::Var(0)] => Some(false), // FALSE - return second arg
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn count_applications(&self, instructions: &[Instruction]) -> Option<i32> {
        // Pattern for Church numeral n: Var(1)^n Var(0) App^n
        // Where Var(1) is f, Var(0) is x, and we apply f n times
        match instructions {
            // Zero: λf.λx.x → just Var(0)
            [Instruction::Var(0)] => Some(0),

            // One: λf.λx.f x → Var(1) Var(0) App
            [Instruction::Var(1), Instruction::Var(0), Instruction::App] => Some(1),

            // Two: λf.λx.f (f x) → Var(1) Var(1) Var(0) App App
            [
                Instruction::Var(1),
                Instruction::Var(1),
                Instruction::Var(0),
                Instruction::App,
                Instruction::App,
            ] => Some(2),

            // Three: λf.λx.f (f (f x)) → Var(1) Var(1) Var(1) Var(0) App App App
            [
                Instruction::Var(1),
                Instruction::Var(1),
                Instruction::Var(1),
                Instruction::Var(0),
                Instruction::App,
                Instruction::App,
                Instruction::App,
            ] => Some(3),

            // General case: count leading Var(1)s
            _ => {
                let mut count = 0;
                for instr in instructions {
                    match instr {
                        Instruction::Var(1) => count += 1,
                        Instruction::Var(0) => break,
                        _ => break,
                    }
                }

                let expected_apps = count;
                let mut app_count = 0;
                let mut found_var_0 = false;

                for instr in instructions.iter().skip(count as usize) {
                    match instr {
                        Instruction::Var(0) if !found_var_0 => found_var_0 = true,
                        Instruction::App => app_count += 1,
                        _ => return None,
                    }
                }

                if found_var_0 && app_count == expected_apps {
                    Some(count)
                } else {
                    None
                }
            }
        }
    }

    fn count_applications_env(&self, instructions: &[Instruction]) -> i32 {
        let mut app_count = 0;
        for instr in instructions {
            if let Instruction::App = instr {
                app_count += 1
            }
        }

        app_count
    }
}

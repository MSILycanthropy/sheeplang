#[derive(Debug, Clone)]
pub enum Instruction {
  Var(usize),
  Lam(Vec<Instruction>),
  App,
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

pub struct VM {
  stack: Env,
}

impl VM {
  pub fn new() -> Self {
    Self { stack: vec![] } 
  }

  pub fn eval(&mut self, instructions: &[Instruction], env: &Env) -> EvalResult {
    for instr in instructions {
      match instr {
        Instruction::Var(idx) => {
          if *idx >= env.len() {
            return Err(format!("Unbound Variable: {}", idx));
          }

          self.stack.push(env[env.len() - 1 - idx].clone());
        },
        Instruction::Lam(body) => {
          let closure = Closure {
            body: body.clone(),
            env: env.clone(),
          };

          self.stack.push(closure)
        },
        Instruction::App => {
          if self.stack.len() < 2 {
            return Err("Not enough arguments to invoke Application on stack".to_string());
          }

          let arg = self.stack.pop().ok_or("No argument found".to_string())?;
          let func = self.stack.pop().ok_or("No closure found".to_string())?;

          let mut env = func.env;
          let body = func.body;

          env.push(arg);
          let result = self.eval(&body, &env)?;
          self.stack.push(result);
        }
      }

    }

    if self.stack.is_empty() {
      return Err("No result on stack".to_string())
    }

    Ok(self.stack.pop().expect("how we get here"))
  } 

  pub fn run(&mut self, program: &[Instruction]) -> EvalResult {
    let env = vec![];

    self.eval(program, &env)
  }
}

use crate::memory::*;
use crate::token::*;
use std::io::*;

pub enum Statement {
    Move(isize),
    Change(isize),
    Input,
    Output,
    Block(Vec<Statement>),
}

pub struct Program {
    pub block: Statement,
}
impl Statement {
    fn execute(&self, memory: &mut Memory) {
        match self {
            Statement::Move(d) => memory.move_pointer(*d),
            Statement::Change(d) => memory.set_data((memory.get_data() as isize + *d) as u8),
            Statement::Output => {
                print!("{}", memory.get_data() as char);
            }
            Statement::Input => {
                for byte in stdin().lock().bytes() {
                    if let Ok(b) = byte {
                        memory.set_data(b);
                        break;
                    }
                }
            }
            Statement::Block(v) => {
                while memory.get_data() != 0 {
                    for s in v {
                        s.execute(memory);
                    }
                }
            }
        }
    }
}

impl Program {
    pub fn run(&self, memory: &mut Memory) {
        if let Statement::Block(ref v) = self.block {
            for s in v {
                s.execute(memory);
            }
        }
    }
}

fn block_recursion(s: &mut Statement, program: &mut TokenProgram) {
    if let Statement::Block(b) = s {
        loop {
            match program.next() {
                Some(Token::Move(d)) => b.push(Statement::Move(*d)),
                Some(Token::Change(d)) => b.push(Statement::Change(*d)),
                Some(Token::Input) => b.push(Statement::Input),
                Some(Token::Output) => b.push(Statement::Output),
                Some(Token::Start) => {
                    b.push(Statement::Block(Vec::new()));
                    if let Some(e) = b.last_mut() {
                        block_recursion(e, program);
                    }
                }
                _ => return,
            }
        }
    }
}

pub fn get_program(token_program: &mut TokenProgram) -> Program {
    let mut program = Program {
        block: Statement::Block(Vec::new()),
    };
    token_program.reset();
    block_recursion(&mut program.block, token_program);
    return program;
}

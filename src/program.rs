use crate::memory::*;
use crate::token::*;
use std::io::*;

#[derive(Debug)]
pub enum Statement {
    Move(isize),
    Change(isize),
    Input(usize),
    Output(usize),
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
            Statement::Output(d) => {
                for _ in 0..(*d) {
                    print!("{}", memory.get_data() as char);
                }
            }
            Statement::Input(d) => {
                let mut i = 0;
                for byte in stdin().lock().bytes() {
                    if let Ok(b) = byte {
                        memory.set_data(b);
                        i = i + 1;
                        if i >= *d {
                            break;
                        }
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
    loop {
        let result = program.next();
        match result {
            Some(e) => {
                if let Statement::Block(b) = s {
                    match e {
                        Token::Move(d) => b.push(Statement::Move(*d)),
                        Token::Change(d) => b.push(Statement::Change(*d)),
                        Token::Input(d) => b.push(Statement::Input(*d)),
                        Token::Output(d) => b.push(Statement::Output(*d)),
                        Token::Start => {
                            b.push(Statement::Block(Vec::new()));
                            if let Some(e) = b.last_mut() {
                                block_recursion(e, program);
                            }
                        }
                        Token::End => {
                            return;
                        }
                    }
                }
            }
            None => (return),
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

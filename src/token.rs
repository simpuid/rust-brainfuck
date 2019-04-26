pub enum Token {
    Move(isize),
    Change(isize),
    Input,
    Output,
    Start,
    End,
}
enum Braces {
    Left(usize, usize),
    Right(usize, usize),
}
pub struct TokenProgram {
    pub tokens: Vec<Token>,
    counter: usize,
}
impl TokenProgram {
    pub fn reset(&mut self) {
        self.counter = 0;
    }
    pub fn next(&mut self) -> Option<&Token> {
        let tmp = self.tokens.get(self.counter);
        self.counter += 1;
        return tmp;
    }
}

pub fn parse(data: &str) -> Result<TokenProgram, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut braces: Vec<Braces> = Vec::new();
    let mut line: usize = 1;
    let mut column: usize = 1;
    for c in data.chars() {
        match c {
            '[' => {
                tokens.push(Token::Start);
                braces.push(Braces::Left(line, column));
            }
            ']' => {
                tokens.push(Token::End);
                if let Some(Braces::Left(_, _)) = braces.last() {
                    braces.pop();
                } else {
                    braces.push(Braces::Right(line, column));
                }
            }
            '>' | '<' => {
                let mut delta: isize = 0;
                if let Some(Token::Move(d)) = tokens.last() {
                    delta = *d;
                    tokens.pop();
                }
                tokens.push(Token::Move(delta + if c == '<' { -1 } else { 1 }));
            }
            '+' | '-' => {
                let mut delta: isize = 0;
                if let Some(Token::Change(d)) = tokens.last() {
                    delta = *d;
                    tokens.pop();
                }
                tokens.push(Token::Change(delta + if c == '-' { -1 } else { 1 }));
            }
            '.' => {
                tokens.push(Token::Output);
            }
            ',' => {
                tokens.push(Token::Input);
            }
            '\n' => {
                line += 1;
                column = 1;
                continue;
            }
            _ => (),
        };
        column += 1;
    }
    if braces.len() == 0 {
        return Ok(TokenProgram {
            tokens: tokens,
            counter: 0,
        });
    } else {
        let mut positions = String::new();
        for b in braces {
            match b {
                Braces::Left(line, column) => {
                    positions += &format!("\t'[' @ line:{} column:{}\n", line, column)[..]
                }
                Braces::Right(line, column) => {
                    positions += &format!("\t']' @ line:{} column:{}\n", line, column)[..]
                }
            }
        }
        return Err(format!("Braces mismatch \n{}", positions));
    }
}

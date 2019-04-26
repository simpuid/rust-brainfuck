#[derive(Debug)]
pub enum Token {
    Move(isize),
    Change(isize),
    Input(usize),
    Output(usize),
    Start,
    End,
}
#[derive(Debug)]
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
                if let Some(e) = braces.last() {
                    if let Braces::Left(_, _) = e {
                        braces.pop();
                    } else {
                        braces.push(Braces::Right(line, column));
                    }
                } else {
                    braces.push(Braces::Right(line, column));
                }
            }
            '>' | '<' => {
                let mut delta: isize = 0;
                if let Some(s) = tokens.last() {
                    if let Token::Move(d) = s {
                        delta = *d;
                        tokens.pop();
                    }
                }
                tokens.push(Token::Move(delta + if c == '<' { -1 } else { 1 }));
            }
            '+' | '-' => {
                let mut delta: isize = 0;
                if let Some(s) = tokens.last() {
                    if let Token::Change(d) = s {
                        delta = *d;
                        tokens.pop();
                    }
                }
                tokens.push(Token::Change(delta + if c == '-' { -1 } else { 1 }));
            }
            '.' => {
                let mut delta: usize = 0;
                if let Some(s) = tokens.last() {
                    if let Token::Output(d) = s {
                        delta = *d;
                        tokens.pop();
                    }
                }
                tokens.push(Token::Output(delta + 1));
            }
            ',' => {
                let mut delta: usize = 0;
                if let Some(s) = tokens.last() {
                    if let Token::Input(d) = s {
                        delta = *d;
                        tokens.pop();
                    }
                }
                tokens.push(Token::Input(delta + 1));
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
                Braces::Left(line, column) | Braces::Right(line, column) => {
                    positions += &format!(
                        "\t{} at line:{} column:{}\n",
                        if let Braces::Left(_, _) = b { '[' } else { ']' },
                        line,
                        column
                    )[..]
                }
            }
        }
        return Err(format!("Braces mismatch \n{}", positions));
    }
}

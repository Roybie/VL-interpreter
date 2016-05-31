use commands::{Command, Type};

use std::char;

/**
 * Internal parsed representation of source
 */

//pub struct Ast(Vec<Command>);
pub type Ast = Vec<Command>;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn parse(&mut self) -> Ast {
        let mut coms = Ast::new();
        coms.push(Command::Line);

        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            let command = self.parse_command();
            match command {
                Command::Comment => {
                    continue;
                },
                _ => (),
            }
            coms.push(command);

        }
        coms.pop();
        coms.push(Command::Nil);
        coms
    }

    fn parse_command(&mut self) -> Command {
        match Command::from_char(self.next_char(true)) {
            Command::InsV(_) => self.parse_insv(),
            Command::InsI(_) => self.parse_insi(),
            Command::Mark(_) => self.parse_mark(),
            Command::Ind(_) => self.parse_ind(),
            Command::Next(_) => self.parse_next(),
            Command::Prev(_) => self.parse_prev(),
            Command::Int(_) => self.parse_int(),
            Command::LP => self.parse_grp(),
            Command::Comment => self.parse_com(),
            com => com,
        }
    }

    fn parse_insi(&mut self) -> Command {
        //find until first unescaped !
        let mut insert = String::new();
        let mut next;

        loop {
            if self.eof() {
                panic!("unclosed insert, reached end of file");
            }
            next = self.next_char(true);
            if next == '\\' && self.next_char(false) == ';' {
                next = self.next_char(true);
            } else if next == ';' {
                return self.get_insi_type(insert);
            }
            insert.push(next);
        }
    }

    fn parse_insv(&mut self) -> Command {
        //find until first unescaped !
        let mut insert = String::new();
        let mut next;

        loop {
            if self.eof() {
                panic!("unclosed insert, reached end of file");
            }
            next = self.next_char(true);
            if next == '\\' && self.next_char(false) == ';' {
                next = self.next_char(true);
            } else if next == ';' {
                return self.get_insv_type(insert);
            }
            insert.push(next);
        }
    }

    fn get_insv_type(&mut self, insert: String) -> Command {
        match insert.parse::<i64>() {
            Ok(n) => Command::InsV(Type::I(n)),
            Err(_) => Command::InsV(Type::S(insert)),
        }
    }

    fn get_insi_type(&mut self, insert: String) -> Command {
        match insert.parse::<i64>() {
            Ok(n) => Command::InsI(Type::I(n)),
            Err(_) => Command::InsI(Type::S(insert)),
        }
    }

    fn parse_mark(&mut self) -> Command {
        match self.next_char(true) {
            c @ 'a'...'z' => Command::Mark(c),
            _ => { panic!("Can only use characters a to z as marks"); }
        }
    }

    fn parse_ind(&mut self) -> Command {
        match self.next_char(true) {
            c @ 'a'...'z' => Command::Ind(c),
            _ => { panic!("Can only use characters a to z as index"); }
        }
    }

    fn parse_next(&mut self) -> Command {
        Command::Next(Box::new(Command::from_char(self.next_char(true))))
    }

    fn parse_prev(&mut self) -> Command {
        Command::Prev(Box::new(Command::from_char(self.next_char(true))))
    }

    fn parse_int(&mut self) -> Command {
        self.pos = self.pos - 1;
        let int = self.consume_while(|c| c.is_digit(10));
        Command::Int(int.parse::<u64>().unwrap())
    }

    fn parse_com(&mut self) -> Command {
         self.consume_while(|c| c != '$');
         self.pos = self.pos + 1;
         Command::Comment
    }

    fn parse_grp(&mut self) -> Command {
        let mut coms = Ast::new();
        coms.push(Command::Line);
        loop {
            self.consume_whitespace();
            if self.eof() {
                panic!("End of file reached but ) expected");
            } else if self.next_char(false) == ')' {
                break;
            }
            let command = self.parse_command();
            match command {
                Command::Comment => {
                    continue;
                },
                _ => (),
            }
            coms.push(command);
        }
        self.next_char(true);

        Command::Grp(coms)
    }

    fn next_char(&mut self, consume: bool) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        if consume {
            self.pos += next_pos;
        }
        return cur_char;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c != '\n' && char::is_whitespace(c));
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut res = String::new();
        while !self.eof() && test(self.next_char(false)) {
            res.push(self.next_char(true));
        }
        res
    }
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

pub fn parse(input: String) -> Ast {
    let ast = Parser { pos: 0, input: input }.parse();

    ast
}

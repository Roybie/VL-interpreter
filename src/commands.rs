use ast::Ast;
use std::fmt;

/**
 * Representation of commands for AST
 */

#[derive(Debug, Clone)]
pub enum Command {
    // Manipulate <value>
    Put,            // p
    Yank,           // y
    Ins(Type),      // i<string>
    Incr,           // a
    Decr,           // x
    // Manipulate <pointer> / <value>
    Mark(char),     // '<char>
    Ind(char),      // `<char>
    NMar,           // ] :- increase <pointer> to next mark
    PMar,           // [ :- decrease <pointer> to previous mark
    NInd,           // } :- increase <index>
    PInd,           // { :- decrease <index>
    // Program Flow (jumps)
    Line,           // New line used for Down and Up commands
    Down,           // j :- jump down <int> lines
    Up,             // k :- jump up <int> lines
    Next(Box<Command>),     // f<char> :- jump forward in line to <int>th <char> if exists
    Prev(Box<Command>),     // F<char> :- jump back in line to <int>th <char> if exists
    Con,            // ? :- do jump if <value> == <int>
    NCon,           // ; :- do jump if <value> != <int>
    Int(u64),       // set <int> to direct integer
    VInt,           // v :- copy <value> to <int>
    Rep,            // . :- do last command or Grp <int> times
    LP,             // ( :- start Grp
    // Group of commands inside () for loops
    Grp(Ast),
    Comment,        //comment
    Nil,            // invalid command
}

impl Command {
    pub fn from_char(inp: char) -> Command {
        match inp {
            'p' => Command::Put,
            'y' => Command::Yank,
            'i' => Command::Ins(Type::C('a')),
            'a' => Command::Incr,
            'x' => Command::Decr,
            '\'' => Command::Mark('a'),
            '`' => Command::Ind('a'),
            ']' => Command::NMar,
            '[' => Command::PMar,
            '}' => Command::NInd,
            '{' => Command::PInd,
            '\n' => Command::Line,
            'j' => Command::Down,
            'k' => Command::Up,
            'f' => Command::Next(Box::new(Command::Nil)),
            'F' => Command::Prev(Box::new(Command::Nil)),
            '?' => Command::Con,
            '!' => Command::NCon,
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => Command::Int(inp as u64),
            'v' => Command::VInt,
            '.' => Command::Rep,
            '(' => Command::LP,
            '/' => Command::Comment,
            _ => Command::Nil,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    S(String),
    I(i64),
    C(char),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Type::S(data) => write!(f, "{}", data),
            Type::I(data) => write!(f, "{}", data),
            Type::C(data) => write!(f, "{}", data),
        }
    }
}

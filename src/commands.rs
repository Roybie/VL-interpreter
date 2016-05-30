use ast::Ast;
use std::fmt;

/**
 * Representation of commands for AST
 */

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // stdio
    Out,            //w
    OutL,           //W
    In,             //e
    // Manipulate <value>
    Put,            // p put value in selected memory
    PutI,           // P put Int in selected memory
    Yank,           // y value from selected memory
    YankI,          // Y int from selected memory
    Ins(Type),      // i<string>
    Incr,           // a
    Decr,           // x
    //Arithmetic
    Plus,
    Minus,
    Times,
    Divide,
    // Manipulate <pointer> / <value>
    Mark(char),     // '<char>
    Ind(char),      // `<char>
    NMar,           // ] :- increase <pointer> to next mark
    PMar,           // [ :- decrease <pointer> to previous mark
    NInd,           // } :- increase <index>
    PInd,           // { :- decrease <index>
    // Program Flow (jumps)
    BLine,          // ^ Jump to beginning of current line
    Line,           // New line used for Down and Up commands
    Down,           // j :- jump down <int> lines
    Up,             // k :- jump up <int> lines
    Next(Box<Command>),     // f<char> :- jump forward in line to <int>th <char> if exists
    Prev(Box<Command>),     // F<char> :- jump back in line to <int>th <char> if exists
    Con,            // ? :- do jump if <value> == <int>
    NCon,           // ; :- do jump if <value> != <int>
    LCon,           // < :- do jump if <value> > <int>
    GCon,           // > :- do jump if <value> < <int>
    Int(u64),       // set <int> to direct integer
    V2I,           // V :- copy <value> to <int>
    I2V,           // v :- copy <int> to <value>
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
            'w' => Command::Out,
            'e' => Command::In,
            'W' => Command::OutL,
            'p' => Command::Put,
            'P' => Command::PutI,
            'y' => Command::Yank,
            'Y' => Command::YankI,
            'i' => Command::Ins(Type::C('a')),
            'a' => Command::Incr,
            'x' => Command::Decr,
            '+' => Command::Plus,
            '-' => Command::Minus,
            '*' => Command::Times,
            '/' => Command::Divide,
            '\'' => Command::Mark('a'),
            '`' => Command::Ind('a'),
            ']' => Command::NMar,
            '[' => Command::PMar,
            '}' => Command::NInd,
            '{' => Command::PInd,
            '^' => Command::BLine,
            '\n' => Command::Line,
            'j' => Command::Down,
            'k' => Command::Up,
            'f' => Command::Next(Box::new(Command::Nil)),
            'F' => Command::Prev(Box::new(Command::Nil)),
            '?' => Command::Con,
            '!' => Command::NCon,
            '<' => Command::LCon,
            '>' => Command::GCon,
            '0'...'9' => Command::Int(inp as u64),
            'V' => Command::V2I,
            'v' => Command::I2V,
            '.' => Command::Rep,
            '(' => Command::LP,
            '$' => Command::Comment,
            _ => Command::Nil,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

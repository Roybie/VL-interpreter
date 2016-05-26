use ast::Ast;
use commands::{Command, Type};

use std::char;
use std::collections::HashMap;

pub struct Interpreter {
    values: HashMap<char, Vec<Type>>,
    pointer: char,
    index: usize,
    value: Type,
    int: u64,
    pc: usize,

    program: Ast,
}

impl Interpreter {

    pub fn new(program: Ast) -> Interpreter {
        Interpreter {
            pointer: 'a',
            index: 0,
            int: 1,
            pc: 0,
            values: HashMap::new(),
            value: Type::I(0),
            program: program,
        }
    }

    fn set_position(&mut self) {
        match self.values.get(&self.pointer) {
            Some(value) => {
                match value.get(self.index) {
                    Some(typ) => { self.value = typ.clone(); },
                    None => {
                        self.value = Type::I(0);
                    },
                }
            },
            None => {
                self.value = Type::I(0);
            },
        }
    }

    fn set_value(&mut self, val: Type) {
        self.value = val.clone();
        if !self.values.contains_key(&self.pointer) {
            let mut vec = vec![Type::I(0); self.index + 1];
            vec[self.index] = val.clone();
            self.values.insert(self.pointer, vec);
        }

        let mut values = self.values.get_mut(&self.pointer).unwrap();

        if values.len() < self.index + 1 {
            values.resize(self.index + 1, Type::I(0));
            values[self.index] = val.clone();
        }
    }

    pub fn run(&mut self) -> usize {
        loop {
            let command = self.program[self.pc].clone();

            match command {
                Command::Nil => {
                    break;
                },
                command => {
                    self.interpret(command);
                    //interpret command
                }
            }
            self.pc = self.pc + 1;
        }
        0
    }

    fn interpret(&mut self, command: Command) {
        match command {
            Command::Put => { println!("{}", self.value); },
            Command::Yank => {},
            Command::Ins(val) => { self.set_value(val); },
            Command::Incr => {
                match self.value {
                    Type::I(v) => self.set_value(Type::I(v + 1)),
                    _ => (),
                }
            },
            Command::Decr => {
                match self.value {
                    Type::I(v) => self.set_value(Type::I(v - 1)),
                    _ => (),
                }
            },
            Command::Mark(mark) => {
                match mark {
                    'a' ... 'z' => {
                        self.pointer = mark;
                        self.index = 0;
                        self.set_position();
                    },
                    _ => (),
                }
            },
            Command::Ind(mark) => {
                match mark {
                    'a' ... 'z' => {
                        self.pointer = mark;
                        match self.value {
                            Type::I(int) => { self.index = int as usize },
                            _ => { panic!("value not an integer"); },
                        }
                        self.set_position();
                    },
                    _ => (),
                }
            },
            Command::NMar => {
                let mut conv = self.pointer.to_digit(36).unwrap();
                conv = conv + 1;
                if conv > 35 {
                    conv = 10;
                }
                self.pointer = char::from_digit(conv, 36).unwrap();
                self.index = 0;
                self.set_position();
            },
            Command::PMar => {
                let mut conv = self.pointer.to_digit(36).unwrap();
                conv = conv - 1;
                if conv < 10  {
                    conv = 35;
                }
                self.pointer = char::from_digit(conv, 36).unwrap();
                self.index = 0;
                self.set_position();
            },
            Command::NInd => {
                self.index = self.index + 1;
                self.set_position();
            },
            Command::PInd => {
                if self.index ==0 {
                    panic!("Can't decrement index, already 0");
                }
                self.index = self.index - 1;
                self.set_position();
            },
            Command::Down => {},
            Command::Up => {},
            Command::Next(next) => {},
            Command::Prev(prev) => {},
            Command::Con => {},
            Command::NCon => {},
            Command::Int(int) => {},
            Command::VInt => {},
            Command::Rep => {},
            Command::Grp(group) => {},
            Command::Nil => {},
            _ => (),
        }
    }
}

pub fn run(program: Ast) -> usize {
    let mut interpreter = Interpreter::new(program);
    interpreter.run()
}

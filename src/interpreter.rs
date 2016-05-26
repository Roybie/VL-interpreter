use ast::Ast;
use commands::{Command, Type};

pub struct Interpreter {
    values: Vec<Vec<Type>>,
    pointer: usize,
    index: usize,
    value: Type,
    int: u64,
    pc: usize,
    dojump: bool,

    last: Command,
    program: Ast,
}

impl Interpreter {

    pub fn new(program: Ast) -> Interpreter {
        Interpreter {
            pointer: 0,
            index: 0,
            int: 1,
            pc: 0,
            values: Vec::new(),
            value: Type::I(0),
            last: Command::Nil,
            dojump: true,
            program: program,
        }
    }

    fn set_position(&mut self) {
        let mut existed = true;
        match self.values.get(self.pointer) {
            Some(value) => {
                match value.get(self.index) {
                    Some(typ) => { self.value = typ.clone(); },
                    None => {
                        existed = false;
                    },
                }
            },
            None => {
                existed = false;
            },
        }
        if !existed {
            self.set_value(Type::I(0));
        }
    }

    fn set_value(&mut self, val: Type) {
        self.value = val.clone();
        if self.values.len() < self.pointer + 1 {
            self.values.resize(self.pointer + 1, Vec::new());
            let mut vec = vec![Type::I(0); self.index + 1];
            vec[self.index] = val.clone();
            self.values[self.pointer] = vec;
            return;
        }

        if self.values[self.pointer].len() < self.index + 1 {
            self.values[self.pointer].resize(self.index + 1, Type::I(0));
        }

        self.values[self.pointer][self.index] = val.clone();
    }

    pub fn run(&mut self) -> usize {
        loop {
            let command = self.program[self.pc].clone();

            match command {
                Command::Nil => {
                    break;
                },
                //loopable commands
                Command::Put |
                Command::Incr |
                Command::Decr |
                Command::NMar |
                Command::PMar |
                Command::NInd |
                Command::PInd |
                Command::Down |
                Command::Up |
                Command::Rep |
                Command::Next(_) |
                Command::Prev(_) |
                Command::Grp(_) => {

                    //interpret command
                    while self.int > 0 {
                        self.interpret(command.clone());
                        self.int = self.int - 1;
                    }
                    self.int = 1;
                },
                command => {
                    self.interpret(command.clone());
                },
            }
            self.pc = self.pc + 1;
        }
        0
    }

    fn interpret(&mut self, command: Command) {
        match command.clone() {
            Command::Put => {
                self.last = command;
                println!("{}", self.value);
            },
            Command::Yank => {
                //TODO
            },
            Command::Ins(val) => {
                self.last = command;
                self.set_value(val.clone());
            },
            Command::Incr => {
                self.last = command;
                match self.value {
                    Type::I(v) => self.set_value(Type::I(v + 1)),
                    _ => (),
                }
            },
            Command::Decr => {
                self.last = command;
                match self.value {
                    Type::I(v) => self.set_value(Type::I(v - 1)),
                    _ => (),
                }
            },
            Command::Mark(mark) => {
                match mark {
                    'a' ... 'z' => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = 0;
                        self.set_position();
                    },
                    _ => { panic!("mark must be a to z"); },
                }
            },
            Command::Ind(mark) => {
                match mark {
                    'a' ... 'z' => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        match self.value {
                            Type::I(int) => { self.index = int as usize },
                            _ => { panic!("value not an integer"); },
                        }
                        self.set_position();
                    },
                    _ => { panic!("mark must be a to z"); },
                }
            },
            Command::NMar => {
                self.last = command;
                self.pointer = self.pointer + 1;
                if self.pointer > 26 {
                    self.pointer = 0;
                }
                self.index = 0;
                self.set_position();
            },
            Command::PMar => {
                self.last = command;
                if self.pointer == 0 {
                    self.pointer = 27;
                }
                self.pointer = self.pointer - 1;
                self.index = 0;
                self.set_position();
            },
            Command::NInd => {
                self.last = command;
                self.index = self.index + 1;
                self.set_position();
            },
            Command::PInd => {
                self.last = command;
                if self.index ==0 {
                    panic!("Can't decrement index, already 0");
                }
                self.index = self.index - 1;
                self.set_position();
            },
            Command::Down => {
                self.last = command;
                if self.dojump {
                    let mut new_pc = self.pc + 1;
                    for i in new_pc..self.program.len() {
                        match self.program[i] {
                            Command::Nil => {
                                new_pc = i- 1;
                                break;
                            }
                            Command::Line => {
                                new_pc = i;
                                break;
                            },
                            _ => (),
                        }
                    }
                    self.pc = new_pc;
                }
                if self.int == 1 {
                    self.dojump = true;
                }
            },
            Command::Up => {
                self.last = command;
                if self.dojump {
                    let mut new_pc = self.pc + 1;
                    let mut ignore_one = 1;
                    for i in (0..new_pc).rev() {
                        match self.program[i] {
                            Command::Line => {
                                new_pc = i;
                                //break;
                                if ignore_one == 0 {
                                    break;
                                }
                                ignore_one = ignore_one - 1;
                            },
                            _ => (),
                        }
                    }
                    self.pc = new_pc;
                }
                if self.int == 1 {
                    self.dojump = true;
                }
            },
            Command::Next(next) => {
                self.last = command;
                //TODO
                self.dojump = true;
            },
            Command::Prev(prev) => {
                self.last = command;
                //TODO
                self.dojump = true;
            },
            Command::Con => {
                self.dojump = match self.value {
                    Type::I(int) => self.int == int as u64,
                    _ => false,
                };
                self.int = 1;
            },
            Command::NCon => {
                self.dojump = match self.value {
                    Type::I(int) => self.int != int as u64,
                    _ => true,
                };
                self.int = 1;
            },
            Command::Int(int) => { self.int = int; },
            Command::VInt => {
                match self.value {
                    Type::I(int) => { self.int = int as u64; },
                    _ => { panic!("value not an integer"); },
                }
            },
            Command::Rep => {
                let com = self.last.clone();
                self.interpret(com);
            },
            Command::Grp(group) => {
                self.last = command;
                //TODO
            },
            _ => (),
        }
    }
}

pub fn run(program: Ast) -> usize {
    let mut interpreter = Interpreter::new(program);
    interpreter.run()
}

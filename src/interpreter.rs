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

    fn get_value(&mut self) {
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
            self.value = Type::I(0);
        }
    }

    fn set_value(&mut self) {
        if self.values.len() < self.pointer + 1 {
            self.values.resize(self.pointer + 1, Vec::new());
            let mut vec = vec![Type::I(0); self.index + 1];
            vec[self.index] = self.value.clone();
            self.values[self.pointer] = vec;
            return;
        }

        if self.values[self.pointer].len() < self.index + 1 {
            self.values[self.pointer].resize(self.index + 1, Type::I(0));
        }

        self.values[self.pointer][self.index] = self.value.clone();
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
            Command::Out => {
                self.last = command;
                print!("{}", self.value);
            },
            Command::OutL => {
                self.last = command;
                println!("{}", self.value);
            },
            Command::In => {
                //TODO
            },
            Command::Put => {
                self.last = command;
                self.set_value();
            },
            Command::Yank => {
                self.last = command;
                self.get_value();
            },
            Command::Ins(val) => {
                self.last = command;
                self.value = val.clone();
                self.set_value();
            },
            Command::Incr => {
                self.last = command;
                self.get_value();
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v + 1);
                        self.set_value();
                    }
                    _ => (),
                }
            },
            Command::Decr => {
                self.last = command;
                self.get_value();
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v - 1);
                        self.set_value();
                    }
                    _ => (),
                }
            },
            Command::Mark(mark) => {
                match mark {
                    'a' ... 'z' => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = 0;
                    },
                    _ => { panic!("mark must be a to z"); },
                }
            },
            Command::Ind(mark) => {
                match mark {
                    'a' ... 'z' => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = self.int as usize;
                    },
                    _ => { panic!("mark must be a to z"); },
                }
                self.int = 1;
            },
            Command::NMar => {
                self.last = command;
                self.pointer = self.pointer + 1;
                if self.pointer > 26 {
                    self.pointer = 0;
                }
                self.index = 0;
            },
            Command::PMar => {
                self.last = command;
                if self.pointer == 0 {
                    self.pointer = 27;
                }
                self.pointer = self.pointer - 1;
                self.index = 0;
            },
            Command::NInd => {
                self.last = command;
                self.index = self.index + 1;
            },
            Command::PInd => {
                self.last = command;
                if self.index ==0 {
                    panic!("Can't decrement index, already 0");
                }
                self.index = self.index - 1;
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
            Command::BLine => {
                self.last = command;
                if self.dojump {
                    let mut new_pc = self.pc + 1;
                    for i in (0..new_pc).rev() {
                        match self.program[i] {
                            Command::Line => {
                                new_pc = i;
                                break;
                            }
                            _ => (),
                        }
                    }
                    self.pc = new_pc;
                }
                self.dojump = true;
            }
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
                self.do_group(group);
            },
            _ => (),
        }
    }

    fn do_group(&mut self, group: Ast) {
        let temp_int = self.int;
        self.int = 1;
        let mut pc = 0;
        loop {
            if pc > group.len() - 1 {
                break;
            }

            let command = group[pc].clone();

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
            pc = pc + 1;
        }
        self.int = temp_int;
    }
}

pub fn run(program: Ast) -> usize {
    let mut interpreter = Interpreter::new(program);
    interpreter.run()
}

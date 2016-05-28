use ast::Ast;
use commands::{Command, Type};

use std::io;

pub struct Interpreter {
    values: Vec<Vec<Type>>,
    pointer: usize,
    index: usize,
    value: Type,
    int: u64,
    pc: usize,
    last: usize,
    dojump: bool,
    rep: bool,

    program: Ast,
}

impl Interpreter {

    pub fn new(program: Ast) -> Interpreter {
        Interpreter {
            pointer: 0,
            index: 0,
            int: 1,
            pc: 0,
            last: 0,
            values: Vec::new(),
            value: Type::I(0),
            dojump: true,
            rep: false,
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
                        self.interpret(&command);
                        self.int = self.int - 1;
                    }
                    self.int = 1;
                },
                command => {
                    self.interpret(&command);
                },
            }
            self.pc = self.pc + 1;
        }
        0
    }

    fn interpret(&mut self, command: &Command) {
        match command {
            &Command::Out => {
                if !self.rep {
                    self.last = self.pc;
                }
                print!("{}", self.value);
            },
            &Command::OutL => {
                if !self.rep {
                    self.last = self.pc;
                }
                println!("{}", self.value);
            },
            &Command::In => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        input = input.trim().to_owned();
                        let val = match input.parse::<i64>() {
                            Ok(n) => Type::I(n),
                            Err(_) => {
                                match input.len() {
                                    1 => Type::C(input.pop().unwrap()),
                                    _ => Type::S(input),
                                }
                            },
                        };
                        self.value = val.clone();
                    },
                    _ => (),
                }
            },
            &Command::Put => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.set_value();
            },
            &Command::Yank => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value();
            },
            &Command::Ins(ref val) => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.value = val.clone();
                self.set_value();
            },
            &Command::Incr => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value();
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v + 1);
                        self.set_value();
                    }
                    _ => (),
                }
            },
            &Command::Decr => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value();
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v - 1);
                        self.set_value();
                    }
                    _ => (),
                }
            },
            &Command::Plus => {
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v + self.int as i64);
                    },
                    _ => (),
                }
                self.int = 1;
            },
            &Command::Minus => {
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v - self.int as i64);
                    },
                    _ => (),
                }
                self.int = 1;
            },
            &Command::Times => {
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v * self.int as i64);
                    },
                    _ => (),
                }
                self.int = 1;
            },
            &Command::Divide => {
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v / self.int as i64);
                    },
                    _ => (),
                }
                self.int = 1;
            },
            &Command::Mark(ref mark) => {
                match mark {
                    &'a' ... 'z' => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = 0;
                    },
                    _ => { panic!("mark must be a to z"); },
                }
            },
            &Command::Ind(ref mark) => {
                match mark {
                    &'a' ... 'z' => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = self.int as usize;
                    },
                    _ => { panic!("mark must be a to z"); },
                }
                self.int = 1;
            },
            &Command::NMar => {
                self.pointer = self.pointer + 1;
                if self.pointer > 26 {
                    self.pointer = 0;
                }
                self.index = 0;
            },
            &Command::PMar => {
                if self.pointer == 0 {
                    self.pointer = 27;
                }
                self.pointer = self.pointer - 1;
                self.index = 0;
            },
            &Command::NInd => {
                self.index = self.index + 1;
            },
            &Command::PInd => {
                if self.index ==0 {
                    panic!("Can't decrement index, already 0");
                }
                self.index = self.index - 1;
            },
            &Command::Down => {
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
            &Command::Up => {
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
            &Command::BLine => {
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
            &Command::Next(ref next) => {
                //TODO
                self.dojump = true;
            },
            &Command::Prev(ref prev) => {
                //TODO
                self.dojump = true;
            },
            &Command::Con => {
                self.dojump = match self.value {
                    Type::I(int) => self.int == int as u64,
                    _ => false,
                };
                self.int = 1;
            },
            &Command::NCon => {
                self.dojump = match self.value {
                    Type::I(int) => self.int != int as u64,
                    _ => true,
                };
                self.int = 1;
            },
            &Command::LCon => {
                self.dojump = match self.value {
                    Type::I(int) => self.int < int as u64,
                    _ => false,
                };
                self.int = 1;
            },
            &Command::GCon => {
                self.dojump = match self.value {
                    Type::I(int) => self.int > int as u64,
                    _ => false,
                };
                self.int = 1;
            },
            &Command::Int(ref int) => { self.int = int.clone(); },
            &Command::VInt => {
                match self.value {
                    Type::I(int) => { self.int = int as u64; },
                    _ => { panic!("value not an integer"); },
                }
            },
            &Command::Grp(ref group) => {
                self.do_group(group);
            },
            &Command::Rep => {
                self.rep = true;
                let command = self.program[self.last].clone();
                self.interpret(&command);
                self.rep = false;
            }
            _ => (),
        }
    }

    fn do_group(&mut self, group: &Ast) {
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
                        self.interpret(&command);
                        self.int = self.int - 1;
                    }
                    self.int = 1;
                },
                command => {
                    self.interpret(&command);
                },
            }
            pc = pc + 1;
        }
        self.int = temp_int;
    }
}

pub fn run(program: Ast) -> usize {
    let mut interpreter = Interpreter::new(program);
    interpreter.run();
    //println!("{:?}", interpreter.values);
    0
}

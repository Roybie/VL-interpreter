use ast::Ast;
use commands::{Command, Type};
use std::io::Write;

use std::io;

pub struct Interpreter<'a, W: Write> {
    values: [Vec<Type>; 26],
    pointer: usize,
    index: usize,
    value: Type,
    int: Type,
    pc: usize,
    last: usize,
    dojump: bool,
    rep: bool,
    program: Ast,
    writer: &'a mut W
}

impl <'a, W: Write> Interpreter <'a, W>{

    pub fn new(program: Ast, writer: &'a mut W) -> Interpreter <'a, W> {
        Interpreter {
            pointer: 0,
            index: 0,
            int: Type::I(1),
            pc: 0,
            last: 0,
            values: [vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!(),vec!()],
            value: Type::I(0),
            dojump: true,
            rep: false,
            program,
            writer,
        }
    }

    fn get_value(&mut self, int: bool) {
        match self.values[self.pointer].get(self.index) {
            Some(typ) => {
                if int {
                    self.int = typ.clone();
                } else {
                    self.value = typ.clone();
                }
            },
            None => {
                if int {
                    self.int = Type::I(0);
                } else {
                    self.value = Type::I(0);
                }
            },
        }
    }

    fn set_value(&mut self, int: bool) {
        if self.values[self.pointer].len() < self.index + 1 {
            self.values[self.pointer].resize(self.index + 1, Type::I(0));
        }

        if int {
            self.values[self.pointer][self.index] = self.int.clone();
        } else {
            self.values[self.pointer][self.index] = self.value.clone();
        }
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
                    let mut loops = match self.int {
                        Type::I(int) => int,
                        _ => 1,
                    };
                    while loops > 0 {
                        if loops == 1 {
                            self.int = Type::I(1);
                        }
                        self.interpret(&command);
                        loops = loops - 1;
                    }
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
            &Command::VOut => {
                if !self.rep {
                    self.last = self.pc;
                }
                let _ = write!(self.writer, "{}", self.value);
            },
            &Command::VOutL => {
                if !self.rep {
                    self.last = self.pc;
                }
                let _ = writeln!(self.writer, "{}", self.value);
            },
            &Command::IOut => {
                if !self.rep {
                    self.last = self.pc;
                }
                let _ = write!(self.writer, "{}", self.int);
            },
            &Command::IOutL => {
                if !self.rep {
                    self.last = self.pc;
                }
                let _ = writeln!(self.writer, "{}", self.int);
            },
            &Command::In => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        input = input.trim().to_owned();
                        let val = match input.parse::<i64>() {
                            Ok(n) => Type::I(n),
                            Err(_) => Type::S(input),
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
                self.set_value(false);
            },
            &Command::PutI => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.set_value(true);
            },
            &Command::Yank => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value(false);
            },
            &Command::YankI => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value(true);
            },
            &Command::InsV(ref val) => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.value = val.clone();
                self.set_value(false);
            },
            &Command::InsI(ref val) => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.int = val.clone();
                self.set_value(true);
            },
            &Command::Incr => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value(false);
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v + 1);
                        self.set_value(false);
                    }
                    _ => (),
                }
            },
            &Command::Decr => {
                if !self.rep {
                    self.last = self.pc;
                }
                self.get_value(false);
                match self.value {
                    Type::I(v) => {
                        self.value = Type::I(v - 1);
                        self.set_value(false);
                    }
                    _ => (),
                }
            },
            &Command::Conv => {
                self.value = match self.value {
                    Type::I(i) => {
                        Type::S(i.to_string())
                    },
                    Type::S(ref s) => {
                        match s.parse::<i64>() {
                            Ok(n) => Type::I(n),
                            Err(_) => panic!("value can't be converted to int"),
                        }
                    },
                }
            },
            &Command::SLen => {
                self.int = match self.value {
                    Type::S(ref s) => Type::I(s.len() as i64),
                    _ => panic!("Can't get length of int"),
                };
            },
            &Command::Plus => {
                self.value = match self.value {
                    Type::I(i) => {
                        let int = self.int.get_int();
                        self.int = Type::I(1);
                        Type::I(i + int)
                    },
                    Type::S(ref s) => {
                        let int = self.int.get_str();
                        Type::S(s.to_owned() + &int)
                    },
                }
            },
            &Command::Minus => {
                self.value = match self.value {
                    Type::I(i) => {
                        let int = self.int.get_int();
                        self.int = Type::I(1);
                        Type::I(i - int)
                    },
                    Type::S(ref s) => {
                        let int = self.int.get_int();
                        let (first, last) = s.split_at(int as usize);
                        self.int = Type::S(last.to_owned());
                        Type::S(first.to_owned())
                    },
                }
            },
            &Command::Times => {
                self.value = match self.value {
                    Type::I(i) => {
                        let int = self.int.get_int();
                        self.int = Type::I(1);
                        Type::I(i * int)
                    },
                    Type::S(_) => {
                        let spl = match self.int {
                            Type::I(_) => "".to_owned(),
                            Type::S(ref s) => s.to_owned(),
                        };
                        Type::S(self.values[self.pointer].iter().map(|s| s.get_str()).collect::<Vec<_>>().join(&spl))
                    },
                };
            },
            &Command::Divide => {
                match self.value {
                    Type::I(i) => {
                        let int = self.int.get_int();
                        self.value = Type::I(i / int);
                        self.int = Type::I(i % int);
                    },
                    Type::S(ref s) => {
                        let spl = match self.int {
                            Type::I(_) => "".to_owned(),
                            Type::S(ref s) => s.to_owned(),
                        };
                        self.values[self.pointer] = s.split(&spl).filter(|s| s.len()>0).map(|s| Type::S(s.to_owned())).collect();
                    },
                }
            },
            &Command::Mark(ref mark) => {
                match mark {
                    &('a'..='z') => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = 0;
                    },
                    _ => { panic!("mark must be a to z"); },
                }
            },
            &Command::Ind(ref mark) => {
                match mark {
                    &('a'..='z') => {
                        self.pointer = mark.to_digit(36).unwrap() as usize - 10;
                        self.index = self.int.get_int() as usize;
                    },
                    _ => { panic!("mark must be a to z"); },
                }
                self.int = Type::I(1);
            },
            &Command::NMar => {
                self.pointer = self.pointer + 1;
                if self.pointer > 25 {
                    self.pointer = 0;
                }
                self.index = 0;
            },
            &Command::PMar => {
                if self.pointer == 0 {
                    self.pointer = 26;
                }
                self.pointer = self.pointer - 1;
                self.index = 0;
            },
            &Command::NInd => {
                self.index = self.index + 1;
            },
            &Command::PInd => {
                if self.index == 0 {
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
                if self.int.get_int() == 1 {
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
                if self.int.get_int() == 1 {
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
                let mut found = false;
                if self.dojump {
                    let mut new_pc = self.pc + 1;
                    for i in new_pc..self.program.len() {
                        if self.program[i].discriminant() == (**next).discriminant() {
                            new_pc = i;
                            found = true;
                            break;
                        }
                    }
                    if found {
                        self.pc = new_pc;
                    }
                }
                if self.int.get_int() == 1 {
                    self.dojump = true;
                    if found {
                        self.pc = self.pc - 1;
                    }
                }
            },
            &Command::Prev(ref prev) => {
                let mut found = false;
                if self.dojump {
                    let mut new_pc = self.pc;
                    for i in (0..new_pc).rev() {
                        if self.program[i].discriminant() == (**prev).discriminant() {
                            new_pc = i;
                            found = true;
                            break;
                        }
                    }
                    self.pc = new_pc;
                }
                if self.int.get_int() == 1 {
                    self.dojump = true;
                    if found {
                        self.pc = self.pc - 1;
                    }
                }
            },
            &Command::Con => {
                self.dojump = self.value == self.int;
                self.int = Type::I(1);
            },
            &Command::NCon => {
                self.dojump = self.value != self.int;
                self.int = Type::I(1);
            },
            &Command::LCon => {
                self.dojump = self.value > self.int;
                self.int = Type::I(1);
            },
            &Command::GCon => {
                self.dojump = self.value < self.int;
                self.int = Type::I(1);
            },
            &Command::Int(ref int) => { self.int = Type::I(int.clone() as i64); },
            &Command::V2I => {
                self.int = self.value.clone();
            },
            &Command::I2V => {
                self.value = self.int.clone();
            }
            &Command::Grp(ref group) => {
                self.do_group(group);
            },
            &Command::Rep => {
                self.rep = true;
                if self.last > 0 {
                    let command = self.program[self.last].clone();
                    self.interpret(&command);
                }
                self.rep = false;
            }
            _ => (),
        }
    }

    fn do_group(&mut self, group: &Ast) {
        let temp_last = self.last;
        self.last = 0;
        let temp_int = self.int.clone();
        self.int = Type::I(1);
        let temp_pc = self.pc;
        self.pc = 0;
        let temp_program = self.program.clone();
        self.program = group.clone();
        loop {
            if self.pc > group.len() - 1 {
                break;
            }

            let command = group[self.pc].clone();

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
                    let mut loops = match self.int {
                        Type::I(int) => int,
                        _ => 1,
                    };
                    while loops > 0 {
                        if loops == 1 {
                            self.int = Type::I(1);
                        }
                        self.interpret(&command);
                        loops = loops - 1;
                    }
                },
                command => {
                    self.interpret(&command);
                },
            }
            self.pc = self.pc + 1;
        }
        self.last = temp_last;
        self.int = temp_int;
        self.pc = temp_pc;
        self.program = temp_program;
    }
}

pub fn run<W: Write>(program: Ast, writer: &mut W) -> usize {
    let mut interpreter = Interpreter::new(program, writer);
    interpreter.run();
    0
}

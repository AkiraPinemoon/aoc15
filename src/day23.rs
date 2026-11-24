use std::{fs::File, io::Read, println};

pub fn part_one() {
    let mut file = File::open("input_23").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let instructions = contents
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect::<Vec<Instruction>>();

    let registers = Registers {
        a: 0,
        b: 0,
    };

    let mut machine = Machine {
        registers, 
        instructions,
        ip: 0,
    };

    machine.run();

    println!("Program complete. b = {}", machine.registers.b);
}

pub fn part_two() {
    let mut file = File::open("input_23").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let instructions = contents
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect::<Vec<Instruction>>();

    let registers = Registers {
        a: 1,
        b: 0,
    };

    let mut machine = Machine {
        registers, 
        instructions,
        ip: 0,
    };

    machine.run();

    println!("Program complete. b = {}", machine.registers.b);
}

struct Machine {
    registers: Registers,
    instructions: Vec<Instruction>,
    ip: usize,
}

impl Machine {
    pub fn step(&mut self) -> bool {
        let instruction = &self.instructions[self.ip];

        match instruction {
            Instruction::HLF(register) => {
                self.registers.set(&register, self.registers.get(&register) / 2);
                self.ip += 1;
            },
            Instruction::TPL(register) => {
                self.registers.set(&register, self.registers.get(&register) * 3);
                self.ip += 1;
            },
            Instruction::INC(register) => {
                self.registers.set(&register, self.registers.get(&register) + 1);
                self.ip += 1;
            },
            Instruction::JMP(offset) => {
                self.ip = (self.ip as isize + offset) as usize;
            },
            Instruction::JIE(register, offset) => {
                if self.registers.get(&register) % 2 == 0 {
                    self.ip = (self.ip as isize + offset) as usize;
                } else {
                    self.ip += 1;
                }
            },
            Instruction::JIO(register, offset) => {
                if self.registers.get(&register) == 1 {
                    self.ip = (self.ip as isize + offset) as usize;
                } else {
                    self.ip += 1;
                }
            },
        }

        // return false after programm end
        self.ip < self.instructions.len()
    }

    pub fn run(&mut self) {
        while self.step() {}
    }
}

enum Register {
    A,
    B,
}

impl Register {
    pub fn parse(text: &str) -> Result<Self, ()> {
        match text {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            register => {
                println!("invalid register: {}", register);
                Err(())
            }
        }
    }
}

struct Registers {
    pub a: usize,
    pub b: usize,
}

impl Registers {
    pub fn get(&self, reg: &Register) -> usize {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
        }
    }
    pub fn set(&mut self, reg: &Register, value: usize) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }
}

enum Instruction {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(isize),
    JIE(Register, isize),
    JIO(Register, isize),
}

impl Instruction {
    pub fn parse(text: &str) -> Result<Self, ()> {
        let mut tokens = text.split_whitespace();

        match tokens.next().ok_or(())? {
            "hlf" => {
                let reg = Register::parse(tokens.next().ok_or(())?)?;
                Ok(Self::HLF(reg))
            },
            "tpl" => {
                let reg = Register::parse(tokens.next().ok_or(())?)?;
                Ok(Self::TPL(reg))
            },
            "inc" => {
                let reg = Register::parse(tokens.next().ok_or(())?)?;
                Ok(Self::INC(reg))
            },
            "jmp" => {
                let offset = tokens.next().ok_or(())?.parse().map_err(|_| ())?;
                Ok(Self::JMP(offset))
            },
            "jie" => {
                let reg = Register::parse(tokens.next().ok_or(())?.strip_suffix(",").ok_or(())?)?;
                let offset = tokens.next().ok_or(())?.parse().map_err(|_| ())?;
                Ok(Self::JIE(reg, offset))
            },
            "jio" => {
                let reg = Register::parse(tokens.next().ok_or(())?.strip_suffix(",").ok_or(())?)?;
                let offset = tokens.next().ok_or(())?.parse().map_err(|_| ())?;
                Ok(Self::JIO(reg, offset))
            },
            opcode => {
                println!("invalid opcode: {}", opcode);
                Err(())
            }
        }
    }
}

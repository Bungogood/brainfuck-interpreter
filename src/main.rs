struct Interpreter {
    loop_stack: Vec<usize>,
    pc: usize,
    memory: [u8; 30000],
    pointer: usize,
    source: Vec<char>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            source: Vec::new(),
            loop_stack: Vec::new(),
            pc: 0,
            pointer: 0,
            memory: [0; 30000],
        }
    }
    
    fn execute(&mut self, source: &str) {
        self.source.append(&mut source.chars().collect());

        while self.pc < self.source.len() {
            match self.source[self.pc] {
                '+' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                '-' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                '<' => self.shift(-1),
                '>' => self.shift(1),
                '.' => self.output(),
                ',' => self.input(),
                '[' => self.loop_start(),
                ']' => self.loop_end(),
                _ => (),
            }
            self.pc += 1;
        }
    }

    fn shift(&mut self, offset: i32) {
        let new_pointer = self.pointer as i32 + offset;
        if new_pointer < 0 || new_pointer >= self.memory.len() as i32 {
            panic!("Pointer out of bounds");
        }
        self.pointer = new_pointer as usize;
    }

    fn output(&self) {
        print!("{}", self.memory[self.pointer] as char);
    }

    fn input(&mut self) {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        self.memory[self.pointer] = buffer.chars().next().unwrap() as u8;
    }

    fn loop_start(&mut self) {
        if self.memory[self.pointer] == 0 {
            let mut depth = 1;
            while depth > 0 {
                self.pc += 1;
                match self.source[self.pc] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => (),
                }
            }
        } else {
            self.loop_stack.push(self.pc);
        }
    }

    fn loop_end(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.loop_stack.pop();
        } else {
            self.pc = *self.loop_stack.last().unwrap();
        }
    }
}

fn main() {
    Interpreter::new().execute("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."); // Hello World
}

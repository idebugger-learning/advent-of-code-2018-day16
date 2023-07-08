pub type Value = usize;
pub type Registers = [Value; 4];
pub struct CPU {
    pub registers: Registers,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0, 0, 0, 0]
        }
    }

    pub fn do_opcode(&mut self, opcode: Opcode, a: Value, b: Value, c: Value) {
        match opcode {
            Opcode::Addr => self.do_addr(a, b, c),
            Opcode::Addi => self.do_addi(a, b, c),
            Opcode::Mulr => self.do_mulr(a, b, c),
            Opcode::Muli => self.do_muli(a, b, c),
            Opcode::Banr => self.do_banr(a, b, c),
            Opcode::Bani => self.do_bani(a, b, c),
            Opcode::Borr => self.do_borr(a, b, c),
            Opcode::Bori => self.do_bori(a, b, c),
            Opcode::Setr => self.do_setr(a, c),
            Opcode::Seti => self.do_seti(a, c),
            Opcode::Gtir => self.do_gtir(a, b, c),
            Opcode::Gtri => self.do_gtri(a, b, c),
            Opcode::Gtrr => self.do_gtrr(a, b, c),
            Opcode::Eqir => self.do_eqir(a, b, c),
            Opcode::Eqri => self.do_eqri(a, b, c),
            Opcode::Eqrr => self.do_eqrr(a, b, c),
        }
    }

    fn do_addr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] + self.registers[reg_b];
    }

    fn do_addi(&mut self, reg_a: usize, val_b: Value, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] + val_b;
    }

    fn do_mulr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] * self.registers[reg_b];
    }

    fn do_muli(&mut self, reg_a: usize, val_b: Value, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] * val_b;
    }

    fn do_banr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] & self.registers[reg_b];
    }

    fn do_bani(&mut self, reg_a: usize, val_b: Value, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] & val_b;
    }

    fn do_borr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] | self.registers[reg_b];
    }

    fn do_bori(&mut self, reg_a: usize, val_b: Value, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] | val_b;
    }

    fn do_setr(&mut self, reg_a: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a];
    }

    fn do_seti(&mut self, val_a: Value, reg_c: usize) {
        self.registers[reg_c] = val_a;
    }

    fn do_gtir(&mut self, val_a: Value, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if val_a > self.registers[reg_b] { 1 } else { 0 };
    }

    fn do_gtri(&mut self, reg_a: usize, val_b: Value, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] > val_b { 1 } else { 0 };
    }

    fn do_gtrr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] > self.registers[reg_b] { 1 } else { 0 };
    }

    fn do_eqir(&mut self, val_a: Value, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if val_a == self.registers[reg_b] { 1 } else { 0 };
    }

    fn do_eqri(&mut self, reg_a: usize, val_b: Value, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] == val_b { 1 } else { 0 };
    }

    fn do_eqrr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] == self.registers[reg_b] { 1 } else { 0 };
    }
}

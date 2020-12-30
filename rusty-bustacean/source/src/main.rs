use std::io::Read;

#[derive(Clone, Copy)]
enum VMReg {
    A,
    B,
    C,
    D,
    IP,
    SP
}

#[derive(Clone, Copy)]
enum VMOp {
    PushI(usize),
    PushR(VMReg),
    Pop(VMReg),
    Add(VMReg, VMReg),
    Sub(VMReg, VMReg),
    Mul(VMReg, VMReg),
    Div(VMReg, VMReg),
    Xor(VMReg, VMReg),
    And(VMReg, VMReg),
    Or(VMReg, VMReg),
    Shl(VMReg, VMReg),
    Shr(VMReg, VMReg),
    Inp(VMReg),
    Eq(VMReg, VMReg),
    Gt(VMReg, VMReg),
    Lt(VMReg, VMReg),
    Jmp(usize),
    JmpRel(usize),
    Call(usize),
    Ret,
    Print(VMReg),
    Halt
}

struct VM {
    pub a: usize,
    b: usize,
    c: usize,
    d: usize,
    ip: usize,
    sp: usize,
    is_halted: bool,
    stack: [usize; 0xff],
    code: Vec<VMOp>
}

impl VM {
    pub fn new(code: Vec<VMOp>) -> VM {
        VM {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            ip: 0,
            sp: 0,
            is_halted: true,
            stack: [0; 0xff],
            code
        }
    }

    pub fn run(&mut self) {
        self.is_halted = false;
        while !self.is_halted {
            let inst = self.code[self.ip];
            // println!("{:?}", inst);
            // self.debug();
            self.exec_inst(&inst);
            self.ip += 1;
        }
    }

    fn exec_inst(&mut self, inst: &VMOp)
    {
        match inst {
            VMOp::PushI(imm) => { self.stack[self.sp] = *imm; self.sp += 1; },
            VMOp::PushR(reg) => { self.exec_inst(&VMOp::PushI(self.get_reg(reg))) },
            VMOp::Pop(reg) => { self.sp -= 1; self.set_reg(reg, self.stack[self.sp]); }
            VMOp::Add(left, right) => { self.set_reg(left, self.get_reg(left) + self.get_reg(right)) },
            VMOp::Sub(left, right) => { self.set_reg(left, self.get_reg(left) - self.get_reg(right)) },
            VMOp::Mul(left, right) => { self.set_reg(left, self.get_reg(left) * self.get_reg(right)) },
            VMOp::Div(left, right) => { self.set_reg(left, self.get_reg(left) / self.get_reg(right)) },
            VMOp::Xor(left, right) => { self.set_reg(left, self.get_reg(left) ^ self.get_reg(right)) },
            VMOp::And(left, right) => { self.set_reg(left, self.get_reg(left) & self.get_reg(right)) },
            VMOp::Or(left, right) => { self.set_reg(left, self.get_reg(left) | self.get_reg(right)) },
            VMOp::Shl(left, right) => { self.set_reg(left, self.get_reg(left) << self.get_reg(right)) },
            VMOp::Shr(left, right) => { self.set_reg(left, self.get_reg(left) >> self.get_reg(right)) },
            VMOp::Inp(reg) => { 
                let in_char = std::io::stdin()
                    .bytes() 
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as usize)
                    .unwrap();
                if in_char == 0xd { 
                    self.exec_inst(&VMOp::Inp(*reg)) 
                }
                else {
                    self.set_reg(reg, in_char);
                }
            },
            VMOp::Eq(left, right) => { if self.get_reg(left) != self.get_reg(right) {self.ip += 1} },
            VMOp::Gt(left, right) => { if self.get_reg(left) <= self.get_reg(right) {self.ip += 1} },
            VMOp::Lt(left, right) => { if self.get_reg(left) >= self.get_reg(right) {self.ip += 1} },
            VMOp::Jmp(addr) => { self.ip = addr - 1 },
            VMOp::JmpRel(off) => { self.ip += off - 1 },
            VMOp::Call(addr) => { self.exec_inst(&VMOp::PushI(self.ip + 1)); self.exec_inst(&VMOp::Jmp(*addr)); },
            VMOp::Ret => { self.sp -= 1; self.ip = self.stack[self.sp]; }
            VMOp::Print(reg) => { print!("{}", self.get_reg(reg) as u8 as char) },
            VMOp::Halt => self.is_halted = true,
        }
    }

    fn get_reg(&self, reg: &VMReg) -> usize {
        match reg {
            VMReg::A => self.a,
            VMReg::B => self.b,
            VMReg::C => self.c,
            VMReg::D => self.d,
            VMReg::IP => self.ip,
            VMReg::SP => self.sp,
        }
    }

    fn set_reg(&mut self, reg: &VMReg, val: usize) {
        match reg {
            VMReg::A => self.a = val,
            VMReg::B => self.b = val,
            VMReg::C => self.c = val,
            VMReg::D => self.d = val,
            VMReg::IP => self.ip = val,
            VMReg::SP => self.sp = val,
        }
    }
}

fn main() {
    use VMOp::*;
    use VMReg::*;

    let mut vm = VM::new(
        vec![
            PushI(8),
            Pop(A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            PushI(0xaa551337),
            Pop(D),
            Xor(C, D),
            PushI(0xcc397250),
            Pop(A),
            Eq(A, C),
            Jmp(33),
            PushI('L' as usize),
            Pop(A),
            Print(A),
            PushI('o' as usize),
            Pop(A),
            Print(A),
            PushI('s' as usize),
            Pop(A),
            Print(A),
            PushI('e' as usize),
            Pop(A),
            Print(A),
            Halt,
            PushI(0x4444),
            Pop(A),
            PushI(0x3759),
            Pop(B),
            PushI(7),
            Pop(B),
            Mul(A, B),
            Or(A, B),
            PushI(3),
            Pop(B),
            Shr(A, B),
            Inp(B),
            PushI(8),
            Pop(C),
            Shl(B, C),
            Inp(D),
            Or(B, D),
            PushI(0x40cc),
            Pop(C),
            Xor(B, C),
            Eq(A, B),
            JmpRel(2),
            Jmp(20),
            PushI(8),
            Pop(A),
            PushI(0),
            Inp(B),
            Pop(C),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            PushR(C),
            Pop(D),
            PushI(0x561245),
            Pop(B),
            Mul(B, IP),
            Shl(B, A),
            Or(B, A),
            Xor(B, IP),
            PushI(0x233),
            Pop(A),
            Mul(B, A),
            Sub(C, B),
            Xor(C, D),
            PushI(13636),
            PushI(5492355013),
            Pop(A),
            Pop(B),
            Mul(A, B),
            Eq(A, C),
            JmpRel(2),
            Jmp(20),
            Add(C, D),
            Mul(C, B),
            PushR(C),
            Pop(D),
            PushI(8),
            Pop(A),
            PushI(0),
            Inp(B),
            Pop(C),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Xor(D, C),
            PushR(D),
            PushI(261534559663319),
            Pop(B),
            PushR(B),
            Pop(D),
            PushI(20803),
            Pop(A),
            PushI(1),
            Pop(C),
            Eq(A, C),
            Jmp(148),
            Add(B, D),
            PushR(A),
            PushI(1),
            Pop(A),
            Add(C, A),
            Pop(A),
            Jmp(139),
            Pop(D),
            Eq(B, D),
            JmpRel(2),
            Jmp(20),
            PushI(8),
            Pop(A),
            PushI(0),
            Inp(B),
            Pop(C),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            Shl(C, A),
            Inp(B),
            Or(C, B),
            PushI(0b111111111111111111111111111111111111111),
            Pop(D),
            And(C, D),
            Add(D, C),
            PushI(0xf2656e6364),
            Pop(B),
            Eq(B, D),
            JmpRel(2),
            Jmp(20),
            PushI(0),
            Pop(C),
            Inp(B),
            Xor(C, B),
            Inp(B),
            Eq(C, B),
            JmpRel(2),
            Jmp(20),
            Inp(C),
            Eq(C, B),
            JmpRel(2),
            Jmp(20),
            PushI(0x2e),
            Inp(A),
            PushI(8),
            Pop(D),
            Shl(A, D),
            Add(A, C),
            PushI(0x7d2e),
            Pop(B),
            Eq(A, B),
            JmpRel(2),
            Jmp(20),
            PushI('W' as usize),
            Pop(A),
            Print(A),
            PushI('i' as usize),
            Pop(A),
            Print(A),
            PushI('n' as usize),
            Pop(A),
            Print(A),
            Halt
            // flag{whats_the_difference...}
        ]
    );

    vm.run();
}

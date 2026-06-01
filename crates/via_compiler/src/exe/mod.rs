use std::collections::HashMap;

use via_vm::{Executable, Immediate, Instruction, Operand};

use crate::{
    mir::{Block, Instruction as MirInstr, Mir, Operand as MirOperand, TempId, Term},
    sema::ConstValue,
};

use register::RegisterAlloc;

mod register;

pub struct ExeBuilder<'cx> {
    mir: &'cx Mir,
    reg_alloc: RegisterAlloc,
    temp_map: HashMap<TempId, Operand>,
}

const WORD_SIZE: usize = size_of::<Operand>();
const DWORD_SIZE: usize = size_of::<Immediate>();

impl<'cx> ExeBuilder<'cx> {
    pub fn new(mir: &'cx Mir) -> Self {
        Self {
            mir,
            reg_alloc: RegisterAlloc::new(16),
            temp_map: HashMap::new(),
        }
    }

    fn push(&self, exe: &mut Executable, instr: Instruction) {
        exe.instrs.push(instr);
    }

    fn alloc(&mut self) -> Operand {
        self.reg_alloc.alloc()
    }

    fn free(&mut self, exe: &mut Executable, r: Operand) {
        exe.instrs.push(Instruction::free1(r));
        self.reg_alloc.free(r);
    }

    fn free2(&mut self, exe: &mut Executable, ra: Operand, rb: Operand) {
        exe.instrs.push(Instruction::free2(ra, rb));
        self.reg_alloc.free(ra);
        self.reg_alloc.free(rb);
    }

    #[allow(unused)]
    fn free3(&mut self, exe: &mut Executable, ra: Operand, rb: Operand, rc: Operand) {
        exe.instrs.push(Instruction::free3(ra, rb, rc));
        self.reg_alloc.free(ra);
        self.reg_alloc.free(rb);
        self.reg_alloc.free(rc);
    }

    fn bind_temp(&mut self, temp: TempId, reg: Operand) {
        self.temp_map.insert(temp, reg);
    }

    fn write_back(&mut self, exe: &mut Executable, out: MirOperand, reg: Operand) {
        match out {
            MirOperand::Temp(temp) => self.bind_temp(temp, reg),
            MirOperand::Local(local) => {
                self.push(exe, Instruction::setlocal(reg, local.inner()));
                self.free(exe, reg);
            }
        }
    }

    fn lower_operand(&mut self, exe: &mut Executable, op: MirOperand) -> (Operand, bool) {
        match op {
            MirOperand::Temp(temp) => {
                let reg = self.temp_map[&temp];
                (reg, true)
            }
            MirOperand::Local(local) => {
                let reg = self.alloc();
                self.push(exe, Instruction::getlocal(reg, local.inner()));
                (reg, false)
            }
        }
    }

    fn lower_instr(&mut self, exe: &mut Executable, instr: &MirInstr) {
        match instr {
            MirInstr::Const { value, out } => {
                let dst = self.alloc();

                match value {
                    ConstValue::Unit => self.push(exe, Instruction::loadnone(dst)),
                    ConstValue::Bool(value) => self.push(
                        exe,
                        if *value {
                            Instruction::loadtrue(dst)
                        } else {
                            Instruction::loadfalse(dst)
                        },
                    ),
                    ConstValue::Int(int) => {
                        let imm = *int;
                        if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                            self.push(exe, Instruction::loadi32(dst, imm as Immediate));
                        } else {
                            let bits = imm as u64;

                            let lo = bits as Immediate;
                            let mid = (bits >> DWORD_SIZE) as Operand;
                            let hi = (bits >> (DWORD_SIZE + WORD_SIZE)) as Operand;

                            self.push(exe, Instruction::loadi64(dst, lo));
                            self.push(exe, Instruction::extraarg2(mid, hi));
                        }
                    }
                    _ => todo!(),
                }

                self.write_back(exe, *out, dst);
            }
            MirInstr::IAdd { lhs, rhs, out } => {
                let (l, l_free) = self.lower_operand(exe, *lhs);
                let (r, r_free) = self.lower_operand(exe, *rhs);

                let dst = self.alloc();

                self.push(exe, Instruction::iadd(dst, l, r));
                self.write_back(exe, *out, dst);

                if l_free && r_free {
                    self.free2(exe, l, r);
                } else if l_free {
                    self.free(exe, l);
                } else if r_free {
                    self.free(exe, r);
                }
            }
            MirInstr::Local { id, .. } => {
                let (reg, free) = self.lower_operand(exe, *id);
                self.push(exe, Instruction::push(reg));

                if free {
                    self.free(exe, reg);
                }
            }
        }
    }

    fn lower_term(&mut self, exe: &mut Executable, term: &Term) {
        match term {
            Term::Halt => self.push(exe, Instruction::halt()),
            _ => todo!(),
        }
    }

    fn lower_block(&mut self, exe: &mut Executable, block: &Block) {
        for instr in &block.instrs {
            self.lower_instr(exe, instr);
        }
        self.lower_term(exe, &block.term);
    }

    pub fn build(&mut self) -> Executable {
        let mut exe = Executable::new();

        for block in &self.mir.blocks {
            self.lower_block(&mut exe, block);
        }

        exe
    }
}

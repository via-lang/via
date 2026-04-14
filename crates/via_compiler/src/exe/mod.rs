use std::collections::HashMap;

use via_vm::{Executable, instr::Instr};

use crate::{
    mir::{Block, Instr as MirInstr, Mir, Operand, TempId, Term},
    sema::ConstValue,
};

use register::RegisterAlloc;

mod register;

type Reg = u16;

pub struct ExeBuilder<'cx> {
    mir: &'cx Mir,
    reg_alloc: RegisterAlloc,
    temp_map: HashMap<TempId, Reg>,
}

impl<'cx> ExeBuilder<'cx> {
    pub fn new(mir: &'cx Mir) -> Self {
        Self {
            mir,
            reg_alloc: RegisterAlloc::new(256),
            temp_map: HashMap::default(),
        }
    }

    fn push(&self, exe: &mut Executable, instr: Instr) {
        exe.instrs.push(instr);
    }

    fn alloc(&mut self) -> Reg {
        self.reg_alloc.alloc()
    }

    fn free(&mut self, exe: &mut Executable, r: Reg) {
        exe.instrs.push(Instr::free1(r));
        self.reg_alloc.free(r);
    }

    fn free2(&mut self, exe: &mut Executable, ra: Reg, rb: Reg) {
        exe.instrs.push(Instr::free2(ra, rb));
        self.reg_alloc.free(ra);
        self.reg_alloc.free(rb);
    }

    fn free3(&mut self, exe: &mut Executable, ra: Reg, rb: Reg, rc: Reg) {
        exe.instrs.push(Instr::free3(ra, rb, rc));
        self.reg_alloc.free(ra);
        self.reg_alloc.free(rb);
        self.reg_alloc.free(rc);
    }

    fn bind_temp(&mut self, temp: TempId, reg: Reg) {
        self.temp_map.insert(temp, reg);
    }

    fn write_back(&mut self, exe: &mut Executable, out: Operand, reg: Reg) {
        match out {
            Operand::Temp(temp) => self.bind_temp(temp, reg),
            Operand::Local(local) => {
                self.push(exe, Instr::setlocal(reg, local.inner()));
                self.free(exe, reg);
            }
        }
    }

    fn lower_operand(&mut self, exe: &mut Executable, op: Operand) -> (Reg, bool) {
        match op {
            Operand::Temp(temp) => {
                let reg = self.temp_map[&temp];
                (reg, true)
            }
            Operand::Local(local) => {
                let reg = self.alloc();
                self.push(exe, Instr::getlocal(reg, local.inner()));
                (reg, false)
            }
        }
    }

    fn lower_instr(&mut self, exe: &mut Executable, instr: &MirInstr) {
        match instr {
            MirInstr::Const { value, out } => {
                let dst = self.alloc();

                match value {
                    ConstValue::Unit => self.push(exe, Instr::loadnone(dst)),
                    ConstValue::Bool(value) => self.push(
                        exe,
                        if *value {
                            Instr::loadtrue(dst)
                        } else {
                            Instr::loadfalse(dst)
                        },
                    ),
                    ConstValue::Int(int) => {
                        let imm = *int;
                        if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                            self.push(exe, Instr::loadi32(dst, imm as u32));
                        } else {
                            let bits = imm as u64;

                            let lo = bits as u32;
                            let mid = (bits >> 32) as u16;
                            let hi = (bits >> 48) as u16;

                            self.push(exe, Instr::loadi64(dst, lo));
                            self.push(exe, Instr::extraarg2(mid, hi));
                        }
                    }
                    ConstValue::Float(val) => {
                        let bits = *val;
                        if (bits as f32 as f64) == bits {
                            self.push(exe, Instr::loadf32(dst, bits as u32));
                        } else {
                            let bits = val.to_bits();

                            let lo = bits as u32;
                            let mid = (bits >> 32) as u16;
                            let hi = (bits >> 48) as u16;

                            self.push(exe, Instr::loadf64(dst, lo));
                            self.push(exe, Instr::extraarg2(mid, hi));
                        }
                    }
                }

                self.write_back(exe, *out, dst);
            }
            MirInstr::IAdd { lhs, rhs, out } => {
                let (l, l_free) = self.lower_operand(exe, *lhs);
                let (r, r_free) = self.lower_operand(exe, *rhs);

                let dst = self.alloc();

                self.push(exe, Instr::iadd(dst, l, r));
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
                self.push(exe, Instr::push(reg));

                if free {
                    self.free(exe, reg);
                }
            }
        }
    }

    fn lower_term(&mut self, exe: &mut Executable, term: &Term) {
        match term {
            Term::Halt => self.push(exe, Instr::halt()),
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

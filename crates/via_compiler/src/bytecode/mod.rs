use std::collections::HashMap;

use via_vm::{Executable, Immediate, Instr, Operand};

use crate::mir::{Block, Instr as MirInstr, InstrBin, Mir, Operand as MirOperand, TempId, Term};

use register::RegisterAlloc;

mod register;

pub struct ExeBuilder<'cx> {
    mir: &'cx Mir,
    reg_alloc: RegisterAlloc,
    temp_map: HashMap<TempId, Operand>,
}

impl<'cx> ExeBuilder<'cx> {
    pub fn new(mir: &'cx Mir) -> Self {
        Self {
            mir,
            reg_alloc: RegisterAlloc::new(16),
            temp_map: HashMap::new(),
        }
    }

    fn push(&self, exe: &mut Executable, instr: Instr) {
        exe.instrs.push(instr);
    }

    fn alloc(&mut self) -> Operand {
        self.reg_alloc.alloc()
    }

    fn free(&mut self, exe: &mut Executable, r: Operand) {
        exe.instrs.push(Instr::FR1(r));
        self.reg_alloc.free(r);
    }

    fn free2(&mut self, exe: &mut Executable, ra: Operand, rb: Operand) {
        exe.instrs.push(Instr::FR2(ra, rb));
        self.reg_alloc.free(ra);
        self.reg_alloc.free(rb);
    }

    #[allow(unused)]
    fn free3(&mut self, exe: &mut Executable, ra: Operand, rb: Operand, rc: Operand) {
        exe.instrs.push(Instr::FR3(ra, rb, rc));
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
                self.push(exe, Instr::SETLOC(reg, local.inner()));
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
                self.push(exe, Instr::GETLOC(reg, local.inner()));
                (reg, false)
            }
        }
    }

    fn lower_instr(&mut self, exe: &mut Executable, instr: &MirInstr) {
        match instr {
            MirInstr::Const { value, out } => {
                let dst = self.alloc();

                match value {
                    ConstValue::Unit => self.push(exe, Instr::LDU(dst)),
                    ConstValue::Bool(value) => self.push(
                        exe,
                        if *value {
                            Instr::LDT(dst)
                        } else {
                            Instr::LDF(dst)
                        },
                    ),
                    ConstValue::Int(int) => {
                        let imm = *int;
                        if imm >= i16::MIN as i64 && imm <= i16::MAX as i64 {
                            self.push(exe, Instr::LDI16(dst, imm as Immediate));
                        } else if imm >= i32::MIN as i64 && imm <= i32::MAX as i64 {
                            let (base, ext) = Instr::encode_32(dst, imm as u32, false);
                            self.push(exe, base);
                            self.push(exe, ext);
                        } else {
                            let (base, ext1, ext2) = Instr::encode_64(dst, imm as u64, false);
                            self.push(exe, base);
                            self.push(exe, ext1);
                            self.push(exe, ext2);
                        }
                    }
                    ConstValue::Float(float) => {
                        let bits = float.to_bits();
                        let is_f32 = (*float as f32 as f64 - float).abs() < f64::EPSILON;

                        if is_f32 {
                            let (base, ext) =
                                Instr::encode_32(dst, (*float as f32).to_bits(), true);
                            self.push(exe, base);
                            self.push(exe, ext);
                        } else {
                            let (base, ext1, ext2) = Instr::encode_64(dst, bits, true);
                            self.push(exe, base);
                            self.push(exe, ext1);
                            self.push(exe, ext2);
                        }
                    }
                }

                self.write_back(exe, *out, dst);
            }
            MirInstr::IAdd(bin)
            | MirInstr::FAdd(bin)
            | MirInstr::IFAdd(bin)
            | MirInstr::ISub(bin)
            | MirInstr::FSub(bin)
            | MirInstr::IFSub(bin)
            | MirInstr::FISub(bin)
            | MirInstr::IMul(bin)
            | MirInstr::FMul(bin)
            | MirInstr::IFMul(bin)
            | MirInstr::IDiv(bin)
            | MirInstr::FDiv(bin)
            | MirInstr::IFDiv(bin)
            | MirInstr::FIDiv(bin)
            | MirInstr::IExp(bin)
            | MirInstr::FExp(bin)
            | MirInstr::IFExp(bin)
            | MirInstr::FIExp(bin)
            | MirInstr::IRem(bin)
            | MirInstr::FRem(bin) => {
                let InstrBin { lhs, rhs, out } = bin;

                let (l, l_free) = self.lower_operand(exe, *lhs);
                let (r, r_free) = self.lower_operand(exe, *rhs);

                let dst = self.alloc();
                let op = match instr {
                    MirInstr::IAdd(_) => Instr::IADD,
                    MirInstr::FAdd(_) => Instr::FADD,
                    MirInstr::IFAdd(_) => Instr::IFADD,
                    MirInstr::ISub(_) => Instr::ISUB,
                    MirInstr::FSub(_) => Instr::FSUB,
                    MirInstr::IFSub(_) => Instr::IFSUB,
                    MirInstr::FISub(_) => Instr::FISUB,
                    MirInstr::IMul(_) => Instr::IMUL,
                    MirInstr::FMul(_) => Instr::FMUL,
                    MirInstr::IFMul(_) => Instr::IFMUL,
                    MirInstr::IDiv(_) => Instr::IDIV,
                    MirInstr::FDiv(_) => Instr::FDIV,
                    MirInstr::IFDiv(_) => Instr::IFDIV,
                    MirInstr::FIDiv(_) => Instr::FIDIV,
                    MirInstr::IExp(_) => Instr::IEXP,
                    MirInstr::FExp(_) => Instr::FEXP,
                    MirInstr::IFExp(_) => Instr::IFEXP,
                    MirInstr::FIExp(_) => Instr::FIEXP,
                    MirInstr::IRem(_) => Instr::IREM,
                    MirInstr::FRem(_) => Instr::FREM,
                    _ => unreachable!(),
                }(dst, l, r);

                self.push(exe, op);
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
                self.push(exe, Instr::PUSH(reg));

                if free {
                    self.free(exe, reg);
                }
            }
        }
    }

    fn lower_term(&mut self, exe: &mut Executable, term: &Term) {
        match term {
            Term::Halt => self.push(exe, Instr::HLT()),
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

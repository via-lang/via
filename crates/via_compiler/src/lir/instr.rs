use derive_more::From;
use ordered_float::OrderedFloat;
use salsa::Update;

#[salsa::tracked(debug)]
pub struct Instr<'db> {
    pub kind: InstrKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum InstrKind {
    LoadConst {
        value: ConstValue,
        out: Operand,
    },
    PushLocal {
        input: Operand,
        out: Local,
    },
    Intrin {
        intrin: Intrinsic,
        input: Vec<Operand>,
        out: Operand,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ConstValue {
    Unit,
    Bool(bool),
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub enum Intrinsic {
    IAdd,
    FAdd,
    IFAdd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub enum Operand {
    Temp(Temp),
    Local(Local),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Update, From)]
pub struct Temp(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Update, From)]
pub struct Local(pub u32);

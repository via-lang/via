use super::{super::symbol::SymbolId, DefId};

#[derive(Debug)]
pub struct ModDef {
    pub sym: SymbolId,
    pub parent: Option<DefId>,
}

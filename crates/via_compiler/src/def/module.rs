use super::DefId;
use crate::symbol::SymbolId;

#[derive(Debug)]
pub struct ModDef {
    pub sym: SymbolId,
    pub parent: Option<DefId>,
}

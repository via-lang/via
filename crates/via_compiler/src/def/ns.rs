use super::DefId;
use crate::symbol::SymbolId;

#[derive(Debug)]
pub struct NsDef {
    pub sym: SymbolId,
    pub parent: Option<DefId>,
}

/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    Hir,
    block::BlockId,
    builder::IrBuilder,
    env::Env,
    error::Error,
    instr::{Instr, ValueId},
};
use crate::ast::place::Place;

pub(super) enum ReadKind {
    Copy,
    Move,
    Borrow,
}

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_place(
        &mut self,
        hir: &mut Hir,
        env: &mut Env,
        block: BlockId,
        place: &'a Place,
        out: ValueId,
        read_kind: ReadKind,
    ) {
        match place {
            Place::Symbol(symbol) => {
                let text = &symbol.symbol;
                let sym = self.symbols.intern(text);
                let Some(id) = env.lookup(sym) else {
                    return self.clinic.report(
                        Error::UndefinedSymbol {
                            span: symbol.span.to_miette_span(),
                            symbol: text.clone(),
                        }
                        .into(),
                    );
                };

                let value = ValueId::Local(id);

                self.push(
                    hir,
                    block,
                    match read_kind {
                        ReadKind::Copy => Instr::Copy { value, out },
                        ReadKind::Move => Instr::Move { value, out },
                        ReadKind::Borrow => todo!(),
                    },
                );
            }
            Place::Dynamic(dy) => {
                let text = self.source.get_span(&dy.field.span);
                let field = self.symbols.intern(text);

                let [value] = env.temp_id.bump::<1>();

                self.lower_expr(hir, env, block, self.ast.get(dy.expr), value, read_kind);
                self.push(hir, block, Instr::GetDynamic { value, field, out });
            }
            _ => todo!(),
        }
    }
}

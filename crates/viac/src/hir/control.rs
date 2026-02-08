/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    Hir, block::BlockId, builder::IrBuilder, env::Env, instr::TempId, place::ReadKind, term::Term,
};
use crate::{
    ast::{aux::Nodes, control::Control, stmt::StmtId},
    hir::{env::LoopEnv, error::Error},
    source::SourceSpan,
};

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_body(
        &mut self,
        hir: &mut Hir,
        env: &mut Env,
        block: BlockId,
        nodes: &'a Nodes<StmtId>,
    ) -> BlockId {
        let mut current = block;
        for stmt in &nodes.inner {
            current = self.lower_stmt(hir, env, current, self.ast.get(*stmt));
        }
        current
    }

    pub(super) fn lower_control(
        &mut self,
        hir: &mut Hir,
        env: &mut Env,
        block: BlockId,
        control: &'a Control,
    ) -> BlockId {
        let mut rogue_ctrl = |span: &SourceSpan| {
            self.clinic.report(
                Error::RogueControlStatement {
                    span: span.to_miette_span(),
                    allowed: vec!["for-loops".to_owned(), "while-loops".to_owned()].into(),
                }
                .into(),
            );
            block
        };

        match control {
            Control::Break(brk) => {
                if let Some(loop_env) = &env.loop_env {
                    self.terminate(
                        hir,
                        block,
                        Term::Jump {
                            block: loop_env.exit,
                        },
                    );
                    loop_env.exit
                } else {
                    rogue_ctrl(&brk.span)
                }
            }
            Control::Continue(cont) => {
                if let Some(loop_env) = &env.loop_env {
                    self.terminate(
                        hir,
                        block,
                        Term::Jump {
                            block: loop_env.control,
                        },
                    );
                    loop_env.control
                } else {
                    rogue_ctrl(&cont.span)
                }
            }
            Control::Return(ret) => {
                let mut value: Option<TempId> = None;

                if let Some(ast_value) = ret.expr {
                    let [out] = env.temp_id.bump::<1>();
                    self.lower_expr(
                        hir,
                        env,
                        block,
                        self.ast.get(ast_value),
                        out,
                        ReadKind::Borrow,
                    );
                    value = Some(out);
                }

                self.terminate(hir, block, Term::Return { value });
                block
            }
            Control::If(ifs) => {
                let merge = self.block(hir);
                let conds = 1 + ifs.elseif.len();

                let mut then_blocks = Vec::with_capacity(conds);
                let mut cond_blocks = Vec::with_capacity(conds);
                cond_blocks.push(block);

                for _ in 1..conds {
                    cond_blocks.push(self.block(hir));
                }

                for _ in 0..conds {
                    then_blocks.push(self.block(hir));
                }

                let else_block = ifs.alt.as_ref().map(|_| self.block(hir));

                for i in 0..conds {
                    let [cond] = env.temp_id.bump::<1>();
                    let cond_block = cond_blocks[i];
                    let cond_expr = if i == 0 {
                        ifs.cond
                    } else {
                        ifs.elseif[i - 1].0
                    };

                    let iftrue = then_blocks[i];
                    let iffalse = if i + 1 < conds {
                        cond_blocks[i + 1]
                    } else {
                        else_block.unwrap_or(merge)
                    };

                    self.lower_expr(
                        hir,
                        env,
                        cond_block,
                        self.ast.get(cond_expr),
                        cond,
                        ReadKind::Borrow,
                    );

                    self.terminate(
                        hir,
                        cond_block,
                        Term::Branch {
                            cond,
                            iftrue,
                            iffalse,
                        },
                    );
                }

                let last = self.lower_body(hir, env, then_blocks[0], &ifs.body);
                if !self.is_terminated(hir, last) {
                    self.terminate(hir, last, Term::Jump { block: merge });
                }

                for (i, (_, body)) in ifs.elseif.iter().enumerate() {
                    let last = self.lower_body(hir, env, then_blocks[i + 1], body);
                    if !self.is_terminated(hir, last) {
                        self.terminate(hir, last, Term::Jump { block: merge });
                    }
                }

                if let Some(alt) = &ifs.alt {
                    let mut cur = else_block.unwrap();
                    for stmt in &alt.inner {
                        cur = self.lower_stmt(hir, env, cur, self.ast.get(*stmt));
                    }
                    if !self.is_terminated(hir, cur) {
                        self.terminate(hir, cur, Term::Jump { block: merge });
                    }
                }

                merge
            }
            Control::While(whiles) => {
                let control = self.block(hir);
                let body = self.block(hir);
                let exit = self.block(hir);

                env.set_loop_env(Some(LoopEnv { control, exit }));

                let [cond] = env.temp_id.bump::<1>();

                self.lower_expr(
                    hir,
                    env,
                    control,
                    self.ast.get(whiles.cond),
                    cond,
                    ReadKind::Borrow,
                );

                self.terminate(
                    hir,
                    control,
                    Term::Branch {
                        cond,
                        iftrue: body,
                        iffalse: exit,
                    },
                );

                self.lower_body(hir, env, body, &whiles.body);
                self.terminate(hir, body, Term::Jump { block: control });
                self.terminate(hir, block, Term::Jump { block: control });

                env.set_loop_env(None);
                exit
            }
            _ => todo!(),
        }
    }
}

use itertools::Itertools;

use crate::db::{Db, Diagnostic, IntoSymbol, Severity, SourceProgram};
use crate::syntax::parse_program;

mod builder;
pub mod core;
pub mod def;
pub mod expr;
pub mod mangle;
pub mod ops;
pub mod pat;
pub mod path;
pub mod resolve;
pub mod scope;
pub mod stat;
pub mod ty;

#[salsa::tracked(debug)]
pub struct Hir<'db> {
    #[tracked]
    #[returns(ref)]
    pub root: def::Module<'db>,
}

#[salsa::tracked]
pub fn lower_program_to_hir<'db>(db: &'db dyn Db, program: SourceProgram) -> Option<Hir<'db>> {
    let name = program.identity(db).as_str().into_symbol(db);
    let ast = parse_program(db, program);

    // dbg!(crate::syntax::SyntaxNode::new_root(ast.root(db)));

    parse_program::accumulated::<Diagnostic>(db, program)
        .iter()
        .inspect(|diagnostic| println!("{diagnostic:#?}"))
        .filter(|Diagnostic { severity, .. }| matches!(severity, Severity::Error))
        .collect_vec()
        .is_empty()
        .then(|| {
            let builder = builder::HirBuilder::new(db, *ast);
            builder.build(name)
        })
        .inspect(|hir| {
            // dbg!(get_scope_tree(db, *hir));
            dbg!(hir.root(db));
        })
}

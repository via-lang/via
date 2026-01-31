/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::path::Path;

use anyhow::Result;

use viac::module::{Fixture, ModuleKind, context::ModuleContext};

pub fn run(path: &Path) -> Result<Fixture> {
    let mut ctxt = ModuleContext::new(path);
    let id = ctxt.load(path, "main")?;
    let module = ctxt.get(id).expect(
        "this module is invalid even though it was just loaded; id assignment is probably cooked",
    );

    // This attribute is needed as
    #[allow(irrefutable_let_patterns)]
    let ModuleKind::Source { fixture, .. } = module.kind() else {
        unreachable!("module kind must be Source here");
    };

    Ok(fixture.clone())
}

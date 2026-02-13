/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::path::Path;

use viac::module::context::ModuleContext;

pub fn run(path: &Path) -> miette::Result<()> {
    let mut ctxt = ModuleContext::new();
    ctxt.load_script(path).map_err(miette::Report::new)?;
    Ok(())
}

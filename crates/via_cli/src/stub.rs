/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::path::Path;

use anyhow::{Ok, Result, anyhow};

use viac::module::context::ModuleContext;

pub fn run(path: &Path) -> Result<()> {
    let mut ctxt = ModuleContext::new(path);
    ctxt.load(path, "main");

    if !ctxt.is_healthy() {
        return Err(anyhow!("dihh"));
    }
    Ok(())
}

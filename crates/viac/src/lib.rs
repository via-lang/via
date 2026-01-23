/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod prelude {
    pub use viac_ast as ast;
    pub use viac_diags as diags;
    pub use viac_ir as ir;
    pub use viac_lexer as lexer;
    pub use viac_parser as parser;
    pub use viac_source as source;
}

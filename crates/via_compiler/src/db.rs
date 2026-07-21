#[salsa::db]
pub trait Db: salsa::Database {}

#[salsa::db]
#[derive(Clone, Default)]
pub struct CompilerDb {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl Db for CompilerDb {}

#[salsa::db]
impl salsa::Database for CompilerDb {}

#[salsa::input]
#[derive(Debug)]
pub struct SourceProgram {
    #[returns(ref)]
    pub identity: String,
    #[returns(ref)]
    pub contents: String,
}

pub trait IntoDiagnostic {
    fn into_diagnostic(self, range: rowan::TextRange) -> Diagnostic;
}

#[salsa::accumulator]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub severity: Severity,
    pub range: rowan::TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Note,
    Warning,
    Error,
}

#[salsa::interned(debug)]
pub struct Symbol {
    #[returns(ref)]
    pub text: String,
}

impl<'db> Symbol<'db> {
    pub fn mangle(self, db: &'db dyn Db) -> String {
        let text = self.text(db);
        format!("{}{}", text.len(), text)
    }
}

pub trait IntoSymbol {
    fn into_symbol<'db>(self, db: &'db dyn Db) -> Symbol<'db>;
}

impl IntoSymbol for &str {
    fn into_symbol<'db>(self, db: &'db dyn Db) -> Symbol<'db> {
        Symbol::new(db, self)
    }
}

impl IntoSymbol for String {
    fn into_symbol<'db>(self, db: &'db dyn Db) -> Symbol<'db> {
        Symbol::new(db, self)
    }
}

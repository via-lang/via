use std::fmt::Debug;

pub enum Severity {
    Info,
    Warning,
    Error,
}

pub trait Diagnostic: Debug {
    fn severity(&self) -> Severity;
}

#[derive(Debug)]
pub struct Clinic {
    healthy: bool,
    diags: Vec<Box<dyn Diagnostic>>,
}

impl Clinic {
    pub fn new() -> Self {
        Self {
            healthy: true,
            diags: Vec::new(),
        }
    }

    pub fn healthy(&self) -> bool {
        self.healthy
    }

    pub fn report(&mut self, diag: impl Diagnostic + 'static) {
        self.healthy = !matches!(diag.severity(), Severity::Error);
        self.diags.push(Box::new(diag));
    }

    pub fn emit(&mut self) {
        for diag in self.diags.drain(..) {
            println!("{diag:?}")
        }
    }
}

impl Default for Clinic {
    fn default() -> Self {
        Self::new()
    }
}

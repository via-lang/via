#[derive(Debug)]
pub enum Error {
    DuplicateTrait,
    DuplicateTraitMethod,
    TraitMethodArityMismatch,
    TraitMethodParamTypeMismatch,
    TraitMethodReturnTypeMismatch,
}

pub type Result<T> = std::result::Result<T, Error>;

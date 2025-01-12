use super::*;

#[derive(Debug)]
pub enum TypeEnum {
    U64,
    U32,
    U16,
    U8,
    I64,
    I32,
    I16,
    I8,
    Void,
}

#[derive(Debug)]
pub struct Type {
    pub ty: TypeEnum,
    pub star: usize,
    pub span: Span,
}

use super::*;

#[derive(Debug)]
pub struct CompUnit {
    pub global_items: Vec<GlobalItem>,
    pub span: Span,
}

#[derive(Debug)]
pub enum GlobalItem {
    Decl(Decl),
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub span: Span,
}

#[derive(Debug)]
pub enum BlockItem {
    Statement(Statement),
    Declaration(Decl),
}

#[derive(Debug)]
pub enum Decl {
    VarDecl(VarDecl),
    ConstDecl(ConstDecl),
    ExternFunctionDef(ExternFunctionDef),
}

#[derive(Debug)]
pub enum Statement {
    Return(Return),
    Exp(Option<Exp>),
}

#[derive(Debug)]
pub struct Return {
    pub value: Option<Exp>,
    pub span: Span,
}

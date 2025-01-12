use crate::ast::*;
use crate::cgen::*;
use alloc::{sync::Arc,vec::Vec};
use spin::RwLock;

mod codegen;
mod error;

pub use error::*;

pub struct CodegenContext {
    pub c_program: CProgram,
    pub current_function: Option<CIdentifier>,
    pub local: SymbolTable,
    pub global: SymbolTable,
    pub func_id: usize,
    pub errors: Vec<CompileError>,
}

pub trait Codegen {
    type Target;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Self::Target>;
}

impl CodegenContext {
    pub fn new() -> Self {
        Self {
            c_program: CProgram::new(),
            local: SymbolTable::new(),
            global: SymbolTable::new(),
            current_function: None,
            func_id: 0,
            errors: Vec::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.local.push(Symbol::Scope);
    }

    pub fn pop_scope(&mut self) {
        self.local.pop();
    }

    pub fn new_func_id(&mut self) -> usize {
        let id = self.func_id;
        self.func_id += 1;
        id
    }
}

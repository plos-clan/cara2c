use core::fmt::Display;

use alloc::{string::String, vec::Vec};

mod decl;
mod func;
mod scope;
mod types;
mod values;

pub use decl::*;
pub use func::*;
pub use scope::*;
pub use types::*;
pub use values::*;

pub struct CProgram {
    items: Vec<CDeclaration>,
}

impl CProgram {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert_function(&mut self, function: CFunction) {
        self.items.push(CDeclaration::FunctionDef(function));
    }

    pub fn insert_const(&mut self, constant: CConst) {
        self.items.push(CDeclaration::ConstDef(constant));
    }

    pub fn insert_variable(&mut self, variable: CVariable) {
        self.items.push(CDeclaration::VariableDef(variable));
    }

    pub fn insert_decl(&mut self, decl: CDeclaration) {
        self.items.push(decl);
    }

    pub fn function_mut(&mut self, function: String) -> Option<&mut CFunction> {
        for item in self.items.iter_mut() {
            match item {
                CDeclaration::FunctionDef(func) => {
                    if func.get_name() == function {
                        return Some(func);
                    }
                }
                _ => {}
            }
        }

        None
    }
}

impl Display for CProgram {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "typedef signed char i8;")?;
        writeln!(f, "typedef short i16;")?;
        writeln!(f, "typedef int i32;")?;
        writeln!(f, "typedef long long i64;\n")?;
        writeln!(f, "typedef unsigned char u8;")?;
        writeln!(f, "typedef unsigned short u16;")?;
        writeln!(f, "typedef unsigned int u32;")?;
        writeln!(f, "typedef unsigned long long u64;\n")?;
        for item in self.items.iter() {
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}

pub enum CDeclaration {
    FunctionDef(CFunction),
    ConstDef(CConst),
    VariableDef(CVariable),
}

impl Display for CDeclaration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CDeclaration::FunctionDef(func) => write!(f, "{}", func),
            CDeclaration::ConstDef(constant) => write!(f, "{}", constant),
            CDeclaration::VariableDef(variable) => write!(f, "{}", variable),
        }
    }
}

use crate::{ast::Span, backend::CompileError};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use super::*;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CFunctionType {
    param_types: Vec<CType>,
    return_type: Box<CType>,
}

impl CFunctionType {
    pub fn new(param_types: Vec<CType>, return_type: CType) -> Self {
        Self {
            param_types,
            return_type: Box::new(return_type),
        }
    }

    pub fn get_return_type(&self) -> CType {
        *self.return_type.clone()
    }

    pub fn get_param_types(&self) -> Vec<CType> {
        self.param_types.clone()
    }
}

impl Display for CFunctionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} (*) (", self.return_type)?;
        for (i, param_type) in self.param_types.iter().enumerate() {
            write!(f, "{}", param_type)?;
            if i < self.param_types.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub struct CIntType {
    bit_width: u32,
    signed: bool,
}

impl CIntType {
    pub fn new(bit_width: u32, signed: bool) -> Self {
        Self { bit_width, signed }
    }
}

#[derive(Debug, Clone)]
enum TypeEnum {
    Void,
    Function(CFunctionType),
    Int(CIntType),
    Type,
    Const(Box<CType>),
}

#[derive(Debug, Clone)]
pub struct CType {
    type_enum: TypeEnum,
    symbols: SymbolTable,
}

impl CType {
    pub fn new_void() -> Self {
        Self {
            type_enum: TypeEnum::Void,
            symbols: SymbolTable::new(),
        }
    }

    pub fn new_type() -> Self {
        Self {
            type_enum: TypeEnum::Type,
            symbols: SymbolTable::new(),
        }
    }

    pub fn new_function(function_type: CFunctionType) -> Self {
        Self {
            type_enum: TypeEnum::Function(function_type),
            symbols: SymbolTable::new(),
        }
    }

    pub fn new_int(int_type: CIntType) -> Self {
        Self {
            type_enum: TypeEnum::Int(int_type),
            symbols: SymbolTable::new(),
        }
    }

    pub fn new_const(type_: CType) -> Self {
        Self {
            type_enum: TypeEnum::Const(Box::new(type_)),
            symbols: SymbolTable::new(),
        }
    }

    pub fn get_name(&self) -> String {
        match &self.type_enum {
            TypeEnum::Void => "void".to_string(),
            TypeEnum::Function(_) => "function".to_string(),
            TypeEnum::Int(_) => "int".to_string(),
            TypeEnum::Type => "type".to_string(),
            TypeEnum::Const(type_) => alloc::format!("const {}", type_.get_name()),
        }
    }
}

impl CType {
    pub fn is_const(&self) -> bool {
        match &self.type_enum {
            TypeEnum::Const(_) => true,
            _ => false,
        }
    }
}

impl CType {
    pub fn as_function(&self, span: Span) -> Result<CFunctionType> {
        match &self.type_enum {
            TypeEnum::Function(func_type) => Ok(func_type.clone()),
            TypeEnum::Const(type_) => type_.as_function(span),
            _ => Err(
                CompileError::new_invalid_type_cast(span, self.get_name(), "function".into())
                    .into(),
            ),
        }
    }
}

impl CType {
    pub fn function_type(&self, paramter_types: Vec<CType>, span: Span) -> Result<Self> {
        let mut paramter_types_list = Vec::new();

        for param_type in paramter_types.leak().iter() {
            paramter_types_list.push(param_type.clone());
        }

        let value = match &self.type_enum {
            TypeEnum::Int(int_type) => {
                CFunctionType::new(paramter_types_list, Self::new_int(int_type.clone()))
            }
            TypeEnum::Void => CFunctionType::new(paramter_types_list, Self::new_void()),
            _ => return Err(CompileError::new_non_comptime_value(span, self.get_name()).into()),
        };

        Ok(Self::new_function(value.clone()))
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.type_enum {
            TypeEnum::Function(function_type) => write!(f, "{}", function_type),
            TypeEnum::Int(int_type) => {
                if int_type.signed {
                    write!(f, "i{}", int_type.bit_width)
                } else {
                    write!(f, "u{}", int_type.bit_width)
                }
            }
            TypeEnum::Void => write!(f, "void"),
            TypeEnum::Type => write!(f, "type"),
            TypeEnum::Const(type_) => write!(f, "const {}", type_),
        }
    }
}

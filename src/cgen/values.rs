use crate::ast::{BinaryOp, Span};

use super::*;
use crate::backend::CompileError;
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CIdentifier {
    name: String,
    type_: CType,
}

impl CIdentifier {
    pub fn new(name: String, type_: CType) -> Self {
        Self { name, type_ }
    }

    pub fn get_type(&self) -> CType {
        self.type_.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone)]
enum ValueEnum {
    Identyfier(CIdentifier),
    Int(u64),
    BinOp(BinOp),
    Type(CType),
    Void,
    GetAddr(CGetAddr),
    ConvertType(CConvertType),
    Call(CCall),
}

#[derive(Debug, Clone)]
pub struct Value {
    value_enum: ValueEnum,
    value_type: CType,
}

impl Value {
    pub fn new_void() -> Self {
        Self {
            value_enum: ValueEnum::Void,
            value_type: CType::new_void(),
        }
    }

    pub fn new_call(call: CCall) -> Self {
        Self {
            value_enum: ValueEnum::Call(call),
            value_type: CType::new_void(),
        }
    }

    pub fn new_identifier(identifier: CIdentifier) -> Self {
        Self {
            value_enum: ValueEnum::Identyfier(identifier.clone()),
            value_type: identifier.get_type(),
        }
    }

    pub fn new_int(value: u64, bit_width: u32, signed: bool) -> Self {
        Self {
            value_enum: ValueEnum::Int(value),
            value_type: CType::new_int(CIntType::new(bit_width, signed)),
        }
    }

    pub fn new_bin_op(bin_op: BinOp) -> Self {
        Self {
            value_enum: ValueEnum::BinOp(bin_op.clone()),
            value_type: bin_op.lhs.get_type(),
        }
    }

    pub fn new_type(type_: CType) -> Self {
        Self {
            value_enum: ValueEnum::Type(type_),
            value_type: CType::new_type(),
        }
    }

    pub fn new_get_addr(get_addr: CGetAddr) -> Self {
        Self {
            value_enum: ValueEnum::GetAddr(get_addr),
            value_type: CType::new_type(),
        }
    }

    pub fn new_convert_type(convert_type: CConvertType) -> Self {
        Self {
            value_enum: ValueEnum::ConvertType(convert_type.clone()),
            value_type: convert_type.type_,
        }
    }
}

impl Value {
    pub fn as_identifier_mut(&mut self, span: Span) -> Result<&mut CIdentifier> {
        match &mut self.value_enum {
            ValueEnum::Identyfier(identifier) => Ok(identifier),
            _ => Err(CompileError::new_invalid_type_cast(
                span,
                self.value_type.get_name(),
                "function".to_string(),
            )
            .into()),
        }
    }

    pub fn as_int(&self, span: Span) -> Result<u64> {
        match &self.value_enum {
            ValueEnum::Int(val) => Ok(*val),
            _ => Err(CompileError::new_invalid_type_cast(
                span,
                self.value_type.get_name(),
                "int".to_string(),
            )
            .into()),
        }
    }
}

impl Value {
    pub fn get_type(&self) -> CType {
        self.value_type.clone()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.value_enum {
            ValueEnum::Identyfier(identifier) => write!(f, "{}", identifier.get_name()),
            ValueEnum::Int(val) => write!(f, "{}", val),
            ValueEnum::BinOp(bin_op) => write!(f, "{}", bin_op),
            ValueEnum::Void => write!(f, "void"),
            ValueEnum::Type(type_) => write!(f, "{}", type_),
            ValueEnum::GetAddr(get_addr) => write!(f, "{}", get_addr),
            ValueEnum::ConvertType(convert_type) => write!(f, "{}", convert_type),
            ValueEnum::Call(call) => write!(f, "{}", call),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinOp {
    lhs: Box<Value>,
    rhs: Box<Value>,
    op: BinaryOp,
}

impl BinOp {
    pub fn new(lhs: Value, rhs: Value, op: BinaryOp) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Neq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Ge => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CGetAddr {
    value: Box<Value>,
}

impl CGetAddr {
    pub fn new(value: Value) -> Self {
        Self {
            value: Box::new(value),
        }
    }
}

impl Display for CGetAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "&{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct CConvertType {
    value: Box<Value>,
    type_: CType,
}

impl CConvertType {
    pub fn new(value: Value, type_: CType) -> Self {
        Self {
            value: Box::new(value),
            type_,
        }
    }
}

impl Display for CConvertType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}){}", self.type_, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct CCall {
    value: Box<Value>,
    args: Vec<Value>,
}

impl CCall {
    pub fn new(value: Value, args: Vec<Value>) -> Self {
        Self {
            value: Box::new(value),
            args,
        }
    }
}

impl Display for CCall {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({})(", self.value)?;

        for (i, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;

            if i < self.args.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}

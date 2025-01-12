use alloc::string::String;

use super::*;

pub struct CFunction {
    name: String,
    return_type: CType,
    r#static: bool,
    r#extern: bool,
    pub body: CBlock,
    params: Vec<(CType, String)>,
}

impl CFunction {
    pub fn new(
        name: String,
        return_type: CType,
        r#static: bool,
        r#extern: bool,
        params: Vec<(CType, String)>,
    ) -> Self {
        Self {
            name,
            return_type,
            body: CBlock::new(),
            r#static,
            r#extern,
            params,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Display for CFunction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.r#static {
            write!(f, "static ")?;
        }

        if !self.r#extern {
            write!(f, "{} {}(", self.return_type, self.name)?;

            for (i, (ty, name)) in self.params.iter().enumerate() {
                write!(f, "{} {}", ty, name)?;

                if i < self.params.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            write!(f, ") {{ \n")?;

            write!(f, "{}", self.body)?;

            write!(f, "}}")
        } else {
            write!(f, "{} {}(", self.return_type, self.name)?;

            for (i, (ty, name)) in self.params.iter().enumerate() {
                write!(f, "{} {}", ty, name)?;

                if i < self.params.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            write!(f, ");")
        }
    }
}

pub struct CBlock {
    items: Vec<CBlockItem>,
}

impl CBlock {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: CBlockItem) {
        self.items.push(item);
    }
}

impl Display for CBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for item in self.items.iter() {
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}

pub enum CBlockItem {
    Statement(CStatement),
    Decl(CDeclaration),
    Exp(Option<Value>),
}

impl Display for CBlockItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::Statement(statement) => write!(f, "{}", statement),
            Self::Decl(decl) => write!(f, "{}", decl),
            Self::Exp(value) => {
                if let Some(value) = value {
                    write!(f, "{};", value)
                } else {
                    write!(f, ";")
                }
            }
        }
    }
}

pub enum CStatement {
    Return(CReturn),
}

impl Display for CStatement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::Return(r) => {
                if let Some(value) = &r.value {
                    write!(f, "return {};", value)
                } else {
                    write!(f, "return;")
                }
            }
        }
    }
}

pub struct CReturn {
    value: Option<Value>,
}

impl CReturn {
    pub fn new(value: Option<Value>) -> Self {
        Self { value }
    }
}

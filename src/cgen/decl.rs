use crate::ast::Span;

use super::*;

pub struct CConst {
    name: String,
    value: Value,
}

impl CConst {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

impl Display for CConst {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Ok(function) =
            self.value
                .get_type()
                .as_function(Span::new((0, 0), (0, 0), "".into(), "".into()))
        {
            write!(f, "const {} (*{}) (", function.get_return_type(), self.name)?;
            for (i, param) in function.get_param_types().iter().enumerate() {
                write!(f, "{}", param)?;
                if i < function.get_param_types().len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, ") = {};", self.value)
        } else {
            write!(
                f,
                "const {} {} = {};",
                self.value.get_type(),
                self.name,
                self.value
            )
        }
    }
}

pub struct CTypedef {
    name: String,
    type_: CType,
}

impl CTypedef {
    pub fn new(name: String, type_: CType) -> Self {
        Self { name, type_ }
    }
}

impl Display for CTypedef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Ok(function) =
            self.type_
                .as_function(Span::new((0, 0), (0, 0), "".into(), "".into()))
        {
            write!(
                f,
                "typedef {} (*{}) (",
                function.get_return_type(),
                self.name
            )?;
            for (i, param) in function.get_param_types().iter().enumerate() {
                write!(f, "{}", param)?;
                if i < function.get_param_types().len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, ");")
        } else {
            write!(f, "typedef {} {};", self.type_, self.name)
        }
    }
}

pub struct CVariable {
    name: String,
    value: Value,
    type_: CType,
}

impl CVariable {
    pub fn new(name: String, value: Value, type_: CType) -> Self {
        Self { name, value, type_ }
    }
}

impl Display for CVariable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Ok(function) =
            self.type_
                .as_function(Span::new((0, 0), (0, 0), "".into(), "".into()))
        {
            if self.type_.is_const() {
                write!(f, "const ")?;
            }

            write!(f, "{} (*{}) (", function.get_return_type(), self.name)?;
            for (i, param) in function.get_param_types().iter().enumerate() {
                write!(f, "{}", param)?;
                if i < function.get_param_types().len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, ") = {};", self.value)
        } else {
            write!(f, "{} {} = {};", self.type_, self.name, self.value)
        }
    }
}

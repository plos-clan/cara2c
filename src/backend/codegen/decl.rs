use core::hash::{BuildHasher, Hasher};
use rs_shield::{HasherContext, Sha512State};

use super::*;

impl Codegen for Decl {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        match self {
            Decl::VarDecl(var_decl) => var_decl.codegen(context),
            Decl::ConstDecl(const_decl) => const_decl.codegen(context),
            Decl::ExternFunctionDef(extern_function_def) => extern_function_def.codegen(context),
        }
    }
}

impl Codegen for ConstDecl {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        let value = self.initial_value.codegen(context.clone())?.clone();

        let mut context = context.write();

        if value.get_type().is_const() {
            context.c_program.insert_variable(CVariable::new(
                self.name.clone(),
                value.clone(),
                value.get_type(),
            ));
        } else {
            context
                .c_program
                .insert_const(CConst::new(self.name.clone(), value.clone()));
        }

        if context.local.len() != 0 {
            context.local.push(Symbol::Const(
                self.name.clone(),
                Value::new_identifier(CIdentifier::new(self.name.clone(), value.get_type())),
            ));
        } else {
            context.global.push(Symbol::Const(
                self.name.clone(),
                Value::new_identifier(CIdentifier::new(self.name.clone(), value.get_type())),
            ));
        }

        Ok(())
    }
}

impl Codegen for VarDecl {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Self::Target> {
        let value = self.initial_value.codegen(context.clone())?.clone();
        let type_ = self.var_type.codegen(context.clone())?;

        let var_decl =
            CDeclaration::VariableDef(CVariable::new(self.name.clone(), value, type_.clone()));

        let var_id = Value::new_identifier(CIdentifier::new(self.name.clone(), type_));

        let mut context = context.write();
        if let Some(current_function) = context.current_function.clone() {
            let current_function = current_function.get_name();
            let function = context.c_program.function_mut(current_function).unwrap();
            function.body.push(CBlockItem::Decl(var_decl));

            context.local.push(Symbol::Var(self.name.clone(), var_id));
        } else {
            context.c_program.insert_decl(var_decl);

            context.global.push(Symbol::Var(self.name.clone(), var_id));
        }

        Ok(())
    }
}

impl Codegen for ConstInitialValue {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Value> {
        match &self.value {
            ConstInitialValueEnum::Exp(exp) => exp.codegen(context.clone()),
            ConstInitialValueEnum::Function(func) => Ok(func.codegen(context.clone())?),
        }
    }
}

impl Codegen for FunctionDef {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Value> {
        let func_type = self
            .return_type
            .codegen(context.clone())?
            .function_type(Vec::new(), self.span.clone())?;

        let params = self
            .params
            .iter()
            .map(|param| {
                (
                    param.param_type.codegen(context.clone()).unwrap(),
                    param.name.clone(),
                )
            })
            .collect::<Vec<_>>();

        let mut context_write = context.write();

        let name = alloc::format!("abcdefg_fn{}_hijklmn", context_write.new_func_id());

        let mut sha512hasher = Sha512State::default().build_hasher();
        sha512hasher.write(name.as_bytes());
        let bytes_result = HasherContext::finish(&mut sha512hasher);
        let mut hashed = alloc::format!("{:02x}", bytes_result);
        let _ = hashed.split_off(20);
        let name = alloc::format!("_{}_{}", hashed, name);

        context_write.c_program.insert_function(CFunction::new(
            name.clone(),
            self.return_type.codegen(context.clone())?,
            true,
            false,
            params,
        ));

        let function = CIdentifier::new(name.clone(), func_type.clone());
        let addr = CGetAddr::new(Value::new_identifier(function.clone()));
        let convert = CConvertType::new(
            Value::new_get_addr(addr),
            CType::new_const(func_type.clone()),
        );

        context_write.current_function = Some(function.clone());

        drop(context_write);

        self.block.codegen(context.clone())?;

        context.write().current_function = None;

        Ok(Value::new_convert_type(convert))
    }
}

impl Codegen for ExternFunctionDef {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Self::Target> {
        let func_type = self.return_type.codegen(context.clone())?.function_type(
            self.params
                .iter()
                .map(|param| param.param_type.codegen(context.clone()).unwrap())
                .collect(),
            self.span.clone(),
        )?;

        let params = self
            .params
            .iter()
            .map(|param| {
                (
                    param.param_type.codegen(context.clone()).unwrap(),
                    param.name.clone(),
                )
            })
            .collect::<Vec<_>>();

        let mut context_write = context.write();

        let name = self.name.clone();

        context_write.c_program.insert_function(CFunction::new(
            name.clone(),
            self.return_type.codegen(context.clone())?,
            false,
            true,
            params,
        ));

        let function = CIdentifier::new(name.clone(), func_type.clone());
        let addr = CGetAddr::new(Value::new_identifier(function.clone()));
        let convert = CConvertType::new(
            Value::new_get_addr(addr),
            CType::new_const(func_type.clone()),
        );

        context_write.global.push(Symbol::Const(
            name.clone(),
            Value::new_convert_type(convert),
        ));

        Ok(())
    }
}

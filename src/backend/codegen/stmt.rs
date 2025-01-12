use super::*;

impl Codegen for Block {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        context.write().push_scope();
        let stack_size = context.read().local.len();

        for item in self.items.iter() {
            item.codegen(context.clone())?;
        }

        let mut context_write = context.write();

        while context_write.local.len() > stack_size {
            context_write.local.pop();
        }

        context_write.pop_scope();

        Ok(())
    }
}

impl Codegen for BlockItem {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        match self {
            BlockItem::Statement(statement) => statement.codegen(context.clone())?,
            BlockItem::Declaration(decl) => decl.codegen(context.clone())?,
        }

        Ok(())
    }
}

impl Codegen for Statement {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        match self {
            Statement::Return(ret) => ret.codegen(context.clone())?,
            Self::Exp(exp) => {
                if let Some(exp) = exp {
                    let value = exp.codegen(context.clone())?;
                    let mut context_write = context.write();
                    let current = context_write.current_function.clone().unwrap().get_name();
                    let current = context_write.c_program.function_mut(current).unwrap();
                    current.body.push(CBlockItem::Exp(Some(value.clone())));
                }
            }
        }

        Ok(())
    }
}

impl Codegen for Return {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        let value = self
            .value
            .as_ref()
            .map(|value| value.codegen(context.clone()));

        match value {
            Some(Ok(value)) => {
                let function = context.read().current_function.clone().unwrap();
                let mut context_write = context.write();
                context_write
                    .c_program
                    .function_mut(function.get_name())
                    .unwrap()
                    .body
                    .push(CBlockItem::Statement(CStatement::Return(CReturn::new(
                        Some(value),
                    ))));
            }
            None => {
                let function_name = context.read().current_function.clone().unwrap().get_name();
                context
                    .write()
                    .c_program
                    .function_mut(function_name)
                    .unwrap()
                    .body
                    .push(CBlockItem::Statement(CStatement::Return(CReturn::new(
                        None,
                    ))));
            }
            _ => {}
        }

        Ok(())
    }
}

use super::*;

impl Codegen for ConstExp {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Self::Target> {
        self.exp.codegen(context)
    }
}

impl Codegen for Exp {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Value> {
        Ok(match self {
            Exp::Binary(lhs, op, rhs, _span) => {
                let lhs = lhs.codegen(context.clone())?;
                let rhs = rhs.codegen(context.clone())?;
                Value::new_bin_op(BinOp::new(lhs, rhs, op.clone()))
            }
            Exp::Number(number) => Value::new_int(number.num, 64, true),
            Exp::ConvertType(convert_type) => convert_type.codegen(context.clone())?,
            Exp::LVal(lval) => lval.codegen(context.clone())?,
            Exp::Call(call) => call.codegen(context.clone())?,
            _ => unimplemented!(),
        })
    }
}

impl Codegen for Call {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Value> {
        let value = self.exp.codegen(context.clone())?;
        
        let call = Value::new_call(CCall::new(
            value,
            self.args
                .iter()
                .map(|arg| arg.codegen(context.clone()).unwrap())
                .collect(),
        ));
        Ok(call)
    }
}

impl Codegen for LVal {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Value> {
        let id = self.ids[0].clone();
        let context = context.read();
        let value = match context.local.get(&id) {
            Some(symbol) => match symbol {
                Symbol::Const(_, value) => value.clone(),
                Symbol::Var(_, value) => value.clone(),
                _ => unimplemented!(),
            },
            None => match context.global.get(&id) {
                Some(symbol) => match symbol {
                    Symbol::Const(_, value) => value.clone(),
                    Symbol::Var(_, value) => value.clone(),
                    _ => unimplemented!(),
                },
                None => unimplemented!(),
            },
        };
        Ok(value)
    }
}

impl Codegen for ConvertType {
    type Target = Value;

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<Self::Target> {
        let value = self.exp.codegen(context.clone())?;
        let ty = self.ty.codegen(context)?;
        Ok(Value::new_convert_type(CConvertType::new(value, ty)))
    }
}

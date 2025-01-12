use super::*;

impl Codegen for CompUnit {
    type Target = ();

    fn codegen(&self, context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<()> {
        for global_item in self.global_items.iter() {
            let ret = match global_item {
                GlobalItem::Decl(decl) => decl.codegen(context.clone()),
            };
            if let Err(e) = ret {
                let e: CompileError = e.downcast()?;
                context.write().errors.push(e);
            }
        }

        Ok(())
    }
}

impl Codegen for Type {
    type Target = CType;

    fn codegen(&self, _context: Arc<RwLock<CodegenContext>>) -> anyhow::Result<CType> {
        match self.ty {
            TypeEnum::U64 => Ok(CType::new_int(CIntType::new(64, false))),
            TypeEnum::U32 => Ok(CType::new_int(CIntType::new(32, false))),
            TypeEnum::U16 => Ok(CType::new_int(CIntType::new(16, false))),
            TypeEnum::U8 => Ok(CType::new_int(CIntType::new(8, false))),
            TypeEnum::I64 => Ok(CType::new_int(CIntType::new(64, true))),
            TypeEnum::I32 => Ok(CType::new_int(CIntType::new(32, true))),
            TypeEnum::I16 => Ok(CType::new_int(CIntType::new(16, true))),
            TypeEnum::I8 => Ok(CType::new_int(CIntType::new(8, true))),
            TypeEnum::Void => Ok(CType::new_void()),
        }
    }
}

use super::*;
use alloc::string::ToString;

impl CParser {
    pub fn parse_decl(&self, rules: Pair<Rule>) -> Decl {
        let mut rules_iter = rules.clone().into_inner();

        let decl = rules_iter.next().unwrap();

        match decl.as_rule() {
            Rule::var_decl => Decl::VarDecl(self.parse_var_decl(decl)),
            Rule::const_decl => Decl::ConstDecl(self.parse_const_decl(decl)),
            Rule::extern_function_def => {
                Decl::ExternFunctionDef(self.parse_extern_function_def(decl))
            }
            _ => unimplemented!(),
        }
    }

    pub fn parse_var_decl(&self, rules: Pair<Rule>) -> VarDecl {
        let mut rules_iter = rules.clone().into_inner();

        let id_rule = rules_iter.next().unwrap();
        let id = self.parse_ident(id_rule);

        let var_type = self.parse_type(rules_iter.next().unwrap());

        let initial_value = self.parse_expr(rules_iter.next().unwrap());

        VarDecl {
            name: id,
            var_type,
            initial_value,
            span: self.get_span(rules.as_span().clone()),
        }
    }

    pub fn parse_const_decl(&self, rules: Pair<Rule>) -> ConstDecl {
        let mut rules_iter = rules.clone().into_inner();

        let id_rule = rules_iter.next().unwrap();
        let id = self.parse_ident(id_rule);

        let initial_value = self.parse_const_initial_value(rules_iter.next().unwrap());

        ConstDecl {
            name: id,
            initial_value,
            span: self.get_span(rules.as_span().clone()),
        }
    }

    pub fn parse_const_initial_value(&self, rules: Pair<Rule>) -> ConstInitialValue {
        let mut rules_iter = rules.clone().into_inner();

        let initial_value = rules_iter.next().unwrap();

        let value = match initial_value.as_rule() {
            Rule::function_def => {
                ConstInitialValueEnum::Function(self.parse_function_def(initial_value))
            }
            Rule::const_exp => ConstInitialValueEnum::Exp(self.parse_const_exp(initial_value)),
            _ => unimplemented!(),
        };

        ConstInitialValue {
            value,
            span: self.get_span(rules.as_span().clone()),
        }
    }

    pub fn parse_extern_function_def(&self, rules: Pair<Rule>) -> ExternFunctionDef {
        let mut rules_iter = rules.clone().into_inner();

        let name = rules_iter.next().unwrap().as_str().to_string();

        let mut params = Vec::new();

        let return_type = loop {
            let first = rules_iter.next().unwrap();

            if first.as_rule() == Rule::param {
                params.push(self.parse_param(first));
            } else {
                break self.parse_type(first);
            }
        };

        ExternFunctionDef {
            name,
            params,
            return_type,
            span: self.get_span(rules.as_span().clone()),
        }
    }

    pub fn parse_function_def(&self, rules: Pair<Rule>) -> FunctionDef {
        let mut rules_iter = rules.clone().into_inner();

        let mut params = Vec::new();

        let return_type = loop {
            let first = rules_iter.next().unwrap();

            if first.as_rule() == Rule::param {
                params.push(self.parse_param(first));
            } else {
                break self.parse_type(first);
            }
        };

        let block = self.parse_block(rules_iter.next().unwrap());

        FunctionDef {
            params,
            return_type,
            block,
            span: self.get_span(rules.as_span().clone()),
        }
    }

    pub fn parse_param(&self, rules: Pair<Rule>) -> Param {
        let mut rules_iter = rules.clone().into_inner();
        let name = rules_iter.next().unwrap().as_str().to_string();
        let param_type = self.parse_type(rules_iter.next().unwrap());
        let span = self.get_span(rules.as_span().clone());
        Param {
            name,
            param_type,
            span,
        }
    }
}

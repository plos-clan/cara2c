use super::*;

impl CParser {
    pub fn parse_statement(&self, rules: Pair<Rule>) -> Statement {
        let mut rules_iter = rules.clone().into_inner();
        let statement = match rules_iter.next().unwrap().as_rule() {
            Rule::r#return => Statement::Return(self.parse_return(rules.clone())),
            Rule::exp_stmt => {
                let rule = rules.clone().into_inner().next().unwrap();
                let rule = rule.into_inner().next();
                
                let exp = Statement::Exp(
                    rule
                        .and_then(|x| Some(self.parse_expr(x))),
                );
                
                exp
            },
            _ => unimplemented!(),
        };
        statement
    }

    pub fn parse_return(&self, rules: Pair<Rule>) -> Return {
        let mut rules_iter = rules.clone().into_inner();
        let exp = rules_iter.next().unwrap().into_inner().next().map(|rule| {
            #[cfg(feature = "std")]
            self.parse_expr(rule)
        });
        Return {
            value: exp,
            span: self.get_span(rules.as_span().clone()),
        }
    }
}

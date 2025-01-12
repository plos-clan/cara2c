use super::*;
use alloc::vec;

impl CParser {
    pub fn parse_lval(&self, rules: Pair<Rule>) -> LVal {
        let mut rules_iter = rules.clone().into_inner();
        let id = rules_iter.next().unwrap();
        let span = self.get_span(rules.as_span().clone());
        LVal {
            ids: vec![self.parse_ident(id)],
            span,
            exp: None,
        }
    }

    pub fn parse_const_exp(&self, rules: Pair<Rule>) -> ConstExp {
        let mut rules_iter = rules.clone().into_inner();
        let exp = rules_iter.next().unwrap();
        ConstExp {
            exp: self.parse_expr(exp),
        }
    }

    pub fn parse_expr(&self, rules: Pair<Rule>) -> Exp {
        let pratt = PrattParser::new()
            .op(Op::infix(Rule::eq, Assoc::Left) | Op::infix(Rule::neq, Assoc::Left))
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left)
                | Op::infix(Rule::div, Assoc::Left)
                | Op::infix(Rule::r#mod, Assoc::Left))
            .op(Op::postfix(Rule::convert_type))
            .op(Op::postfix(Rule::call))
            .op(Op::prefix(Rule::neg) | Op::prefix(Rule::pos));

        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::exp => self.parse_expr(primary),
                Rule::number => Exp::Number(Number {
                    num: primary.as_str().parse().unwrap(),
                    span: self.get_span(primary.as_span()),
                }),
                Rule::lval => Exp::LVal(Box::new(self.parse_lval(primary))),
                Rule::ident => Exp::LVal(Box::new(LVal {
                    ids: vec![self.parse_ident(primary.clone())],
                    span: self.get_span(primary.as_span()),
                    exp: None,
                })),

                _ => panic!("Unkown primary {}!", primary),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::neg => Exp::Unary(
                    UnaryOp::Negative,
                    Box::new(rhs),
                    self.get_span(op.as_span()),
                ),
                Rule::pos => Exp::Unary(
                    UnaryOp::Positive,
                    Box::new(rhs),
                    self.get_span(op.as_span()),
                ),
                _ => unimplemented!(),
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::convert_type => {
                    let ty = self.parse_type(op.clone().into_inner().next().unwrap());
                    Exp::ConvertType(Box::new(ConvertType {
                        exp: lhs,
                        ty,
                        span: self.get_span(op.as_span()),
                    }))
                }
                Rule::call => {
                    let args = op.clone().into_inner().map(|arg| self.parse_expr(arg)).collect();

                    Exp::Call(Box::new(Call {
                        exp: lhs,
                        args,
                        span: self.get_span(op.as_span()),
                    }))
                },
                _ => unimplemented!(),
            })
            .map_infix(|lhs, op, rhs| {
                let lhs = Box::new(lhs);
                let rhs = Box::new(rhs);

                Exp::Binary(
                    lhs,
                    match op.as_rule() {
                        Rule::eq => BinaryOp::Eq,
                        Rule::neq => BinaryOp::Neq,
                        Rule::add => BinaryOp::Add,
                        Rule::sub => BinaryOp::Sub,
                        Rule::mul => BinaryOp::Mul,
                        Rule::div => BinaryOp::Div,
                        Rule::r#mod => BinaryOp::Mod,
                        _ => unimplemented!(),
                    },
                    rhs,
                    self.get_span(op.as_span()),
                )
            })
            .parse(rules.clone().into_inner())
    }
}

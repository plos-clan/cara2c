use crate::ast::*;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use pest::{iterators::Pair, pratt_parser::*};
use pest_derive::Parser;

mod decl;
mod exp;
mod stmt;

#[derive(Parser)]
#[grammar = "cara.pest"]
struct CaraParser;

pub struct CParser {
    code: String,
    file: String,
}

impl CParser {
    pub fn new(code: String, file: String) -> Self {
        Self { code, file }
    }

    fn get_span(&self, span: pest::Span<'_>) -> Span {
        let code = span.lines().next().unwrap();
        Span::new(
            span.start_pos().line_col(),
            span.end_pos().line_col(),
            code.into(),
            self.file.clone(),
        )
    }
}

impl CParser {
    pub fn parse(&self) -> CompUnit {
        use pest::Parser;
        let rules = CaraParser::parse(Rule::comp_unit, &self.code);

        if let Err(err) = rules {
            panic!("{}", err);
        }

        let rules = rules.unwrap().next().unwrap();

        let mut items = Vec::new();
        let span = rules.as_span().clone();

        for line in rules.into_inner() {
            match line.as_rule() {
                Rule::decl => items.push(GlobalItem::Decl(self.parse_decl(line))),
                Rule::soi | Rule::eoi => {}
                _ => unimplemented!(),
            }
        }

        CompUnit {
            global_items: items,
            span: self.get_span(span),
        }
    }

    pub fn parse_ident(&self, rules: Pair<Rule>) -> String {
        let mut ident = rules.as_str().to_string();
        if ident.starts_with("@") {
            ident.remove(0);
            ident.remove(0);
            ident.pop();
        }
        ident
    }

    pub fn parse_block(&self, rules: Pair<Rule>) -> Block {
        let mut rules_iter = rules.clone().into_inner();
        let mut items = Vec::new();
        while let Some(rule) = rules_iter.next() {
            match rule.as_rule() {
                Rule::stmt => items.push(BlockItem::Statement(self.parse_statement(rule))),
                Rule::decl => items.push(BlockItem::Declaration(self.parse_decl(rule))),
                _ => unimplemented!(),
            }
        }

        Block {
            items,
            span: self.get_span(rules.as_span().clone()),
        }
    }

    pub fn parse_type(&self, rules: Pair<Rule>) -> Type {
        let mut rules_iter = rules.clone().into_inner();

        let vtype_enum = rules_iter.next().unwrap();

        let vty_enum = match vtype_enum.as_str() {
            "u64" => TypeEnum::U64,
            "u32" => TypeEnum::U32,
            "u16" => TypeEnum::U16,
            "u8" => TypeEnum::U8,
            "i64" => TypeEnum::I64,
            "i32" => TypeEnum::I32,
            "i16" => TypeEnum::I16,
            "i8" => TypeEnum::I8,
            "void" => TypeEnum::Void,
            _ => panic!("Unkown type {}!", vtype_enum.as_str()),
        };

        let mut star_cnt = 0usize;
        while let Some(_) = rules_iter.next() {
            star_cnt += 1;
        }
        Type {
            ty: vty_enum,
            star: star_cnt,
            span: self.get_span(rules.as_span().clone()),
        }
    }
}

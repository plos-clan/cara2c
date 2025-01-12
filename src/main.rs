#![no_std]
#![allow(dead_code)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "compile_c")]
mod c_compile;

extern crate alloc;

mod ast;
mod backend;
mod cgen;
mod parser;

use std::fs::File;

use alloc::string::{String, ToString};
use backend::Codegen;

#[cfg(feature = "std")]
use {argh::FromArgs, std::borrow::ToOwned, std::format, std::vec, std::vec::Vec};

#[cfg(feature = "std")]
#[derive(FromArgs)]
#[argh(description = "Cara compiler")]
struct CCPMArgs {
    #[argh(option, short = 'i')]
    #[argh(description = "the input source code")]
    input_source: String,

    #[argh(option, short = 'o')]
    #[argh(description = "the output file")]
    output_source: String,
}

fn main() {
    #[cfg(feature = "std")]
    let args = argh::from_env::<CCPMArgs>();
    #[cfg(feature = "std")]
    let code = {
        let mut input = File::open(args.input_source).unwrap();
        let mut code = String::new();
        std::io::Read::read_to_string(&mut input, &mut code).unwrap();
        code
    };

    #[cfg(not(feature = "std"))]
    let code = r#"
    const main = fn () u64 {
        return 1+1-2;
    };

    "#;

    let parser = parser::CParser::new(code.to_string(), "test.cara".to_string());
    let ast = parser.parse();

    let context = backend::CodegenContext::new();
    let context = alloc::sync::Arc::new(spin::RwLock::new(context));
    ast.codegen(context.clone()).unwrap();

    let context = context.read();
    if context.errors.len() > 0 {
        for error in context.errors.iter() {
            crate::error!("{}", error);
        }
        return;
    }

    #[cfg(feature = "std")]
    {
        use std::io::Write;
        writeln!(
            File::create(args.output_source.clone()).unwrap(),
            "{}",
            context.c_program
        )
        .unwrap();

        #[cfg(feature = "compile_c")]
        {
            use c_compile::*;
            use std::path::PathBuf;

            let mut compiler_settings = CompilerSettings::new();
            compiler_settings.set_compiler_type(CCompilerType::GCC);
            compiler_settings.compile(
                PathBuf::from(args.output_source.clone()),
                PathBuf::from("test.o"),
            );

            let mut linker_settings = LinkerSettings::new();
            linker_settings.set_linker_type(LinkerType::Mold);
            linker_settings.link(
                vec![PathBuf::from("test.o"), PathBuf::from("lib/libcrrt.a")],
                PathBuf::from("test"),
            );
        }
    }
}

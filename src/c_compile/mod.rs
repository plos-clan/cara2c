use std::{path::PathBuf, process::Command, vec::Vec};

pub enum CCompilerType {
    GCC,
    Clang,
    Unknown,
}

pub struct CompilerSettings {
    pub path: Option<PathBuf>,
    pub compiler_type: CCompilerType,
}

impl CompilerSettings {
    pub fn new() -> Self {
        Self {
            path: None,
            compiler_type: CCompilerType::Unknown,
        }
    }

    pub fn set_path(&mut self) -> &mut Self {
        self.path = Some(PathBuf::from("/usr/bin/gcc"));
        self
    }

    pub fn set_compiler_type(&mut self, compiler_type: CCompilerType) -> &mut Self {
        self.compiler_type = compiler_type;
        self
    }

    pub fn compile(&self, file: PathBuf, output: PathBuf) {
        let compiler_path = if let Some(path) = &self.path {
            path.clone()
        } else {
            match self.compiler_type {
                CCompilerType::GCC => PathBuf::from("gcc"),
                CCompilerType::Clang => PathBuf::from("clang"),
                CCompilerType::Unknown => {
                    panic!("No Compiler!");
                }
            }
        };

        let mut command = Command::new(compiler_path);
        command.arg(file);
        command.arg("-o");
        command.arg(output);
        command.arg("-c");
        command.arg("-fno-stack-protector").arg("-nostdlib").arg("-ffreestanding").arg("-fno-builtin");

        std::println!("CC: {:?}", command);

        let mut child = command.spawn().unwrap();
        let exit_code = child.wait().unwrap();
        assert!(exit_code.success());
    }
}

pub enum LinkerType {
    Ld,
    Lld,
    Mold,
    Unknown,
}

pub struct LinkerSettings {
    pub path: Option<PathBuf>,
    pub linker_type: LinkerType,
}

impl LinkerSettings {
    pub fn new() -> Self {
        Self {
            path: None,
            linker_type: LinkerType::Unknown,
        }
    }

    pub fn set_path(&mut self, path: PathBuf) -> &mut Self {
        self.path = Some(path);
        self
    }

    pub fn set_linker_type(&mut self, linker_type: LinkerType) -> &mut Self {
        self.linker_type = linker_type;
        self
    }

    pub fn link(&self, files: Vec<PathBuf>, output: PathBuf) {
        let linker_path = if let Some(path) = &self.path {
            path.clone()
        } else {
            match self.linker_type {
                LinkerType::Ld => PathBuf::from("ld"),
                LinkerType::Lld => PathBuf::from("lld"),
                LinkerType::Mold => PathBuf::from("mold"),
                LinkerType::Unknown => {
                    panic!("No Linker!");
                }
            }
        };

        let mut command = Command::new(linker_path);
        for file in files {
            command.arg(file);
        }
        command.arg("-o");
        command.arg(output);

        std::println!("LD: {:?}", command);

        let mut child = command.spawn().unwrap();
        let exit_code = child.wait().unwrap();
        assert!(exit_code.success());
    }
}

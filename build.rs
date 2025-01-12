use std::{fs::File, process::Command};

fn main() {
    let mut command = Command::new("xmake");
    command.current_dir("crrt");
    command.arg("build");

    let mut child = command.spawn().unwrap();
    let exit_code = child.wait().unwrap();
    assert!(exit_code.success());

    let mut source = File::open("crrt/build/linux/x86_64/release/libcrrt.a").unwrap();
    let mut dest = File::create("lib/libcrrt.a").unwrap();
    std::io::copy(&mut source, &mut dest).unwrap();
}

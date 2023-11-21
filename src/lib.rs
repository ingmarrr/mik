use std::fs::File;
use std::io::Write;
use std::process::Command;

pub mod err;
pub mod gen;
pub mod ir;
pub mod ir2;
#[cfg(test)]
pub mod tests;

pub fn gen() -> String {
    let buf = r#"
.global _start
.align 2

_start: mov	X0, #1
	adr	X1, helloworld
	mov	X2, #13
	mov	X16, #4
	svc	#0x80

	mov     X0, #0
	mov     X16, #1
	svc     #0x80

helloworld:      .ascii  "Hello World!\n"
        "#;
    buf.to_string()
}

pub fn write(path: &str, asm: &str) {
    let asm_path = format!("{}.s", path);
    let mut file = File::create(&asm_path).unwrap();
    file.write_all(asm.as_bytes()).unwrap();
}

pub fn run(path: &str) -> Result<(), err::RunErr> {
    let asm_path = format!("{}.s", path);
    let object_path = format!("{}.o", path);

    // as -o <name>.o <name>.s
    Command::new("as")
        .arg("-o")
        .arg(&object_path)
        .arg(&asm_path)
        .output()?;

    // ld -e _start -l System -syslibroot `xcrun -sdk macosx --show-sdk-path` -arch arm64 -o <name> <name>.o
    Command::new("ld")
        .arg("-e")
        .arg("_start")
        .arg("-l")
        .arg("System")
        .arg("-syslibroot")
        .arg("/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk")
        .arg("-arch")
        .arg("arm64")
        .arg("-o")
        .arg(path)
        .arg(&object_path)
        .output()?;

    // rm <name>.s <name>
    // Command::new("rm").arg(&asm_path).arg(&path).output()?;

    // rm <name>.o <name>
    // Command::new("rm").arg(&object_path).arg(&path).output()?;

    // ./<name>
    let to_run = format!("./{}", path);
    let out = Command::new(to_run).output()?;
    println!("RESULT: {}", String::from_utf8_lossy(&out.stdout));

    Ok(())
}

use crate::ir2::{self};

const INDENT: &str = "    ";

pub trait GenAsm {
    fn gen_asm(&self) -> String;
}

impl GenAsm for ir2::Func {
    fn gen_asm(&self) -> String {
        let mut asm = format!(".global {}\n", self.name);
        asm.push_str(&format!(".align 2\n"));
        asm.push_str(&format!("{}:\n", self.name));

        for instr in &self.body {
            asm.push_str(INDENT);
            asm.push_str(&instr.gen_asm());
            asm.push('\n');
        }
        asm
    }
}

impl GenAsm for ir2::Expr {
    fn gen_asm(&self) -> String {
        match self {
            ir2::Expr::Imm(imm) => format!("#{}", imm),
            ir2::Expr::Reg(reg) => reg.gen_asm(),
        }
    }
}

impl GenAsm for ir2::Reg {
    fn gen_asm(&self) -> String {
        match self {
            ir2::Reg::R(r) => format!("X{}", r),
            ir2::Reg::Sp => "SP".to_string(),
        }
    }
}

impl GenAsm for ir2::Dest {
    fn gen_asm(&self) -> String {
        match self {
            ir2::Dest::Var(name) => name.clone(),
            ir2::Dest::Reg(reg) => reg.gen_asm(),
        }
    }
}

impl GenAsm for ir2::Instr {
    fn gen_asm(&self) -> String {
        match self {
            ir2::Instr::Add { dest, src1, src2 } => {
                format!(
                    "add {}, {}, {}",
                    dest.gen_asm(),
                    src1.gen_asm(),
                    src2.gen_asm()
                )
            }
            ir2::Instr::Sub { dest, src1, src2 } => {
                format!(
                    "sub {}, {}, {}",
                    dest.gen_asm(),
                    src1.gen_asm(),
                    src2.gen_asm()
                )
            }
        }
    }
}

impl GenAsm for Vec<ir2::Instr> {
    fn gen_asm(&self) -> String {
        let mut asm = String::new();
        for instr in self {
            asm.push_str(&instr.gen_asm());
            asm.push('\n');
        }
        asm
    }
}

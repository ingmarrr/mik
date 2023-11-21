pub enum Reg {
    R(usize),
    Sp,
}

pub enum Dest {
    Var(String),
    Reg(Reg),
}

pub enum Expr {
    Imm(i64),
    Reg(Reg),
}

pub enum Instr {
    Add { dest: Dest, src1: Expr, src2: Expr },
    Sub { dest: Dest, src1: Expr, src2: Expr },
}

pub struct Func {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Instr>,
}

use crate::{gen::GenAsm, ir2};

#[test]
fn t_func() {
    let ir_src = ir2::Func {
        name: "main".to_string(),
        args: vec![],
        body: vec![],
    };
    let act = ir_src.gen_asm();
    let exp = r#".global main
.align 2
main:
"#;
    assert_eq!(act, exp);
}

#[test]
fn t_add_imm() {
    let ir_src = ir2::Instr::Add {
        dest: ir2::Dest::Reg(ir2::Reg::R(0)),
        src1: ir2::Expr::Imm(35),
        src2: ir2::Expr::Imm(34),
    };
    let act = ir_src.gen_asm();
    let exp = "add X0, #35, #34";
    assert_eq!(act, exp);
}

#[test]
fn t_add_reg() {
    let ir_src = ir2::Instr::Add {
        dest: ir2::Dest::Reg(ir2::Reg::R(0)),
        src1: ir2::Expr::Reg(ir2::Reg::R(1)),
        src2: ir2::Expr::Imm(35),
    };
    let act = ir_src.gen_asm();
    let exp = "add X0, X1, #35";
    assert_eq!(act, exp);
}

#[test]
fn t_sub_imm() {
    let ir_src = ir2::Instr::Sub {
        dest: ir2::Dest::Reg(ir2::Reg::R(0)),
        src1: ir2::Expr::Imm(500),
        src2: ir2::Expr::Imm(80),
    };
    let act = ir_src.gen_asm();
    let exp = "sub X0, #500, #80";
    assert_eq!(act, exp);
}

#[test]
fn t_sub_reg() {
    let ir_src = ir2::Instr::Sub {
        dest: ir2::Dest::Reg(ir2::Reg::R(0)),
        src1: ir2::Expr::Reg(ir2::Reg::R(1)),
        src2: ir2::Expr::Imm(80),
    };
    let act = ir_src.gen_asm();
    let exp = "sub X0, X1, #80";
    assert_eq!(act, exp);
}

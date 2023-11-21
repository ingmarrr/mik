pub trait DumpIr {
    fn dump_ir(&self) -> String;
}

pub trait DumpAsm {
    fn dump_asm(&self) -> String;
}

pub enum Reg {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    Iota,
    Sp,
    Fp,
    Lr,
    Pc,
}

impl Into<Reg> for &str {
    fn into(self) -> Reg {
        match self {
            "X0" => Reg::X0,
            "X1" => Reg::X1,
            "X2" => Reg::X2,
            "X3" => Reg::X3,
            "X4" => Reg::X4,
            "X5" => Reg::X5,
            "X6" => Reg::X6,
            "X7" => Reg::X7,
            "X8" => Reg::X8,
            "X9" => Reg::X9,
            "X10" => Reg::X10,
            "X11" => Reg::X11,
            "X12" => Reg::X12,
            "X13" => Reg::X13,
            "X14" => Reg::X14,
            "X15" => Reg::X15,
            "X16" => Reg::X16,
            "XIota" => Reg::Iota,
            "Sp" => Reg::Sp,
            "Fp" => Reg::Fp,
            "Lr" => Reg::Lr,
            "Pc" => Reg::Pc,
            _ => panic!("Invalid register: {}", self),
        }
    }
}

impl std::str::FromStr for Reg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X0" => Ok(Reg::X0),
            "X1" => Ok(Reg::X1),
            "X2" => Ok(Reg::X2),
            "X3" => Ok(Reg::X3),
            "X4" => Ok(Reg::X4),
            "X5" => Ok(Reg::X5),
            "X6" => Ok(Reg::X6),
            "X7" => Ok(Reg::X7),
            "X8" => Ok(Reg::X8),
            "X9" => Ok(Reg::X9),
            "X10" => Ok(Reg::X10),
            "X11" => Ok(Reg::X11),
            "X12" => Ok(Reg::X12),
            "X13" => Ok(Reg::X13),
            "X14" => Ok(Reg::X14),
            "X15" => Ok(Reg::X15),
            "X16" => Ok(Reg::X16),
            "XIota" => Ok(Reg::Iota),
            "Sp" => Ok(Reg::Sp),
            "Fp" => Ok(Reg::Fp),
            "Lr" => Ok(Reg::Lr),
            "Pc" => Ok(Reg::Pc),
            _ => Err(format!("Invalid register: {}", s)),
        }
    }
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::X0 => write!(f, "X0"),
            Reg::X1 => write!(f, "X1"),
            Reg::X2 => write!(f, "X2"),
            Reg::X3 => write!(f, "X3"),
            Reg::X4 => write!(f, "X4"),
            Reg::X5 => write!(f, "X5"),
            Reg::X6 => write!(f, "X6"),
            Reg::X7 => write!(f, "X7"),
            Reg::X8 => write!(f, "X8"),
            Reg::X9 => write!(f, "X9"),
            Reg::X10 => write!(f, "X10"),
            Reg::X11 => write!(f, "X11"),
            Reg::X12 => write!(f, "X12"),
            Reg::X13 => write!(f, "X13"),
            Reg::X14 => write!(f, "X14"),
            Reg::X15 => write!(f, "X15"),
            Reg::X16 => write!(f, "X16"),
            Reg::Iota => write!(f, "XIota"),
            Reg::Sp => write!(f, "Sp"),
            Reg::Fp => write!(f, "Fp"),
            Reg::Lr => write!(f, "Lr"),
            Reg::Pc => write!(f, "Pc"),
        }
    }
}

pub enum Decl {
    /// Functions are defined and called at runtime.
    /// They are linked against at compile time.
    ///
    /// Example:
    /// ```ir
    /// define i64 @main() {
    /// entry:
    ///   %0 = call i64 @foo()
    ///   ret i64 %0
    /// }
    /// ```
    Func(Func),
    /// Externs are functions that are defined in another module.
    /// They are linked against at compile time.
    ///
    /// Example:
    /// ```ir
    /// extern i64 @write(i32, i8*, i64)
    /// ```
    Extern(Extern),
    /// Static Variables are defined and initialized at compile time.
    /// They are stored in the data section of the binary.
    ///
    /// Static variables can be global or local. Global vairables are only
    /// accessible within the module they are defined in and cannot be accessed
    /// or linked against from other modules.
    ///
    /// Example:
    /// ```ir
    /// @foo = global i64 69
    /// @bar = local i64 420
    ///
    /// ```
    Static(Static),
}

/// Externs are functions that are defined in another module.
/// They are linked against at compile time.
///
/// Example:
/// ```ir
/// extern i64 @write(i32, i8*, i64)
/// ```
pub struct Extern {
    _name: String,
    _ty: Ty,
    _args: Vec<String>,
}

/// Static Variables are defined and initialized at compile time.
/// They are stored in the data section of the binary.
///
/// Static variables can be global or local. Global vairables are only
/// accessible within the module they are defined in and cannot be accessed
/// or linked against from other modules.
///
/// Example:
/// ```ir
/// @foo = global i64 69
/// @bar = local i64 420
///
/// ```
pub struct Static {
    name: String,
    _scope: Scope,
    ty: Ty,
    val: String,
    // ... more to come ...
}

pub enum Scope {
    Global,
    Local(String),
}

pub enum Ty {
    I64,
    I32,
    I16,
    I8,
    U64,
    U32,
    U16,
    U8,
    F64,
    F32,
    Ptr(Box<Ty>),
    Arr(Box<Ty>, usize),
    Void,
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dump_ir())
    }
}

pub enum Expr {
    Reg(Reg),
    Imm { ty: Ty, val: i64 },
    Global(String),
}

pub enum Instr {
    Const {
        reg: Reg,
        ty: Ty,
        val: i64,
    },
    Call {
        name: String,
        ty: String,
        args: Vec<Expr>,
    },
    AddrOf {
        dst: Reg,
        ty: Ty,
        ptr: String,
        ix: String,
        offset: String,
    },
    Push {
        /// Space in bytes - meaning allocation space for 1 item is 8 bytes
        space: usize,
        expr: Expr,
    },
    Add {
        dst: Reg,
        ty: Ty,
        first: Expr,
        second: Expr,
    },

    /// This is only temporary until I figure out linking
    Syscall {
        imm: String,
    },
}

pub struct Func {
    pub name: String,
    pub ty: Ty,
    pub params: Vec<(String, Ty)>,
    pub body: Vec<Instr>,
}

pub struct Module {
    funcs: Vec<Func>,
    globals: Vec<Static>,
    _externs: Vec<Extern>,
}

impl DumpIr for Ty {
    fn dump_ir(&self) -> String {
        match self {
            Ty::I64 => "i64".to_string(),
            Ty::I32 => "i32".to_string(),
            Ty::I16 => "i16".to_string(),
            Ty::I8 => "i8".to_string(),
            Ty::U64 => "u64".to_string(),
            Ty::U32 => "u32".to_string(),
            Ty::U16 => "u16".to_string(),
            Ty::U8 => "u8".to_string(),
            Ty::F64 => "f64".to_string(),
            Ty::F32 => "f32".to_string(),
            Ty::Ptr(ty) => format!("{}*", ty.dump_ir()),
            Ty::Arr(ty, len) => format!("[{}; {}]", ty.dump_ir(), len),
            Ty::Void => "void".to_string(),
        }
    }
}

impl DumpIr for Expr {
    fn dump_ir(&self) -> String {
        match self {
            Expr::Reg(reg) => reg.to_string(),
            Expr::Imm { ty, val } => format!("{} {}", ty.dump_ir(), val),
            Expr::Global(global) => global.clone(),
        }
    }
}

impl DumpIr for Instr {
    fn dump_ir(&self) -> String {
        match self {
            Instr::Const { reg, ty, val } => format!("{} = const {} {}", reg, ty, val),
            Instr::Call { name, ty, args } => {
                let mut buf = String::new();
                buf.push_str(&format!("call {} @{} (", ty, name));
                for arg in args {
                    buf.push_str(&format!(", {}", arg.dump_ir()));
                }
                buf.push_str(")");
                buf
            }
            Instr::AddrOf {
                dst: reg,
                ty,
                ptr,
                ix,
                offset,
            } => {
                format!(
                    "{} = addrof {}, {}, {}, {}",
                    reg,
                    ty.dump_ir(),
                    ptr,
                    ix,
                    offset
                )
            }
            Instr::Push { .. } => todo!(),
            Instr::Add { .. } => todo!(),

            Instr::Syscall { imm } => {
                format!("syscall #{}", imm)
            }
        }
    }
}

impl DumpIr for Func {
    fn dump_ir(&self) -> String {
        let mut buf = String::new();
        buf.push_str(&format!("define {} @{}(", self.ty, self.name));
        for (i, (arg, ty)) in self.params.iter().enumerate() {
            if i != 0 {
                buf.push_str(", ");
            }
            buf.push_str(&format!("{} %{}", arg, ty));
        }
        buf.push_str(") {\nentry:\n");
        for instr in &self.body {
            buf.push_str(&format!("    {}\n", instr.dump_ir()));
        }
        buf.push_str("}\n");
        buf
    }
}

impl DumpIr for Module {
    fn dump_ir(&self) -> String {
        let mut buf = String::new();

        for var in &self.globals {
            buf.push_str(&format!(
                "@{} = global {} {}\n",
                var.name,
                var.ty.dump_ir(),
                var.val
            ));
        }

        for func in &self.funcs {
            buf.push_str(&func.dump_ir());
        }
        buf
    }
}

impl DumpAsm for Instr {
    fn dump_asm(&self) -> String {
        match self {
            Instr::Const { reg, val, .. } => format!("mov {}, #{}", reg, val),
            Instr::Call { name, args, .. } => {
                let mut buf = String::new();
                buf.push_str(&format!("mov X0, #{}\n", args.len()));
                for (i, arg) in args.iter().enumerate() {
                    buf.push_str(&format!("adr X{}, {}\n", i + 1, arg.dump_ir()));
                }
                buf.push_str(&format!("blr {}\n", name));
                buf
            }
            Instr::Syscall { imm } => format!("svc #{}", imm),
            Instr::AddrOf { dst: reg, ptr, .. } => format!("adr {}, {}", reg, ptr),
            Instr::Push { space, expr } => {
                let mut buf = String::new();
                buf.push_str(&format!("sub sp, sp, #{}\n", space * 8));
                buf.push_str(&format!("str {}, [sp, #0]\n", expr.dump_ir()));
                buf
            }
            Instr::Add { .. } => todo!(),
        }
    }
}

impl DumpAsm for Module {
    fn dump_asm(&self) -> String {
        let mut buf = String::new();

        for var in &self.globals {
            buf.push_str(&format!(".global {}\n", var.name));
            buf.push_str(&format!(".align 2\n"));
            buf.push_str(&format!("{}: .ascii {}\n", var.name, var.val));
        }

        for func in &self.funcs {
            buf.push_str(&format!(".global {}\n", func.name));
            buf.push_str(&format!(".align 2\n"));
            buf.push_str(&format!("{}:\n", func.name));
            for instr in &func.body {
                buf.push_str(&format!("    {}\n", instr.dump_asm()));
            }
        }
        buf
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn helloworld_ir() {
//         let func = Func {
//             name: "main".into(),
//             ty: Ty::I64,
//             params: Vec::new(),
//             body: vec![
//                 Instr::Const {
//                     reg: Reg::X0,
//                     ty: Ty::I64,
//                     val: 1,
//                 },
//                 Instr::AddrOf {
//                     dst: Reg::X1,
//                     ty: Ty::I64,
//                     ptr: "@helloworld".into(),
//                     ix: "i64 0".into(),
//                     offset: "i64 0".into(),
//                 },
//                 Instr::Const {
//                     reg: Reg::X2,
//                     ty: Ty::I64,
//                     val: 13,
//                 },
//                 Instr::Const {
//                     reg: Reg::X16,
//                     ty: Ty::I64,
//                     val: 4,
//                 },
//                 Instr::Syscall { imm: "0x80".into() },
//                 Instr::Const {
//                     reg: Reg::X0,
//                     ty: Ty::I64,
//                     val: 69,
//                 },
//                 Instr::Const {
//                     reg: Reg::X16,
//                     ty: Ty::I64,
//                     val: 1,
//                 },
//                 Instr::Syscall { imm: "0x80".into() },
//             ],
//         };

//         let module = Module {
//             funcs: vec![func],
//             globals: vec![Static {
//                 name: "helloworld".into(),
//                 scope: Scope::Global,
//                 ty: Ty::Arr(Box::new(Ty::I8), 14),
//                 val: r#""Hello World!\n""#.into(),
//             }],
//             _externs: vec![Extern {
//                 name: "write".into(),
//                 ty: Ty::Void,
//                 args: vec!["i64".into(), "i8*".into(), "i64".into()],
//             }],
//         };

//         let expected = r#"@helloworld = global [i8; 14] "Hello World!\n"
// define i64 @main() {
// entry:
//     X0 = const i64 1
//     X1 = addrof i64, @helloworld, i32 0, i32 0
//     X2 = const i64 13
//     X16 = const i64 4
//     syscall #0x80
//     X0 = const i64 69
//     X16 = const i64 1
//     syscall #0x80
// }
// "#;

//         println!("Actual: {}", module.dump_ir());
//         println!("Expected: {}", expected);

//         assert_eq!(module.dump_ir(), expected);
//     }

//     #[test]
//     fn helloworld_asm() {
//         let func = Func {
//             name: "main".into(),
//             ty: Ty::I64,
//             params: Vec::new(),
//             body: vec![
//                 Instr::Const {
//                     reg: Reg::X0,
//                     ty: Ty::I64,
//                     val: 1,
//                 },
//                 Instr::AddrOf {
//                     dst: Reg::X1,
//                     ty: Ty::I64,
//                     ptr: "helloworld".into(),
//                     ix: "i64 0".into(),
//                     offset: "i64 0".into(),
//                 },
//                 Instr::Const {
//                     reg: Reg::X2,
//                     ty: Ty::I64,
//                     val: 13,
//                 },
//                 Instr::Const {
//                     reg: Reg::X16,
//                     ty: Ty::I64,
//                     val: 4,
//                 },
//                 Instr::Syscall { imm: "0x80".into() },
//                 Instr::Const {
//                     reg: Reg::X0,
//                     ty: Ty::I64,
//                     val: 69,
//                 },
//                 Instr::Const {
//                     reg: Reg::X16,
//                     ty: Ty::I64,
//                     val: 1,
//                 },
//                 Instr::Syscall { imm: "0x80".into() },
//             ],
//         };

//         let module = Module {
//             funcs: vec![func],
//             globals: vec![Static {
//                 name: "helloworld".into(),
//                 scope: Scope::Global,
//                 ty: Ty::Arr(Box::new(Ty::I8), 14),
//                 val: r#""Hello World!\n""#.into(),
//             }],
//             _externs: vec![Extern {
//                 name: "write".into(),
//                 ty: Ty::Void,
//                 args: vec!["i64".into(), "i8*".into(), "i64".into()],
//             }],
//         };

//         let expected = r#".global helloworld
// .align 2
// helloworld: .ascii "Hello World!\n"
// .global main
// .align 2
// main:
//     mov X0, #1
//     adr X1, helloworld
//     mov X2, #13
//     mov X16, #4
//     svc #0x80
//     mov X0, #69
//     mov X16, #1
//     svc #0x80
// "#;

//         println!("Actual: {}", module.dump_asm());
//         println!("Expected: {}", expected);

//         assert_eq!(module.dump_asm(), expected);
//     }
// }

use crate::prelude::*;

pub trait CodeGeneration {
    fn generate_code(&self) -> String;
}

impl CodeGeneration for Ast {
    fn generate_code(&self) -> String {
        match self {
            Value(src, val) => format!("{} {}", src.generate_code(), val.to_string()),
            BinOp(op, left, right) => match (&**left, &**right) {
                (Value(_, _), _) => format!(
                    "{}\nSW\n{}\n{}",
                    right.generate_code(),
                    left.generate_code(),
                    op.generate_code()
                ),
                (_, Value(_, _)) => format!(
                    "{}\nSW\n{}\n{}{}",
                    left.generate_code(),
                    right.generate_code(),
                    if *op == Sub || *op == Div { "SW\n" } else { "" },
                    op.generate_code()
                ),
                _ => format!(
                    "{}\nPU\n{}\nSW\nPO\n{}",
                    left.generate_code(),
                    right.generate_code(),
                    op.generate_code()
                ),
            },
        }
    }
}

impl CodeGeneration for Operator {
    fn generate_code(&self) -> String {
        match self {
            Add => "AD",
            Sub => "SU",
            Mul => "MU",
            Div => "DI",
        }
        .to_string()
    }
}

impl CodeGeneration for Source {
    fn generate_code(&self) -> String {
        match self {
            Arg => "AR",
            Imm => "IM",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArchSim;

    #[test]
    fn generate_code() {
        // let prog = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";
        let ast = binop!(/, binop!(-, binop!(+, binop!(*, imm!(6), arg!(0)), binop!(*, imm!(5), arg!(1))), binop!(*, imm!(3), arg!(2))), imm!(8));

        let code = ast.generate_code();

        assert_simulate!(&code, [4, 0, 0], 3, "prog(4,0,0) == 3");
        assert_simulate!(&code, [4, 8, 0], 8, "prog(4,8,0) == 8");
        assert_simulate!(&code, [4, 8, 16], 2, "prog(4,8,6) == 2");
    }
}

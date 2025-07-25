use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Optimizer {
    original: Ast,
    optimized: Ast,
}

impl Display for Optimizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Optimizer state:\noriginal: {}\noptimized: {}",
            self.original, self.optimized
        )
    }
}

impl Optimizer {
    pub fn optimize(&mut self, ast: Ast) -> Ast {
        self.original = ast.clone();
        self.optimized = Self::optimize_recursion(ast);

        self.optimized.clone()
    }

    fn optimize_recursion(ast: Ast) -> Ast {
        match ast {
            Value(src, val) => Value(src, val),
            BinOp(op, left, right) => {
                let l = Self::optimize_recursion(*left);
                let r = Self::optimize_recursion(*right);

                if let Value(Imm, l) = l {
                    if let Value(Imm, r) = r {
                        let result = match op {
                            Add => l + r,
                            Sub => l - r,
                            Mul => l * r,
                            Div => l / r,
                        };
                        return imm!(result);
                    }
                }

                Ast::binop(op, l, r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimize() {
        // let prog = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";
        let ast = binop!(/, binop!(-, binop!(+, binop!(*, binop!(*, imm!(2), imm!(3)), arg!(0)), binop!(*, imm!(5), arg!(1))), binop!(*, imm!(3), arg!(2))), binop!(+, binop!(+, imm!(1), imm!(3)), binop!(*, imm!(2), imm!(2))));
        let expected = binop!(/, binop!(-, binop!(+, binop!(*, imm!(6), arg!(0)), binop!(*, imm!(5), arg!(1))), binop!(*, imm!(3), arg!(2))), imm!(8));

        let mut optimizer = Optimizer::default();
        let actual = optimizer.optimize(ast);

        assert_eq!(actual, expected, "Optimize");
    }
}

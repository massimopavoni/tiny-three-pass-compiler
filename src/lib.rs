mod ast;
mod generator;
mod optimizer;
mod parser;
mod prelude;

use std::fmt::Display;

use crate::{generator::CodeGeneration, optimizer::Optimizer, parser::Parser};

#[derive(Debug, Default)]
pub struct Compiler<'a> {
    parser: Parser<'a>,
    optimizer: Optimizer,
    assembly: String,
}

impl Display for Compiler<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Compiler\n\n{}\n\n{}\n\nassembly:\n{}",
            self.parser, self.optimizer, self.assembly
        )
    }
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(&mut self, program: &'a str) -> String {
        self.assembly = self
            .optimizer
            .optimize(self.parser.parse(program))
            .generate_code();

        self.assembly.clone()
    }
}

pub struct ArchSim {}

impl ArchSim {
    pub fn simulate(assembly: &String, argv: Vec<i32>) -> i32 {
        let (mut r0, mut r1) = (0, 0);
        let mut stack = vec![];
        let num = |opt: Option<&str>| opt.unwrap().parse::<i32>().unwrap();

        for instruction in assembly.lines() {
            let mut args = instruction.split_whitespace();
            match args.next() {
                Some("IM") => r0 = num(args.next()),
                Some("AR") => r0 = argv[num(args.next()) as usize],
                Some("SW") => (r0, r1) = (r1, r0),
                Some("PU") => stack.push(r0),
                Some("PO") => r0 = stack.pop().unwrap(),
                Some("AD") => r0 += r1,
                Some("SU") => r0 -= r1,
                Some("MU") => r0 *= r1,
                Some("DI") => r0 /= r1,
                _ => unreachable!("Invalid instruction encountered"),
            }
        }

        r0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_of_operations() {
        let prog = "[ x y z ] x - y - z + 10 / 5 / 2 - 7 / 1 / 7";

        let mut compiler = Compiler::new();
        let code = compiler.compile(prog);

        assert!(!code.is_empty(), "No code was produced");
        assert_simulate!(&code, [5, 4, 1], 0, "{} @ [5,4,1]", prog);
    }
}

fn main() {
    println!("{:0>16b}", Instruction::A(2).asm());
    println!(
        "{:0>16b}",
        Instruction::C(
            Dest {
                d: true,
                m: false,
                a: false
            },
            Comp {
                expr: String::from("A")
            },
            Jump {
                lt: false,
                eq: false,
                gt: false
            },
        )
        .asm(),
    );
}

// @2
// D=A
// @3
// D=D+A
// @0
// M=D

enum Instruction {
    A(u16),
    C(Dest, Comp, Jump),
}

impl Instruction {
    fn asm(&self) -> u16 {
        match self {
            Instruction::A(value) => *value,
            Instruction::C(d, c, j) => {
                (1 << 15) + (1 << 14) + (1 << 13) + (c.asm() << 6) + (d.asm() << 3) + j.asm()
            }
        }
    }
}

struct Dest {
    a: bool,
    m: bool,
    d: bool,
}

impl Dest {
    fn asm(&self) -> u16 {
        let mut result: u16 = 0;
        if self.m {
            result += 1;
        }
        if self.d {
            result += 1 << 1;
        }
        if self.a {
            result += 1 << 2;
        }
        result
    }

    fn parse(exp: &str) -> Dest {
        Dest {
            a: exp.contains("A"),
            m: exp.contains("M"),
            d: exp.contains("D"),
        }
    }
}

struct Jump {
    lt: bool,
    eq: bool,
    gt: bool,
}

impl Jump {
    fn asm(&self) -> u16 {
        0 // TODO imple
    }
}

struct Comp {
    expr: String, // FIXME: more fine-grained modeling
}

impl Comp {
    fn asm(&self) -> u16 {
        0 // TODO impl
    }
}

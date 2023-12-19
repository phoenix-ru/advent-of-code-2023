use std::ops::Neg;

use rustc_hash::FxHashMap;

pub struct Clause<'s> {
    pub left: BinOpLeft,
    pub op: BinOpKind,
    pub right: u16,
    pub action: Action<'s>,
}

pub enum BinOpLeft {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy)]
pub enum BinOpKind {
    LessThan,
    GreaterThan,
}

impl Neg for BinOpKind {
    type Output = BinOpKind;

    fn neg(self) -> Self::Output {
        match self {
            BinOpKind::LessThan => BinOpKind::GreaterThan,
            BinOpKind::GreaterThan => BinOpKind::LessThan,
        }
    }
}

pub enum Action<'s> {
    Accept,
    Reject,
    GoTo(&'s str),
}

pub struct Workshop<'s> {
    pub clauses: Vec<Clause<'s>>,
    pub else_action: Action<'s>,
}

#[derive(Default, Debug)]
pub struct Part {
    pub x: u16,
    pub m: u16,
    pub a: u16,
    pub s: u16,
}

impl Part {
    pub fn sum(&self) -> u32 {
        self.x as u32 + self.m as u32 + self.a as u32 + self.s as u32
    }
}

pub fn parse_input(input: &str) -> (FxHashMap<&str, Workshop>, Vec<Part>) {
    let mut input_iter = input.lines().map(str::trim);

    // e.g. `px{a<2006:qkq,m>2090:A,rfg}`
    let mut workshops = FxHashMap::default();
    while let Some(line) = input_iter.next() {
        if line.is_empty() {
            break;
        }

        let Some(data_start) = line.find('{') else {
            unreachable!()
        };
        let prefix = &line[..data_start];
        let data = &line[data_start..]
            .trim_start_matches('{')
            .trim_end_matches('}');

        let mut clauses = Vec::new();
        let mut else_action: Option<Action> = None;
        for raw in data.split(',') {
            match raw.split_once(':') {
                Some((raw_cond, raw_action)) => {
                    let left = match &raw_cond[..1] {
                        "x" => BinOpLeft::X,
                        "m" => BinOpLeft::M,
                        "a" => BinOpLeft::A,
                        "s" => BinOpLeft::S,
                        _ => unreachable!(),
                    };

                    let op = match &raw_cond[1..2] {
                        ">" => BinOpKind::GreaterThan,
                        "<" => BinOpKind::LessThan,
                        _ => unreachable!(),
                    };

                    let right: u16 = raw_cond[2..].parse().unwrap();

                    let action = match raw_action {
                        "A" => Action::Accept,
                        "R" => Action::Reject,
                        x => Action::GoTo(x),
                    };

                    clauses.push(Clause {
                        left,
                        op,
                        right,
                        action,
                    })
                }
                None => {
                    else_action = Some(match raw {
                        "A" => Action::Accept,
                        "R" => Action::Reject,
                        x => Action::GoTo(x),
                    });
                }
            }
        }

        workshops.insert(
            prefix,
            Workshop {
                clauses,
                else_action: else_action.expect("Must have else action"),
            },
        );
    }

    let mut parts = Vec::new();
    while let Some(line) = input_iter.next() {
        let mut part = Part::default();
        for definition in line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
        {
            let (symbol, value) = definition.split_once('=').unwrap();
            let value: u16 = value.parse().unwrap();
            match symbol {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => {}
            }
        }
        parts.push(part)
    }

    (workshops, parts)
}

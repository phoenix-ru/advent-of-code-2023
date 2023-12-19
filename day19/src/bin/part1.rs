use common::main;
use day19::{Action, BinOpKind, BinOpLeft, parse_input, Part};

main!();

fn solve(input: &str) -> u32 {
    let (workshops, parts) = parse_input(input);

    let mut total = 0;
    for part in parts {
        let mut curr_workshop = workshops.get("in").expect("in should exist");

        let Part { x, m, a, s } = part;

        loop {
            let found = curr_workshop
                .clauses
                .iter()
                .find_map(|clause| {
                    let compared = match clause.left {
                        BinOpLeft::X => x,
                        BinOpLeft::M => m,
                        BinOpLeft::A => a,
                        BinOpLeft::S => s,
                    };

                    let is_true = match clause.op {
                        BinOpKind::LessThan => compared < clause.right,
                        BinOpKind::GreaterThan => compared > clause.right,
                    };

                    if is_true {
                        Some(&clause.action)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| &curr_workshop.else_action);

            match found {
                Action::Accept => {
                    // println!("Adding part {:?}", part);
                    total += part.sum();
                    break;
                }
                Action::Reject => {
                    // println!("Rejecting part {:?}", part);
                    break;
                }
                Action::GoTo(workshop) => {
                    curr_workshop = workshops.get(workshop).expect("Should exist")
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            7540,
            solve(
                "px{a<2006:qkq,m>2090:A,rfg}
                 pv{a>1716:R,A}
                 lnx{m>1548:A,A}
                 rfg{s<537:gd,x>2440:R,A}
                 qs{s>3448:A,lnx}
                 qkq{x<1416:A,crn}
                 crn{x>2662:A,R}
                 in{s<1351:px,qqz}
                 qqz{s>2770:qs,m<1801:hdj,R}
                 gd{a>3333:R,R}
                 hdj{m>838:A,pv}
                 
                 {x=787,m=2655,a=1222,s=2876}"
            )
        );

        assert_eq!(
            19114,
            solve(
                "px{a<2006:qkq,m>2090:A,rfg}
                 pv{a>1716:R,A}
                 lnx{m>1548:A,A}
                 rfg{s<537:gd,x>2440:R,A}
                 qs{s>3448:A,lnx}
                 qkq{x<1416:A,crn}
                 crn{x>2662:A,R}
                 in{s<1351:px,qqz}
                 qqz{s>2770:qs,m<1801:hdj,R}
                 gd{a>3333:R,R}
                 hdj{m>838:A,pv}
                 
                 {x=787,m=2655,a=1222,s=2876}
                 {x=1679,m=44,a=2067,s=496}
                 {x=2036,m=264,a=79,s=2244}
                 {x=2461,m=1339,a=466,s=291}
                 {x=2127,m=1623,a=2188,s=1013}"
            )
        );
    }
}

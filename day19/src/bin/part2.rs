use std::ops::Range;

use common::main;
use day19::{parse_input, Workshop, BinOpLeft, BinOpKind, Action};
use rustc_hash::FxHashMap;

main!();

const RANGE_MIN: u16 = 1;
const RANGE_MAX: u16 = 4000;

#[derive(Clone, Debug)]
struct CurrentRange {
    range_x: Range<u16>,
    range_m: Range<u16>,
    range_a: Range<u16>,
    range_s: Range<u16>,
}

fn solve(input: &str) -> usize {
    let (workshops, _parts) = parse_input(input);

    let initial_range = CurrentRange {
        range_x: RANGE_MIN..RANGE_MAX,
        range_m: RANGE_MIN..RANGE_MAX,
        range_a: RANGE_MIN..RANGE_MAX,
        range_s: RANGE_MIN..RANGE_MAX,
    };

    solve_for(&workshops, initial_range, "in")
}

fn solve_for(map: &FxHashMap<&str, Workshop>, mut current_range: CurrentRange, workshop_id: &str) -> usize {
    let mut total = 0;
    let current_workshop = map.get(workshop_id).unwrap();

    for clause in current_workshop.clauses.iter() {
        // Range which would satisfy the condition
        let mut new_range = current_range.clone();

        // Change range to not satisfy the condition
        change_range(&mut current_range, &clause.left, &-clause.op, clause.right, true);

        // And to satisfy
        change_range(&mut new_range, &clause.left, &clause.op, clause.right, false);

        match clause.action {
            Action::Accept => {
                total += range_total(&new_range);
            }
            Action::Reject => {}
            Action::GoTo(new_workshop) => {
                total += solve_for(map, new_range, new_workshop);
            }
        }
    }

    // Add else action
    match current_workshop.else_action {
        Action::Accept => {
            total += range_total(&current_range);
        }
        Action::Reject => {}
        Action::GoTo(new_workshop) => {
            total += solve_for(map, current_range, new_workshop);
        }
    }

    total
}

/// Changes the range to satisfy the condition
#[inline]
fn change_range(range: &mut CurrentRange, what_part: &BinOpLeft, how: &BinOpKind, value: u16, inclusive: bool) {
    let target_subrange = match what_part {
        BinOpLeft::X => &mut range.range_x,
        BinOpLeft::M => &mut range.range_m,
        BinOpLeft::A => &mut range.range_a,
        BinOpLeft::S => &mut range.range_s,
    };

    match how {
        BinOpKind::LessThan => {
            target_subrange.end = if inclusive {
                value
            } else {
                value - 1
            };
        }
        BinOpKind::GreaterThan => {
            target_subrange.start = if inclusive {
                value
            } else {
                value + 1
            };
        }
    }
}

#[inline]
fn range_total(range: &CurrentRange) -> usize {
    (range.range_x.len() + 1) *
    (range.range_m.len() + 1) *
    (range.range_a.len() + 1) *
    (range.range_s.len() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            1*2*3*4,
            solve(
                "in{x>1:R,m>2:R,a>3:R,s>4:R,A}

                 {x=787,m=2655,a=1222,s=2876}"
            )
        );

        assert_eq!(
            (4000 - 2662) * 4000 * 4000 * 4000,
            solve(
                "in{x>2662:A,R}

                 {x=787,m=2655,a=1222,s=2876}"
            )
        );

        assert_eq!(
            167409079868000,
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

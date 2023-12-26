use common::main;
use day20::{cycle, parse_input, GateKind, Tracking};
use num::Integer;

main!();

fn solve(input: &str) -> usize {
    let gates = parse_input(input);

    // Track these nodes
    let track = gates.iter().find_map(|g| {
        if let GateKind::Conjunction(g) = g {
            if g.outs.contains(&u8::MAX) {
                return Some(g.ins);
            }
        }

        None
    });

    let mut tracking = track.map(|track| Tracking {
        track,
        reached_in: vec![],
    });

    let _ = cycle(gates, 1_000_000, tracking.as_mut());

    let tracking = tracking.expect("Should exist");
    let reached_in = tracking
        .reached_in
        .into_iter()
        .collect::<Vec<_>>();

    dbg!(&reached_in);

    let mut lcm = 1;
    for num in reached_in {
        lcm = lcm.lcm(&num);
    }
    lcm
}

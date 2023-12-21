use std::collections::VecDeque;

use rustc_hash::FxHashMap;

const MAX_OUTS: usize = 7;

#[derive(Debug)]
pub enum GateKind {
    Noop,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

#[derive(Debug)]
pub struct FlipFlop {
    pub state: bool,
    // Up to 7 outs
    pub outs: [u8; MAX_OUTS],
}

#[derive(Debug)]
pub struct Conjunction {
    // Up to 13 bits are stored
    pub state: u16,
    pub ins: [u8; 13],
    // Up to 7 outs
    pub outs: [u8; MAX_OUTS],
}

#[derive(Debug)]
pub struct Broadcaster {
    // Up to 7 outs
    pub outs: [u8; MAX_OUTS],
}

pub fn parse_input(input: &str) -> Vec<GateKind> {
    // Build result to take as little space as possible
    let mut gates = Vec::new();
    let mut names_map = FxHashMap::<&str, u8>::default();

    // Index 0 is reserved
    gates.push(GateKind::Noop);

    // First, put the names and types
    for line in input.lines().map(str::trim) {
        let (source, _) = line.split_once(" -> ").unwrap();

        // Type of gate and name
        let typ = &source[..1];
        let name = if source.starts_with("broadcaster") {
            "broadcaster"
        } else {
            &source[1..]
        };

        let new_idx = gates.len();
        names_map.insert(name, new_idx as u8);

        let gate = match typ {
            "b" if source.starts_with("broadcaster") => GateKind::Broadcaster(Broadcaster { outs: [0; 7] }),
            "%" => GateKind::FlipFlop(FlipFlop { state: false, outs: [0; 7] }),
            "&" => GateKind::Conjunction(Conjunction {
                state: 0,
                ins: [0; 13],
                outs: [0; 7],
            }),
            _ => unreachable!(),
        };
        gates.push(gate);
    }

    // Fill outs
    for line in input.lines().map(str::trim) {
        let (source, targets) = line.split_once(" -> ").unwrap();

        // Type of gate and name
        let typ = &source[..1];
        let name = if source.starts_with("broadcaster") {
            "broadcaster"
        } else {
            &source[1..]
        };

        // Max 7 outs
        let mut outs = [0u8; 7];
        for (idx, out_ident) in targets.split(", ").enumerate().take(MAX_OUTS) {
            if let Some(existing_idx) = names_map.get(out_ident) {
                outs[idx] = *existing_idx;
            } else {
                // Ignore non-existing gate
                outs[idx] = u8::MAX;

                // let new_idx = next_idx;
                // next_idx += 1;
                // names_map.insert(out_ident, new_idx);
                // println!("Inserting {out_ident}  [2]");
                // outs[idx] = new_idx;
            }
        }

        if let Some(put_at_idx) = names_map.get(name) {
            let put_at = gates
                .get_mut(*put_at_idx as usize)
                .expect("Should be allocated already");

            *put_at = match typ {
                "b" if source.starts_with("broadcaster") => GateKind::Broadcaster(Broadcaster { outs }),
                "%" => GateKind::FlipFlop(FlipFlop { state: false, outs }),
                "&" => GateKind::Conjunction(Conjunction {
                    state: 0,
                    ins: [0; 13],
                    outs,
                }),
                _ => unreachable!(),
            }
        }
    }

    // Build inputs for `Conjunction` gates
    for gate_idx in 1..gates.len() {
        match gates.get(gate_idx) {
            Some(GateKind::Conjunction(_)) => {
                // Find all the gates referring this one
                let mut ins = [0u8; 13];
                let mut next_in = 0;

                for (inner_gate_idx, gate) in gates.iter().enumerate() {
                    match gate {
                        GateKind::FlipFlop(FlipFlop { outs, .. }) | GateKind::Conjunction(Conjunction { outs, .. }) | GateKind::Broadcaster(Broadcaster { outs }) => {
                            for out in outs.iter() {
                                if *out as usize == gate_idx {
                                    ins[next_in] = inner_gate_idx as u8;
                                    next_in += 1;
                                }
                            }
                        }
                        GateKind::Noop => {}
                    }
                }

                let GateKind::Conjunction(conj) = &mut gates[gate_idx] else { unreachable!() };
                conj.ins = ins;
            }

            _ => {}
        }
    }

    // dbg!(&gates);

    gates
}

pub fn cycle(mut gates: Vec<GateKind>, iterations: u32) -> usize {
    let mut lows = 0;
    let mut highs = 0;
    
    let mut queue = VecDeque::<(usize, usize, bool)>::new();

    // Find broadcaster index
    let broadcaster_idx = gates.iter().position(|gate| matches!(gate, GateKind::Broadcaster(_))).expect("Should have broadcaster") as usize;

    // Cycle
    for _ in 0..iterations {
        // Push button
        queue.push_back((0, broadcaster_idx, false));

        while let Some((pulse_from, pulse_to, pulse_is_hi)) = queue.pop_front() {
            // dbg!(pulse_from, pulse_to, pulse_is_hi);
            if pulse_is_hi {
                highs += 1;
            } else {
                lows += 1;
            }

            macro_rules! push_outs {
                ($where: expr, $pulse: expr) => {
                    for out in $where.iter() {
                        if out == &0 {
                            break;
                        }
                        queue.push_back((pulse_to, *out as usize, $pulse))
                    }
                };
            }

            match gates.get_mut(pulse_to) {
                Some(GateKind::Broadcaster(broadcaster)) => {
                    push_outs!(broadcaster.outs, pulse_is_hi);
                }

                Some(GateKind::FlipFlop(flip_flop)) => {
                    if pulse_is_hi {
                        continue;
                    }

                    flip_flop.state = !flip_flop.state;
                    push_outs!(flip_flop.outs, flip_flop.state);
                }

                Some(GateKind::Conjunction(conj)) => {
                    // Find and set the needed input bit
                    if let Some(bit_idx) = conj.ins.iter().position(|input| *input == pulse_from as u8) {
                        // Set to 0
                        if pulse_is_hi {
                            conj.state |= 1 << bit_idx;
                        } else {
                            conj.state &= !(1 << bit_idx);
                        }
                    }

                    let inputs_total = conj.ins.iter().filter(|&&x| x != 0).count();
                    let all_hi = conj.state == (1 << inputs_total) - 1;
                    push_outs!(conj.outs, !all_hi);
                }

                _ => {
                    // dbg!(pulse_from, pulse_to, pulse_is_hi);
                    // unreachable!()
                }
            }
        }
    }

    dbg!(lows) * dbg!(highs)
}

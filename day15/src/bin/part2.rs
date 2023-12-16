use std::fmt::Debug;

use common::main;
use day15::compute_hash;

main!();

struct Lens<'i> {
    seq: &'i str,
    power: u8,
}

impl Debug for Lens<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{} {}]", self.seq, self.power))
    }
}

fn solve(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = Vec::from_iter(std::iter::repeat_with(Vec::new).take(256));

    for seq in input.trim().split(',') {
        // `-` operation
        if seq.ends_with('-') {
            let label = &seq[..seq.len() - 1];
            let hash = compute_hash(label);
            if let Some(box_) = boxes.get_mut(hash) {
                box_.retain(|v| v.seq != label);
            }

            continue;
        }

        // `=` operation
        let (label, pow) = seq.split_once('=').unwrap();
        let hash = compute_hash(label);
        let power: u8 = pow.parse().unwrap();

        if let Some(box_) = boxes.get_mut(hash) {
            match box_.iter_mut().find(|lens| lens.seq == label) {
                Some(lens) => {
                    lens.power = power;
                }
                None => box_.push(Lens { seq: label, power }),
            }
        }
    }

    let mut total_focusing_power = 0;
    for (box_idx, box_) in boxes.iter().enumerate() {
        for (lens_idx, lens) in box_.iter().enumerate() {
            total_focusing_power += (box_idx + 1) * (lens_idx + 1) * lens.power as usize;
        }
    }

    total_focusing_power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            145,
            solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
    }
}

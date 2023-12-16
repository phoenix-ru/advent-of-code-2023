use common::main;
use day15::compute_hash;

main!();

fn solve(input: &str) -> usize {
    input.trim().split(',').map(|seq| compute_hash(&seq)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(52, solve("HASH"));

        assert_eq!(
            1320,
            solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        );
    }
}

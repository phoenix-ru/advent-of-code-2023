pub fn compute_hash(input: &str) -> usize {
    let mut result: usize = 0;
    for c in input.bytes() {
        result = ((result + c as usize) * 17) % 256;
    }

    result
}

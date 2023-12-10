pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut result = Vec::new();

    for line in input.lines() {
        result.push(
            line.trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        )
    }

    result
}

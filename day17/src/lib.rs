pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for line in input.lines().map(str::trim) {
        let mut line_data = Vec::with_capacity(line.len());
        line_data.extend(line.chars().map(|c| c.to_digit(10).unwrap()));
        result.push(line_data)
    }
    result
}

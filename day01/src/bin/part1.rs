fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .rfind(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();

        let num = first_digit * 10 + last_digit;
        sum += num;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            142,
            solve(
                "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"
            )
        )
    }
}

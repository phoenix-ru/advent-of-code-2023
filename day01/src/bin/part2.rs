const SEARCH_WORDS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut current_input = &line[..];

        'outer: while !current_input.is_empty() {
            // First, check a word
            for (idx, word) in SEARCH_WORDS.iter().enumerate() {
                if current_input.starts_with(word) {
                    let digit = (idx + 1) as u32;

                    if first_digit == 0 {
                        first_digit = digit;
                    }

                    last_digit = digit;
                    current_input = &current_input[1..];
                    continue 'outer;
                }
            }

            // Second, check a digit
            let c = current_input.chars().next();
            if let Some(c) = c {
                if c.is_numeric() {
                    let digit = c.to_digit(10).unwrap();

                    if first_digit == 0 {
                        first_digit = digit;
                    }
                    last_digit = digit;
                }
            }

            current_input = &current_input[1..];
        }

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
            281,
            solve(
                "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
            )
        )
    }

    #[test]
    fn it_works_real() {
        assert_eq!(
            33+97+32+54+48,
            solve("threerznlrhtkjp23mtflmbrzq395three
            9sevenvlttm
            3twochzbv
            mdxdlh5six5nqfld9bqzxdqxfour
            422268")
        );
        assert_eq!(
            81+89+47+44+33,
            solve("88msthvt4vbmnbrzjone
            nbgcs8nine
            4three53pczsx1sevenmzmtrzz
            four24qphdrxfsf
            gdgj3f")
        );
        assert_eq!(
            28,
            solve("hthphptmmtwo7sixsevenoneightls")
        );
        assert_eq!(
            28+66+83+62+87,
            solve("hthphptmmtwo7sixsevenoneightls
            qxbhjmmqsixfkfn36three6
            eightmkmdtvkctkvptsbckzpnkhpskdmp3
            six2twobgzsfsptlqnine42xtmdprjqc
            pxreightwo7")
        );
        
    }
}

#[macro_export]
macro_rules! main {
    () => {
        fn main() {
            let input = include_str!("../../input.txt");
            println!("{}", solve(input))
        }
    };
    ($additional_input: expr) => {
        fn main() {
            let input = include_str!("../../input.txt");
            println!("{}", solve(input, $additional_input))
        }
    };
}

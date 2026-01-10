pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    for chars in input.chars() {
        reversed.insert(0, chars);
    }
    reversed
}
pub fn reverse(input: &str) -> String {
    let mut chars: Vec<&str> = input.split("").collect();
    chars.reverse();

    chars.join("")
}

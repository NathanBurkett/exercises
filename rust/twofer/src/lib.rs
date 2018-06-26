pub fn twofer(name: &str) -> String {
    let output: &str;

    if name.is_empty() {
        output = "you";
    } else {
        output = name;
    }

    format!("One for {}, one for me.", output)
}

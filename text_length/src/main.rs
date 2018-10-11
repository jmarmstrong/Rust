fn main() {
    let text = String::from("Joseph");

    let len = calculate_length(&text);

    println!("The length of '{}' is '{}'", text, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

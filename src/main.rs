use regex::Regex;
fn main() {
    println!("Hello, world!");
}
fn extract_text_field(body: &str, regex: &str) -> Option<String> {
    let re = Regex::new(regex).unwrap();
    let text = re.find(body)?.as_str();
    Some(text.to_string())
}

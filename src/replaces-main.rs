use regex::Regex;

#[derive(Debug)]
struct LogEntry {
    timestamp: String,
    category: String,
    endpoint: String,
    id: String,
}

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let text = "123 apples and 456 oranges";
    let result = re.replace(text, "X");
    println!("{}", result);
    let result = re.replace_all(text, "X");
    println!("{}", result);
}

use regex::Regex;

fn main() {
    let re = Regex::new(r"ue").unwrap();
    let text = "my movie queue meue 6";

    match re.find(text) {
        Some(data) => {
            println!("{data:?}");
        }
        None => println!("No matches found"),
    }
}

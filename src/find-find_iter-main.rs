use regex::Regex;

fn main() {
    let re = Regex::new(r"ue").unwrap();
    let text = "Mueve el cuerpo, mueve la tibia mueve el esternon.";

    // Trobar la primera aparició del patró.
    match re.find(text) {
        Some(data) => println!("{data:?}"),
        None => println!("No matches found"),
    }
    println!();
    // trobar-les totes
    for data in re.find_iter(text) {
        println!(
            "Start at: {} Ends at: {} Found: {}",
            data.start(),
            data.end(),
            data.as_str()
        );
    }
}

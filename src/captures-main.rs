use regex::Regex;

fn main() {
    let re = Regex::new(r"(?<estat1>\d+).+(?<estat2>\w+{2})").unwrap();
    let text = "123 Elm Street, Palm Springs 08033, CA";
    let captures = re.captures(text).unwrap();
    println!("{}", captures.len());

    println!("{}", &captures[0]);
    println!("{}", &captures["estat1"]);
    println!("{}", &captures["estat2"]);

    //println!("{}", &captures[0]);
}

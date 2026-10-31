use std::env;

fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in input.trim().chars().flat_map(|c| c.to_lowercase()) {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    out.trim_end_matches('-').to_string()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tool <value> [value...]");
        std::process::exit(2);
    }
    for a in args {
        println!("{} -> {}", a, slugify(&a));
    }
}

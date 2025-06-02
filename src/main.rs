use lqf::parse_lqf;

fn main() {
    let input = r#"
    # This is an example lqf file
    > database
    host >> "localhost"
    port >> 5432

    > features
    enabled >> ["search", "logging", "metrics"]
    "#;

    match parse_lqf(input) {
        Ok(parsed) => println!("Parsed: {:?}", parsed),
        Err(e) => println!("Parsing failed: {}", e),
    }
}
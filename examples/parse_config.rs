use lqf::parse_lqf;

fn main() {
    let input = r#"
    # This is an example lqf file
    > database
    host >> "localhost"
    port >> 5432
    username >> "admin"
    usessl >> true

    > features
    enabled >> ["search", "logging", "metrics"]
    beta >> ["ai", "backup"]

    > server
    address >> "0.0.0.0"
    port >> 8080
    connections >> 1000
    timeout >> 30
    "#;

    // parse the LQF content
    match parse_lqf(input) {
        Ok(parsed_data) => println!("Parsed data: {:?}", parsed_data),
        Err(e) => {
            println!("Error parsing LQF: {}", e);
            return;
        }
    }
}
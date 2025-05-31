use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "lqf.pest"] // path to your pest grammar file
pub struct LqfParser;

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Null,
}

pub type Section = HashMap<String, Value>;
pub type LqfFile = HashMap<String, Section>;

pub fn parse_lqf(input: &str) -> Result<LqfFile, pest::error::Error<Rule>> {
    let mut result = HashMap::new();
    let pairs = LqfParser::parse(Rule::file, input)?;

    for pair in pairs {
        if pair.as_rule() == Rule::file {
            for section_pair in pair.into_inner() {
                if section_pair.as_rule() == Rule::section {
                    let mut inner_rules = section_pair.into_inner();

                    // Section header
                    let section_header = inner_rules.next().unwrap();
                    let section_name = section_header
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .to_string();

                    let mut assignments = HashMap::new();

                    // Assignments
                    for assignment in inner_rules {
                        let mut assign_inner = assignment.into_inner();
                        let key = assign_inner.next().unwrap().as_str().to_string();
                        let value_pair = assign_inner.next().unwrap();

                        let value = parse_value(value_pair);
                        assignments.insert(key, value);
                    }

                    result.insert(section_name, assignments);
                }
            }
        }
    }

    Ok(result)
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::string => {
            let s = pair.as_str();
            Value::String(s[1..s.len() - 1].to_string()) // remove quotes
        }
        Rule::number => {
            let s = pair.as_str().trim();
            match s.parse::<f64>() {
                Ok(n) => Value::Number(n),
                Err(_) => {
                    panic!("Failed to parse number from: {:?}", s); // or handle gracefully
                }
            }
        }
        Rule::boolean => Value::Boolean(pair.as_str().trim() == "true"),
        Rule::null => Value::Null,
        Rule::array => {
            let items = pair
                .into_inner()
                .map(parse_value)
                .collect();
            Value::Array(items)
        }

        _ => unreachable!("Unexpected rule in value: {:?}", pair.as_rule()),
    }
}


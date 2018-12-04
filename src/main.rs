#[macro_use]
extern crate clap;

use std::collections::BTreeMap;
use clap::App;

fn example(input: String) -> String
{
    return input;
}

fn main() {
    let mut advents: BTreeMap<&str, fn(String) -> String> = BTreeMap::new();

    advents.insert("0", example);

    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let days_to_run: Vec<&str> = match matches.values_of("days") {
        Some(days) => {
            let mut days_acc: Vec<&str> = Vec::new();
            for day in days {
                if !advents.contains_key(day) {
                    eprintln!("No such advent puzzle as \"{}\"", day);
                    std::process::exit(1)
                }
                days_acc.push(day);
            }
            days_acc
        }

        None => {
            advents.keys().cloned().collect()
        }
    };

    for day in days_to_run {
        println!("{}", day);
    }
}

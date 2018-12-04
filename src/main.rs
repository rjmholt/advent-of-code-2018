#[macro_use]
extern crate clap;

#[macro_use]
extern crate rust_embed;

mod puzzles;

use std::borrow::Cow;
use std::collections::BTreeMap;
use clap::App;
use puzzles::*;

#[derive(RustEmbed)]
#[folder = "input/"]
struct Input;

fn main() {
    let mut advents: BTreeMap<&str, fn(String) -> String> = BTreeMap::new();

    advents.insert("1_1", day1_1);
    advents.insert("1_2", day1_2);
    advents.insert("2_1", day2_1);

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
        let file_name: String = format!("{}.txt", day);
        let file_result: Option<Cow<'static, [u8]>> = Input::get(&file_name);
        let file_bytes = file_result.unwrap();
        let file_contents: String = std::str::from_utf8(file_bytes.as_ref()).unwrap().to_string();

        let puzzle_fn: &fn(String) -> String = advents.get(day).unwrap();

        let solution = puzzle_fn(file_contents);

        println!("Puzzle {}: {}", day, solution);
    }
}

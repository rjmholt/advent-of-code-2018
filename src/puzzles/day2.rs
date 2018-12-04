use std::collections::HashMap;

pub fn day2_1(input: String) -> String {
    let mut pairs: i32 = 0;
    let mut triples: i32 = 0;

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        let (has_pair, has_triple) = has_pair_triple(line);

        if has_pair {
            pairs += 1;
        }

        if has_triple {
            triples += 1;
        }
    }

    format!("{}", pairs * triples)
}

fn has_pair_triple(line: &str) -> (bool, bool) {
    let mut char_acc: HashMap<char, i32> = HashMap::new();

    for c in line.chars() {
        char_acc.entry(c)
            .and_modify(|v| { *v += 1 })
            .or_insert(1);
    }

    let has_pair = char_acc.values().any(|&v| v == 2);
    let has_triple = char_acc.values().any(|&v| v == 3);

    return (has_pair, has_triple);
}
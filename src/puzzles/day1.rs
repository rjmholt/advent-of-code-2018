use std::collections::HashSet;

pub fn day1_1(input: String) -> String {
    let mut sum: i32 = 0;
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let val = get_number(line);
        sum += val;
    }
    format!("{}", sum)
}

fn get_number(line: &str) -> i32 {
    let sign: &str = &line[0..1];
    let num: &str = &line[1..];

    match sign {
        "+" => {
            num.parse().unwrap()
        }

        "-" => {
            - num.parse::<i32>().unwrap()
        }

        _ => {
            panic!("Bad number format in input file")
        }
    }
}

pub fn day1_2(input: String) -> String {
    let mut sum: i32 = 0;
    let mut freqs: HashSet<i32> = HashSet::new();
    for line in input.lines().cycle() {
        if line.len() == 0 {
            continue;
        }
        let val = get_number(line);
        sum += val;

        if freqs.contains(&sum) {
            return format!("{}", sum);
        }

        freqs.insert(sum);
    }
    panic!("Program diverges");
}

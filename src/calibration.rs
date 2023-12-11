static NUMBERS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

pub fn calibration_value(line: &str, include_words: bool) -> Option<u32> {
    dbg!(line);
    let mut first_occurrences = Vec::with_capacity(line.len());
    let mut last_occurrences = Vec::with_capacity(line.len());
    for number in NUMBERS {
        if !include_words && is_word(number) {
            continue;
        }
        if let Some(left) = line.find(number) {
            first_occurrences.push((left, value(number)));
        }
        if let Some(right) = line.rfind(number) {
            last_occurrences.push((right, value(number)));
        }
    }

    let (_, left) = first_occurrences.iter().min_by_key(|(idx, _)| idx)?;
    let (_, right) = last_occurrences.iter().max_by_key(|(idx, _)| idx)?;

    Some(10 * left + right)
}

fn value(s: &str) -> u32 {
    if !is_word(s) {
        s.parse().unwrap()
    } else {
        match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => unreachable!(),
        }
    }
}

fn is_word(s: &str) -> bool {
    s.len() > 1
}

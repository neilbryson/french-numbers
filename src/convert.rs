use crate::words::{CONNECTOR, EIGHTY, HUNDRED, SEVENTY_PREFIX, TENS, THOUSAND, UNITS};
use crate::MAX_ALLOWED_NUMBER;

const UNIT_LIMIT: u32 = 19;
const TENS_LIMIT: u32 = 69;
const COMPOUND_TENS_LIMIT: u32 = 99;
const HUNDREDS_LIMIT: u32 = 999;

pub fn convert(value: u32) -> String {
    let mut remainder = value;
    let mut tmp_words: Vec<String> = Vec::new();
    let mut max_run = 3;

    if value == 0 {
        return UNITS.get(0).unwrap().to_string();
    }

    while remainder > 0 {
        if remainder <= UNIT_LIMIT {
            if let Some(word) = UNITS.get(remainder as usize) {
                tmp_words.push(word.to_string());
            }

            remainder = 0;
        } else if remainder <= TENS_LIMIT {
            let index = (remainder / 10) as usize;
            if let Some(word) = TENS.get(index - 1) {
                tmp_words.push(word.to_string());
            }
            remainder = remainder % 10;
            if remainder == 1 {
                tmp_words.push(CONNECTOR.to_string());
            }
        } else if remainder <= COMPOUND_TENS_LIMIT {
            let prefix_remainder = remainder / 10;
            let word = match prefix_remainder {
                7 => SEVENTY_PREFIX,
                _ => EIGHTY,
            };

            remainder = match prefix_remainder {
                7 => remainder % 60, // handle 70-79
                9 => remainder % 80, // handle 90-99
                _ => remainder % 10,
            };

            tmp_words.push(word.to_string());

            if prefix_remainder == 7 && remainder == 11 {
                tmp_words.push(CONNECTOR.to_string());
            }
        } else if remainder <= HUNDREDS_LIMIT {
            let prefix_remainder = remainder / 100;
            if let Some(word) = UNITS.get(prefix_remainder as usize) {
                if prefix_remainder > 1 {
                    tmp_words.push(word.to_string());
                }
            }
            remainder = remainder % 100;
            let hundred = plural(HUNDRED, prefix_remainder, remainder);
            tmp_words.push(hundred);
        } else if remainder <= MAX_ALLOWED_NUMBER {
            let prefix_remainder = remainder / 1000;
            let prefix = convert(prefix_remainder);
            if prefix_remainder > 1 {
                tmp_words.push(prefix);
            }
            remainder = remainder % 1000;
            let thousand = plural(THOUSAND, prefix_remainder, remainder);
            tmp_words.push(thousand);
        } else {
            // just to prevent possible infinite loops
            max_run = max_run - 1;
            if max_run <= 0 {
                break;
            }
        }
    }

    tmp_words.join("-")
}

fn plural(word: &str, prefix_remainder: u32, remaining_value: u32) -> String {
    if prefix_remainder > 1 && remaining_value == 0 {
        format!("{}s", word)
    } else {
        word.to_string()
    }
}

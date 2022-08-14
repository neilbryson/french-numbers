use crate::words::{CONNECTOR, EIGHTY, HUNDRED, SEVENTY_PREFIX, TENS, THOUSAND, UNITS};
use crate::MAX_ALLOWED_NUMBER;

const UNIT_LIMIT: u32 = 19;
const TENS_LIMIT: u32 = 69;
const COMPOUND_TENS_LIMIT: u32 = 99;
const HUNDREDS_LIMIT: u32 = 999;

pub fn convert(value: u32, process_plural: bool) -> String {
    let mut remainder = value;
    let mut word_list: Vec<String> = Vec::new();
    let mut max_run = 3;

    if value == 0 {
        return UNITS.get(0).unwrap().to_string();
    }

    while remainder > 0 {
        if remainder <= UNIT_LIMIT {
            if let Some(word) = UNITS.get(remainder as usize) {
                word_list.push(word.to_string());
            }

            remainder = 0;
        } else if remainder <= TENS_LIMIT {
            let index = (remainder / 10) as usize;
            if let Some(word) = TENS.get(index - 1) {
                word_list.push(word.to_string());
            }
            remainder = remainder % 10;
            if remainder == 1 {
                word_list.push(CONNECTOR.to_string());
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

            word_list.push(word.to_string());

            if prefix_remainder == 7 && remainder == 11 {
                word_list.push(CONNECTOR.to_string());
            }
        } else if remainder <= HUNDREDS_LIMIT {
            let prefix_remainder = remainder / 100;
            if let Some(word) = UNITS.get(prefix_remainder as usize) {
                if prefix_remainder > 1 {
                    word_list.push(word.to_string());
                }
            }
            remainder = remainder % 100;
            word_list.push(HUNDRED.to_string());
        } else if remainder <= MAX_ALLOWED_NUMBER {
            let prefix_remainder = remainder / 1000;
            let prefix = convert(prefix_remainder, false);
            if prefix_remainder > 1 {
                word_list.push(prefix);
            }
            remainder = remainder % 1000;
            word_list.push(THOUSAND.to_string());
        } else {
            // just to prevent possible infinite loops
            max_run = max_run - 1;
            if max_run <= 0 {
                break;
            }
        }
    }

    if process_plural {
        convert_to_plural(&mut word_list);
    }

    word_list.join("-")
}

fn convert_to_plural(word_list: &mut Vec<String>) {
    let length = word_list.len();

    if let Some(last_element) = word_list.get(length - 1) {
        if (length > 1 && (last_element == HUNDRED || last_element == THOUSAND))
            || (length == 1 && last_element == EIGHTY)
        {
            word_list[length - 1] = format!("{}s", last_element);
        }
    }
}

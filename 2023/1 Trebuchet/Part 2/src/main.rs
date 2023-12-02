/*
## \--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually _spelled out with letters_: `one`, `two`, `three`, `four`, `five`, `six`, `seven`, `eight`, and `nine` _also_ count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding these together produces `_281_`.

_What is the sum of all of the calibration values?_
*/
use std::{collections::HashMap, i8, io};

#[allow(non_snake_case)]

fn findDigit(line: &str, rev: bool) -> i8 {
    let mainEnumerate: Box<dyn Iterator<Item = (usize, char)>>;
    if rev {
        mainEnumerate = Box::new(line.chars().rev().enumerate());
    } else {
        mainEnumerate = Box::new(line.chars().enumerate());
    }
    for (i, char) in mainEnumerate {
        let wordToDigit = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        if char.is_numeric() {
            return char.to_digit(10).unwrap() as i8;
        } else {
            let mut word: String = String::new();
            word.push(char);

            let enumerator: Box<dyn Iterator<Item = (usize, char)>>;
            if rev {
                enumerator = Box::new(line.chars().rev().skip(i + 1).enumerate());
            } else {
                enumerator = Box::new(line.chars().skip(i + 1).enumerate());
            }
            for (j, char) in enumerator {
                if char.is_numeric() || j >= 5 {
                    break;
                } else {
                    word.push(char);
                    let digit: Option<&i8>;
                    if rev {
                        digit = wordToDigit.get(&word.chars().rev().collect::<String>() as &str);
                    } else {
                        digit = wordToDigit.get(&word as &str);
                    }
                    if digit != None {
                        return digit.unwrap().clone() as i8;
                    }
                }
            }
        }
    }
    return 0;
}

fn main() {
    let mut sum: i32 = 0;
    for rawLine in io::stdin().lines() {
        let line: String = rawLine.unwrap();
        let mut number: i8 = 0;
        let result = findDigit(&line.clone(), false);
        number += result * 10;
        let result = findDigit(&line.clone(), true);
        number += result;
        sum += number as i32;
    }
    println!("{}", sum);
}

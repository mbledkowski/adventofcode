/*
## \--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all _fifty stars_ by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants _one star_. Good luck!

You try to ask why they can't just use a [weather machine](https://adventofcode.com/2015/day/1) ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a [trebuchet](https://en.wikipedia.org/wiki/Trebuchet) ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been _amended_ by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific _calibration value_ that the Elves now need to recover. On each line, the calibration value can be found by combining the _first digit_ and the _last digit_ (in that order) to form a single _two-digit number_.

For example:

```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`. Adding these together produces `_142_`.

Consider your entire calibration document. _What is the sum of all of the calibration values?_
*/
use std::{i8, io};

fn main() {
    let mut sum: i32 = 0;
    for rawLine in io::stdin().lines() {
        let line: String = rawLine.unwrap();
        let mut number: i8 = 0;
        for char in line.chars() {
            if char.is_numeric() {
                number = char.to_digit(10).unwrap() as i8 * 10;
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_numeric() {
                number += char.to_digit(10).unwrap() as i8;
                break;
            }
        }
        sum += number as i32;
    }
    println!("{}", sum);
}

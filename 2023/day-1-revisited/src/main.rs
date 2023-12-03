/*
    --- Day 1: Trebuchet?! ---

    Something is wrong with global snow production, and you've been selected to take a look.
    The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

    You've been doing this long enough to know that to restore snow operations,
    you need to check all fifty stars by December 25th.

    Collect stars by solving puzzles.
    Two puzzles will be made available on each day in the Advent calendar;
    the second puzzle is unlocked when you complete the first.
    Each puzzle grants one star. Good luck!

    You try to ask why they can't just use a weather machine ("not powerful enough")
    and where they're even sending you ("the sky") and why your map looks mostly blank
    ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from")
    when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

    As they're making the final adjustments,
    they discover that their calibration document (your puzzle input)
    has been amended by a very young Elf who was apparently just excited to
    show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

    The newly-improved calibration document consists of lines of text;
    each line originally contained a specific calibration value that the Elves now need to recover.
    On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

    For example:

    ```
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    ```

    In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`. Adding these together produces `142`.

    Consider your entire calibration document. What is the sum of all of the calibration values?
*/

/*
    --- Part Two ---

    Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

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

    In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding these together produces `281`.

    What is the sum of all of the calibration values?
*/

include!(concat!(env!("OUT_DIR"), "/input.rs"));

#[derive(PartialEq, Eq)]
enum ScanDirection {
    Forwards,
    Backwards
}
fn find_first_digit(input: &str, direction: ScanDirection, include_words: bool) -> u32 {
    const DIGIT_WORDS: [&str; 10] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let mut chars = input.chars();
    let mut i = 0;
    while let Some(character) = if direction == ScanDirection::Forwards { chars.next() } else { chars.next_back() } {
        if let Some(digit) = character.to_digit(10) {
            return digit;
        }

        if include_words {
            for (digit, digit_word) in DIGIT_WORDS.iter().enumerate() {
                if input.len() - i < digit_word.len() {
                    continue;
                }
                let index_range = if direction == ScanDirection::Forwards {
                    i..(digit_word.len() + i)
                } else {
                    (input.len() - digit_word.len() - i)..(input.len() - i)
                };
                if input[index_range] == **digit_word {
                    return digit as u32;
                }
            }
        }

        i += 1;
    }

    panic!("No digits found lol");
}

fn find_sum(input: &[&str], include_words: bool) -> u32 {
    input.iter()
        .map(|calibration_value| {
            let first = find_first_digit(calibration_value, ScanDirection::Forwards, include_words);
            let last = find_first_digit(calibration_value, ScanDirection::Backwards, include_words);
            (first * 10) + last
        })
        .sum()
}

fn main() {
    println!("--- Part One ---");
    println!(
        "The sum of {} values is: {}.",
        INPUT.len(),
        find_sum(&INPUT, false)
    );

    println!();

    println!("--- Part Two ---");
    println!(
        "The sum of {} values is: {}.",
        INPUT.len(),
        find_sum(&INPUT, true)
    );
}
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

include!(concat!(env!("OUT_DIR"), "/input.rs"));

fn sanitize_calibration_value(dirty_value: &str) -> u32 {
    let numbers = dirty_value
            .chars()
            .filter(|character| character.is_numeric())
            .collect::<Vec<_>>();

        format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap()
}

fn part_one() {
    let calibration_values = INPUT.into_iter()
        .map(sanitize_calibration_value)
        .collect::<Vec<_>>();

    let sum: u32 = calibration_values.iter().sum();

    println!(
        "The sum of {} values is: {}.",
        calibration_values.len(),
        sum
    );
}

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

fn parse_numbers(input: &str) -> String {
    static DIGIT_WORDS: [&str; 10] = [
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

    let mut output = String::with_capacity(input.len());

    let mut i = 0;
    'outer: while i < input.len() {
        for (digit, digit_word) in DIGIT_WORDS.iter().enumerate() {
            if input.len() - i < digit_word.len() {
                continue;
            }
            if input[i..(i + digit_word.len())] == **digit_word {
                output.push_str(&digit.to_string());
                i += digit_word.len() - 1;
                continue 'outer;
            }
        }
        output.push(input.as_bytes()[i] as char);
        i += 1;
    }

    output
}

fn part_two() {
    let calibration_values = INPUT.into_iter()
        .map(parse_numbers)
        .map(|calibration_value| sanitize_calibration_value(&calibration_value))
        .collect::<Vec<_>>();

    let sum: u32 = calibration_values.iter().sum();

    println!(
        "The sum of {} values is: {}.",
        calibration_values.len(),
        sum
    );
}

fn main() {
    println!("--- Part One ---");
    part_one();

    println!();

    println!("--- Part Two ---");
    part_two();
}
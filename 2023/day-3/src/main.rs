/*
    --- Day 3: Gear Ratios ---

    You and the Elf eventually reach a gondola lift station;
    he says the gondola lift will take you up to the water source,
    but this is as far as he can bring you.
    You go inside.

    It doesn't take long to find the gondolas,
    but there seems to be a problem: they're not moving.

    "Aaah!"

    You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
    "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now;
    it'll still be a while before I can fix it." You offer to help.

    The engineer explains that an engine part seems to be missing from the engine,
    but nobody can figure out which one.
    If you can add up all the part numbers in the engine schematic,
    it should be easy to work out which part is missing.

    The engine schematic (your puzzle input) consists of a visual representation of the engine.
    There are lots of numbers and symbols you don't really understand,
    but apparently any number adjacent to a symbol, even diagonally,
    is a "part number" and should be included in your sum.
    (Periods (`.`) do not count as a symbol.)

    Here is an example engine schematic:

    ```
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ```

    In this schematic, two numbers are not part numbers because they are not
    adjacent to a symbol: `114` (top right) and `58` (middle right). Every other
    number is adjacent to a symbol and so is a part number; their sum is `4361`.

    Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/

include!(concat!(env!("OUT_DIR"), "/input.rs"));

fn collect_number<I: Iterator<Item = (usize, char)>>(iter: &mut I, first_character: char) -> (usize, u32) {
    let mut digits = vec![first_character];
    for (_, character) in iter.by_ref() {
        if !character.is_numeric() {
            break;
        }

        digits.push(character);
    }

    let number: String = digits.iter().collect();
    (number.len(), number.parse().unwrap())
}

fn part_one() {
    let mut part_nums = Vec::<u32>::new();

    for (y, line) in INPUT.iter().enumerate() {
        let mut line_iterator = line.chars().enumerate();
        while let Some((x, character)) = line_iterator.next() {
            if !character.is_numeric() {
                continue;
            }

            let (number_length, number) = collect_number(&mut line_iterator, character);

            let mut parts_to_check_for_symbol: Vec<String> = Vec::new();

            let witnin_bounds_left = x != 0;
            let within_bounds_right = x + number_length < line.len();
            let x_start = if witnin_bounds_left { x - 1 } else { 0 };
            let x_end = if within_bounds_right { x + number_length } else { line.len() - 1 };

            if y > 0 { parts_to_check_for_symbol.push(INPUT[y - 1][x_start..=x_end].to_string()); }
            if witnin_bounds_left { parts_to_check_for_symbol.push(INPUT[y].chars().nth(x_start).unwrap().to_string()) };
            if within_bounds_right { parts_to_check_for_symbol.push(INPUT[y].chars().nth(x_end).unwrap().to_string()) }
            if y + 1 < INPUT.len() { parts_to_check_for_symbol.push(INPUT[y + 1][x_start..=x_end].to_string()); }

            if parts_to_check_for_symbol
                .iter()
                .any(|part| part.chars().any(|x| x != '.'))
            {
                part_nums.push(number);
            }
        }
    }

    let sum: u32 = part_nums.iter().sum();

    println!("Sum of {} part numbers is: {}", part_nums.len(), sum);
}

/*
    --- Part Two ---

    The engineer finds the missing part and installs it in the engine! As the engine springs to life,
    you jump in the closest gondola, finally ready to ascend to the water source.

    You don't seem to be going very fast, though. Maybe something is still wrong?
    Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

    Before you can explain the situation, she suggests that you look out the window.
    There stands the engineer, holding a phone in one hand and waving with the other.
    You're going so slowly that you haven't even left the station. You exit the gondola.

    The missing part wasn't the only issue - one of the gears in the engine is wrong.
    A gear is any `*` symbol that is adjacent to exactly two part numbers.
    Its gear ratio is the result of multiplying those two numbers together.

    This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

    Consider the same engine schematic again:

    ```
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ```

    In this schematic, there are two gears.
    The first is in the top left; it has part numbers `467` and `35`, so its gear ratio is `16345`.
    The second gear is in the lower right; its gear ratio is `451490`.
    (The `*` adjacent to `617` is not a gear because it is only adjacent to one part number.)
    Adding up all of the gear ratios produces `467835`.

    What is the sum of all of the gear ratios in your engine schematic?
*/

fn go_to_start_of_number_and_collect(line: &str, x: usize) -> (usize, u32) {
    let mut start_index = x;
    loop {
        if start_index == 0 {
            break;
        }

        let character = line.chars().nth(start_index - 1).unwrap();
        if !character.is_numeric() {
            break;
        }

        start_index -= 1;
    }

    let mut end_index = x;
    loop {
        if end_index == line.len() - 1 {
            break;
        }

        let character = line.chars().nth(end_index + 1).unwrap();
        if !character.is_numeric() {
            break;
        }

        end_index += 1;
    }

    let number = line[start_index..=end_index].parse().unwrap();
    (end_index, number)
}

fn collect_numbers_within_bounds(line: &str, start: usize, end: usize) -> Vec<u32> {
    let mut discovered_numbers = Vec::new();

    let mut i = start;
    while i < end {
        let character = line.chars().nth(i).unwrap();
        if character.is_numeric() {
            let (end_index, number) = go_to_start_of_number_and_collect(line, i);
            discovered_numbers.push(number);
            i = end_index;
        }
        i += 1;
    }

    discovered_numbers
}

fn part_two() {
    let mut gear_ratios = Vec::<u32>::new();

    for (y, line) in INPUT.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '*' {
                continue;
            }

            let witnin_bounds_left = x != 0;
            let within_bounds_right = x + 2 < line.len();
            let x_start = if witnin_bounds_left { x - 1 } else { 0 };
            let x_end = if within_bounds_right { x + 2 } else { line.len() - 1 };

            let mut discovered_numbers: Vec<u32> = Vec::new();
            if y > 0 { discovered_numbers.append(&mut collect_numbers_within_bounds(INPUT[y - 1], x_start, x_end)); }
            discovered_numbers.append(&mut collect_numbers_within_bounds(line, x_start, x_end));
            if y + 1 < INPUT.len() { discovered_numbers.append(&mut collect_numbers_within_bounds(INPUT[y + 1], x_start, x_end)); }

            if discovered_numbers.len() == 2 {
                gear_ratios.push(discovered_numbers[0] * discovered_numbers[1]);
            }
        }
    }

    let sum: u32 = gear_ratios.iter().sum();

    println!("Sum of {} gear ratios is: {}", gear_ratios.len(), sum);
}





fn main() {
    part_one();
    part_two();
}
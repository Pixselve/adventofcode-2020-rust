use regex::Regex;
use std::{fmt, fs};

fn main() {
    let lines = read_from_file("C:/Users/maelk/Documents/CLionProjects/adventofcode-day2/src/input.txt".parse().unwrap());
    let bools_first_part = lines.iter().map(|s| parse_input(s).unwrap().validate()).collect::<Vec<bool>>();
    let bools_second_part = lines.iter().map(|s| parse_input(s).unwrap().validate_alter_rule()).collect::<Vec<bool>>();
    let mut valid_count_first_part = 0;
    let mut valid_count_second_part = 0;
    for bool in bools_first_part {
        if bool { valid_count_first_part += 1 }
    }
    for bool in bools_second_part {
        if bool { valid_count_second_part += 1 }
    }
    println!("{}", valid_count_first_part);
    println!("{}", valid_count_second_part);


    println!("{}", parse_input("1-3 a: abcde").unwrap().validate_alter_rule());
    println!("{}", parse_input("1-3 b: cdefg").unwrap().validate_alter_rule());
    println!("{}", parse_input("2-9 c: ccccccccc").unwrap().validate_alter_rule());
}


struct PasswordData {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordData {
    fn validate(&self) -> bool {
        let mut letter_count: usize = 0;
        let password_chars = &self.password.chars().collect::<Vec<char>>();
        for i in 0..self.password.len() {
            if password_chars[i].eq(&self.letter) {
                letter_count += 1;
            }
        }
        return letter_count >= self.min && letter_count <= self.max;
    }

    fn validate_alter_rule(&self) -> bool {
        let password_chars = &self.password.chars().collect::<Vec<char>>();
        return (password_chars.len() >= (self.min) && password_chars[self.min - 1].eq(&self.letter)) != (password_chars.len() >= (self.max) && password_chars[self.max - 1].eq(&self.letter));
    }
}


impl fmt::Display for PasswordData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "min: {}, max: {}, letter: {}, password: {}", self.min, self.max, self.letter, self.password)
    }
}

fn parse_input(data: &str) -> Option<PasswordData> {     // 10-20 a: abcde
    let seperator = Regex::new(r"(: )|( )").expect("Invalid regex");
    let three_parts = seperator.split(data).collect::<Vec<&str>>();
    if three_parts.len() != 3 { return None; }
    let min_and_max = three_parts[0].split("-").collect::<Vec<&str>>();
    if min_and_max.len() != 2 { return None; }
    return Some(PasswordData {
        password: String::from(three_parts[2]),
        letter: three_parts[1].chars().collect::<Vec<char>>()[0],
        max: min_and_max[1].parse().unwrap(),
        min: min_and_max[0].parse().unwrap(),
    });
}

fn read_from_file(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    return contents.lines().map(|x| x.to_string()).collect::<Vec<String>>();
}
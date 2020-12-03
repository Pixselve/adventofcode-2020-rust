use std::fs;

fn main() {
    let strings = read_from_file("src/input.txt".parse().unwrap());
    let numbers = strings.iter().map(|s| s.parse::<isize>().unwrap_or(0)).collect::<Vec<isize>>();
    let values2 = find_sum_2(numbers.clone());
    let values3 = find_sum_3(numbers);
    println!("{}", values2.unwrap().0 * values2.unwrap().1);
    println!("{}", values3.unwrap().0 * values3.unwrap().1 * values3.unwrap().2);
}

fn find_sum_2(values: Vec<isize>) -> Option<(isize, isize)> {
    for a in &values {
        for b in &values {
            if a + b == 2020 {
                return Some((*a, *b));
            }
        }
    }
    return None;
}


fn find_sum_3(values: Vec<isize>) -> Option<(isize, isize, isize)> {
    for a in &values {
        for b in &values {
            for c in &values {
                if a + b + c == 2020 {
                    return Some((*a, *b, *c));
                }
            }


        }
    }
    return None;
}

fn read_from_file(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    return contents.lines().map(|x| x.to_string()).collect::<Vec<String>>();
}
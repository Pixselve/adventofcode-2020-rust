use std::fs;

#[derive(Debug, PartialEq)]
enum TerrainType {
    Tree,
    Field,
}

struct Line {
    line: Vec<TerrainType>
}

impl Line {
    pub fn new(line: &String) -> Line {
        let mut result: Vec<TerrainType> = Vec::new();
        let line_chars = line.chars().collect::<Vec<char>>();
        for character_index in 0..line.len() {
            result.push(if line_chars[character_index] == '#' { TerrainType::Tree } else { TerrainType::Field })
        }
        return Line { line: result };
    }

    fn value(&self, index: usize) -> &TerrainType {
        let length = self.line.len();
        return &self.line[index % length];
    }
}

fn count_tree_for_slope(lines: &Vec<Line>, right: u8, down: u8) -> usize {
    let mut tree_count: usize = 0;
    let mut status: usize = 0;
    for i in (0..lines.len()).step_by(down as usize) {
        if lines[i].value(status) == &TerrainType::Tree {
            tree_count += 1;
        }
        status += right as usize;
    }
    return tree_count;
}


fn read_from_file(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    return contents.lines().map(|x| x.to_string()).collect::<Vec<String>>();
}


fn main() {
    let lines_string = read_from_file("src/input.txt".parse().unwrap());
    let lines = lines_string.iter().map(|s| Line::new(s)).collect::<Vec<Line>>();


    println!("Right 3, down 1: {}", count_tree_for_slope(&lines, 3, 1));
    println!("Multiplication: {}",
             count_tree_for_slope(&lines, 1, 1) *
                 count_tree_for_slope(&lines, 3, 1) *
                 count_tree_for_slope(&lines, 5, 1) *
                 count_tree_for_slope(&lines, 7, 1) *
                 count_tree_for_slope(&lines, 1, 2)
    );
}

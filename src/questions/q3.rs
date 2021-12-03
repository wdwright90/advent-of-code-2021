use advent_of_code_2021::read_lines;

struct Pos {
    horizontal: i32,
    depth: i32,
    aim: i32
}

impl Pos {
    fn forward_x(&mut self, number: i32) {
        self.horizontal += number;
        self.depth += number * self.aim;
    }
    fn down_x(&mut self, number: i32) {
        self.aim += number
    }
    fn up_x(&mut self, number: i32) {
        self.aim -= number
    }
    fn depth_horz_mult(&self) -> i32 {
        self.depth * self.horizontal
    }
}

pub(crate) fn main() {
    if let Ok(lines) = read_lines("inputs/input.txt") {
        let mut sub_pos = Pos {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        for line in lines {
            if let Ok(line) = line {
                let mut split_lines = line.split_whitespace();
                let command = split_lines.next().unwrap();
                let value = split_lines.next().unwrap().parse::<i32>().unwrap();
                match command {
                    "forward" => sub_pos.forward_x(value),
                    "down" => sub_pos.down_x(value),
                    "up" => sub_pos.up_x(value),
                    _ => {}
                }
            }
        }
        println!("The depth times horizontal is: {}", sub_pos.depth_horz_mult());
    }
}

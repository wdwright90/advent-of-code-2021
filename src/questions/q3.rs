use advent_of_code_2021::read_lines;

pub fn answer_part_1() -> isize {
    let mut line_count:u32 = 0;
    let mut count_vec: Vec<u32> = vec![0; 12];
    let mut gammma_string = String::new();
    let mut epsilon_string = String::new();
    if let Ok(lines) = read_lines("src/inputs/input_3.txt") {
        for line in lines {
            if let Ok(line) = line {
                println!("{}",line);
                for (i, chr) in line.chars().enumerate() {
                    count_vec[i] += chr.to_digit(10).unwrap();
                }
                line_count += 1;
            }
        }
        for val in count_vec.iter() {
            if val > &(line_count / 2) {
                gammma_string.push('1');
                epsilon_string.push('0');
            } else {
                gammma_string.push('0');
                epsilon_string.push('1');
            }
        }
    }
    let gamma = isize::from_str_radix(&gammma_string, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();
    println!("answer {} {}", gamma, epsilon);
    return gamma * epsilon;
}

pub fn answer_part_2() -> isize {
    let mut line_count:u32 = 0;
    let mut count_vec: Vec<u32> = vec![0; 12];
    let mut o2: Vec<String> = Vec::new();
    let mut co2: Vec<String> = Vec::new();
    let mut o2_new: Vec<String> = Vec::new();
    let mut co2_new: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("src/inputs/input_3.txt") {
        for line in lines {
            if let Ok(line) = line {
                o2.push(line.clone());
                co2.push(line.clone());
            }
        }
        for i in 0..12 {
            line_count = 0;
            count_vec = vec![0; 12];
            for ind in 0..o2.len() {
                for (j, chr) in o2[ind].chars().enumerate() {
                    count_vec[j] += chr.to_digit(10).unwrap();
                }
                line_count += 1;
            }
            for j in 0..o2.len() {
                for (k, chr) in o2[j].chars().enumerate() {
                    if k == i {
                        let a = chr.to_digit(2).unwrap();
                        let mut b = 0;
                        if count_vec[i] as f32 >= line_count as f32 / 2.0 {
                            b = 1;
                        }
                        if a == b {
                            o2_new.push(o2[j].clone());
                        }
                    }
                }
            }
            o2 = o2_new.clone();
            o2_new = Vec::new();
            if o2.len() == 1 {
                break
            }
        }
        for i in 0..12 {
            line_count = 0;
            count_vec = vec![0; 12];
            for ind in 0..co2.len() {
                for (j, chr) in co2[ind].chars().enumerate() {
                    count_vec[j] += chr.to_digit(10).unwrap();
                }
                line_count += 1;
            }
            for j in 0..co2.len() {
                for (k, chr) in co2[j].chars().enumerate() {
                    if k == i {
                        let a = chr.to_digit(2).unwrap();
                        let mut b = 1;
                        if count_vec[i] as f32 >= line_count as f32 / 2.0 {
                            b = 0;
                        }
                        if a == b {
                            //println!("this was true");
                            co2_new.push(co2[j].clone());
                        }
                    }
                }
            }
            co2 = co2_new.clone();
            co2_new = Vec::new();
            if co2.len() == 1 {
                break
            }
        }
    }
    println!("o2.len is {}", o2.len());
    println!("co2.len is {}", co2.len());
    let o2_an = isize::from_str_radix(&o2[0], 2).unwrap();
    let co2_an = isize::from_str_radix(&co2[0], 2).unwrap();
    println!("o2 {}", o2_an * co2_an);

    return o2_an * co2_an;
}

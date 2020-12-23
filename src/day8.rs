use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
pub enum Data {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn run(data: &[Data]) -> (bool, i32) {
    let mut pc = 0_i32;
    let mut acc = 0_i32;
    let mut visited = vec![false; data.len()];
    while pc as usize != data.len() && !visited[pc as usize] {
        visited[pc as usize] = true;
        match data[pc as usize] {
            Data::Nop(_) => pc += 1,
            Data::Acc(arg) => {
                pc += 1;
                acc += arg
            }
            Data::Jmp(arg) => pc += arg,
        }
    }
    (pc as usize == data.len(), acc)
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            if let Some(line) = line.strip_prefix("nop ") {
                let result = line.parse().unwrap();
                Data::Nop(result)
            } else if let Some(line) = line.strip_prefix("jmp ") {
                let result = line.parse().unwrap();
                Data::Jmp(result)
            } else if let Some(line) = line.strip_prefix("acc ") {
                let result = line.parse().unwrap();
                Data::Acc(result)
            } else {
                panic!("Unknown instruction")
            }
        })
        .collect()
}

fn flip(data: &mut [Data], line: usize) {
    match data[line] {
        Data::Nop(v) => data[line] = Data::Jmp(v),
        Data::Jmp(v) => data[line] = Data::Nop(v),
        _ => (),
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &[Data]) -> i32 {
    let (_, acc) = run(data);
    acc
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &[Data]) -> i32 {
    let mut my_data = data.to_vec();
    for (line, _instr) in data
        .iter()
        .enumerate()
        .filter(|(_, n)| matches!(n, Data::Nop(_) | Data::Jmp(_)))
    {
        flip(&mut my_data, line);
        let (res, acc) = run(&my_data);
        if res {
            // println!("Changed line {} from {:?}", line, instr);
            return acc;
        }

        flip(&mut my_data, line);
    }
    panic!("No solution found");
}

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(i64),
    Add,
    Mul,
    LPar,
    RPar,
}

fn tokenize(line: &[char]) -> Vec<Token> {
    let mut line = line;

    let mut result = Vec::new();

    while !line.is_empty() {
        while !line.is_empty() && line[0] == ' ' {
            line = &line[1..];
        }
        if !line.is_empty() && line[0] == '+' {
            line = &line[1..];
            result.push(Token::Add);
        } else if !line.is_empty() && line[0] == '*' {
            line = &line[1..];
            result.push(Token::Mul);
        } else if !line.is_empty() && line[0] == '(' {
            line = &line[1..];
            result.push(Token::LPar);
        } else if !line.is_empty() && line[0] == ')' {
            line = &line[1..];
            result.push(Token::RPar);
        } else if !line.is_empty() && line[0].is_ascii_digit() {
            let mut num = 0_i64;
            while !line.is_empty() && line[0].is_ascii_digit() {
                num = num * 10 + line[0].to_digit(10).unwrap() as i64;
                line = &line[1..];
            }
            result.push(Token::Num(num));
        } else {
            panic!("Unknown char");
        }
    }
    result
}

type Data = Vec<Token>;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            let line: Vec<_> = line.chars().collect();
            tokenize(&line)
        })
        .collect()
}

fn calc(tokens: &[Token]) -> i64 {
    let mut tokens: Vec<_> = tokens.into();
    loop {
        let mut start = None;
        'tokens: for i in 0..tokens.len() {
            if tokens[i] == Token::LPar {
                start = Some(i);
            } else if tokens[i] == Token::RPar {
                let start = start.unwrap();
                let val = calc_span(&tokens[start + 1..i]);
                tokens.splice(start..=i, Some(Token::Num(val)));
                break 'tokens;
            }
        }
        if start.is_none() {
            break;
        }
    }
    calc_span(&tokens)
}

fn calc_span(tokens: &[Token]) -> i64 {
    let mut tokens = tokens;
    let mut acc = -1;
    while !tokens.is_empty() {
        match tokens[0] {
            Token::Num(n) => {
                acc = n;
                tokens = &tokens[1..];
            }
            Token::Add => {
                if let Token::Num(n) = tokens[1] {
                    acc += n;
                    tokens = &tokens[2..];
                } else {
                    panic!("Incorrect");
                }
            }
            Token::Mul => {
                if let Token::Num(n) = tokens[1] {
                    acc *= n;
                    tokens = &tokens[2..];
                } else {
                    panic!("Incorrect");
                }
            }
            _ => panic!("unkown"),
        }
    }
    acc
}

#[aoc(day18, part1)]
pub fn solve_part1(data: &[Data]) -> i64 {
    data.iter().map(|expr| calc(expr)).sum()
}

fn calc2(tokens: &[Token]) -> i64 {
    let mut tokens: Vec<_> = tokens.into();
    loop {
        let mut any = false;
        let mut start = None;
        'tokens: for i in 0..tokens.len() {
            if tokens.len() > i + 2 {
                if let Token::Num(n) = tokens[i] {
                    if let Token::Num(n2) = tokens[i + 2] {
                        if tokens[i + 1] == Token::Add {
                            tokens.splice(i..=i + 2, Some(Token::Num(n + n2)));
                            any = true;
                            break 'tokens;
                        }
                    }
                }
            }
            if tokens[i] == Token::LPar {
                start = Some(i);
            } else if tokens[i] == Token::RPar {
                let start = start.unwrap();
                let val = calc_span2(&tokens[start + 1..i]);
                tokens.splice(start..=i, Some(Token::Num(val)));
                any = true;
                break 'tokens;
            }
        }
        if !any {
            break;
        }
    }
    calc_span2(&tokens)
}

fn calc_span2(tokens: &[Token]) -> i64 {
    let mut tokens = tokens;
    let mut acc = -1;
    while !tokens.is_empty() {
        match tokens[0] {
            Token::Num(n) => {
                acc = n;
                tokens = &tokens[1..];
            }
            Token::Mul => {
                if let Token::Num(n) = tokens[1] {
                    acc *= n;
                    tokens = &tokens[2..];
                } else {
                    panic!("Incorrect");
                }
            }
            Token::Add => {
                if let Token::Num(n) = tokens[1] {
                    acc += n;
                    tokens = &tokens[2..];
                } else {
                    panic!("Incorrect");
                }
            }
            _ => panic!("unkown"),
        }
    }
    acc
}

#[aoc(day18, part2)]
pub fn solve_part2(data: &[Data]) -> i64 {
    data.iter().map(|expr| calc2(expr)).sum()
}

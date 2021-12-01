use std::str::FromStr;
use std::vec::Vec;


static INPUT: &'static str = include_str!("input");

fn parse_input<T>() -> Result<Vec<T>, <T as FromStr>::Err> where T: FromStr {
    let mut ret: Vec<T> = vec![];
    for line in INPUT.lines() {
        ret.push(line.parse::<T>()?);
    }
    Ok(ret)
}

fn main() -> Result<(), <u16 as FromStr>::Err> {
    let input: Vec<i32> = parse_input()?;
    
    let mut getting_deeper: Vec<bool> = vec![];
    for i in 0..(input.len() - 1) {
        getting_deeper.push(input[i+1] > input[i]);
    }

    let mut getting_deeper_3_win: Vec<bool> = vec![];
    for i in 0..(input.len() - 3) {
        getting_deeper_3_win.push(input[i+1] + input[i+2] + input[i+3] > input[i] + input[i+1] + input[i+2]);
    }

    println!("Hello, world!");
    println!("{}", getting_deeper.into_iter().fold(0 as usize, |acc, val| acc + (val as usize)));
    println!("{}", getting_deeper_3_win.into_iter().fold(0 as usize, |acc, val| acc + (val as usize)));
    Ok(())
}

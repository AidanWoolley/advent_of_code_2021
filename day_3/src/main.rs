use std::num::ParseIntError;

static INPUT: &'static str = include_str!("input");


fn parse_input() -> Vec<u16> {
    INPUT.lines().map(|l| u16::from_str_radix(l, 2)).collect::<Result<Vec<u16>, ParseIntError>>().unwrap()
}


fn most_set_bitmask(v: &Vec<u16>) -> u16{
    let input_bit_len = v.iter().map(|x| 16 - x.leading_zeros()).max().unwrap() as usize;
    let mut counts: Vec<usize> = (0..input_bit_len).map(|_| 0).collect();
    for x in v.iter() {
        for i in 0usize..input_bit_len {
            if x & (1 << i) != 0 {
                counts[i] += 1;
            }
        }
    }
    counts.into_iter().rev().fold(0, |acc, v| (acc << 1) | if v >= 500 {1} else {0})
}


fn bit_partitition(v: &Vec<u16>, bit: u8) -> (Vec<u16>, Vec<u16>) {
    let mut ones = Vec::new();
    let mut zeros = Vec::new();
    for x in v {
        if (x >> bit) & 1 == 1 {
            ones.push(*x);
        } else {
            zeros.push(*x);
        }
    }
    assert!(ones.len() + zeros.len() == v.len());
    (ones, zeros)
}


fn main() {
    let input = parse_input();
    let gamma = most_set_bitmask(&input) as usize;
    println!("{}", gamma * ((!gamma) & 0x0FFF));

    let mut og_factor = input.clone();
    for bit in (0..=11).rev() {
        let (ones, zeros) = bit_partitition(&og_factor, bit);
        if ones.len() >= zeros.len() {
            og_factor = ones;
        } else {
            og_factor = zeros;
        }
    }
    assert!(og_factor.len() == 1);

    let mut cs_factor = input.clone();
    for bit in (0..=11).rev() {
        let (ones, zeros) = bit_partitition(&cs_factor, bit);
        if std::cmp::max(ones.len(), zeros.len()) == 1 {
            if zeros.len() == 1 {
                cs_factor = zeros;
            } else {
                cs_factor = ones;
            }
            break;
        }
        if zeros.len() <= ones.len() {
            cs_factor = zeros;
        } else {
            cs_factor = ones;
        }
    }
    assert!(cs_factor.len() == 1);

    println!("{}", og_factor[0] as usize * cs_factor[0] as usize);
}

use std::str::FromStr;
use std::vec::Vec;


static INPUT: &'static str = include_str!("input");

enum SubDirection {
    Vertical(i32),
    Forwards(i32)
}

impl FromStr for SubDirection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [dir, val] = s.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            match dir {
                "forward" => Ok(SubDirection::Forwards(val.parse().map_err(|_| "Invalid distance")?)),
                "up" => Ok(SubDirection::Vertical(-val.parse().map_err(|_| "Invalid distance")?)),
                "down" => Ok(SubDirection::Vertical(val.parse().map_err(|_| "Invalid distance")?)),
                _ => Err("Invalid Sub Direction")
            }
        } else {
            Err("Wrong number of values on line")
        }
    }
}


fn parse_input<T>() -> Result<Vec<T>, <T as FromStr>::Err> where T: FromStr {
    INPUT.lines().map(|line| line.parse() ).collect()
}


fn main() -> Result<(), &'static str>{
    let input: Vec<SubDirection> = parse_input()?;
    println!("{}", input.iter().fold([0, 0], |[depth, forwards], dir| {
        match dir {
            &SubDirection::Forwards(dist) => [depth, forwards + dist],
            &SubDirection::Vertical(dist) => [depth + dist, forwards]
        }
    }).into_iter().product::<i32>());
    println!("{}", input.iter().fold([0, 0, 0], |[aim, depth, forwards], dir| {
        match dir {
            &SubDirection::Forwards(dist) => [aim, depth + dist * aim, forwards + dist],
            &SubDirection::Vertical(dist) => [aim + dist, depth, forwards]
        }
    // Don't want to include aim value in product
    }).into_iter().skip(1).product::<i32>());
    Ok(())
}

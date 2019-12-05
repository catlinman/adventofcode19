
use std::fs;
use std::env;
use std::num;

// Recursively calculate fuel.
fn recurse_fuel(mass: u32, division: u32, subtraction: u32) -> u32 {
    let mut fuel: u32 = u32::checked_sub(mass / division, subtraction).unwrap_or(0);

    if mass > subtraction {
        fuel += recurse_fuel(fuel, division, subtraction);
    }

    fuel
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input argument. Skip first execution argument, flatten further arguments as input path.
    let contents: String = fs::read_to_string(env::args().skip(1)
        .fold(String::new(), |acc, cur| format!("{}{}", acc, cur))
    ).expect("Failed to read the input file!");

    // Split the strhing into lines, parse the array as a u32 and sum all values.
    let sum: Result<u32, num::ParseIntError> = contents.lines()
        .map(|value| Ok(recurse_fuel(value.parse::<u32>()?, 3, 2)))
        .sum();

    // We can handle our sum without a match as we handle dynamic errors.
    println!("{}", sum?);

    Ok(())
}

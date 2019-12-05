
use std::fs;
use std::env;
use std::num;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input argument. Skip first execution argument, flatten further arguments as input path.
    let contents: String = fs::read_to_string(env::args().skip(1)
        .fold(String::new(), |acc, cur| format!("{}{}", acc, cur))
    )?;

    // Split the string into lines, parse the array as a u32 and sum all values.
    let sum: Result<u32, num::ParseIntError> = contents.lines()
        .map(|value| Ok((value.parse::<u32>()? / 3) - 2))
        .sum();
    
    // We can handle our sum without a match as we handle dynamic errors.
    println!("{}", sum?);

    Ok(())
}

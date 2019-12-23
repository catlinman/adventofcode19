use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input argument. Skip first execution argument, flatten further arguments as input path.
    let contents: String = fs::read_to_string(
        env::args()
            .skip(1)
            .fold(String::new(), |acc, cur| format!("{}{}", acc, cur)),
    )?;

    // Parse our strings as u16 and filter any incorrect values. Make sure to strip whitespace.
    let mut memory: Vec<usize> = contents
        .trim()
        .split(",")
        .filter_map(|value| value.parse::<usize>().ok())
        .collect();

    // Reset the first two value memory as specified at the end of the puzzle.
    memory[1] = 12;
    memory[2] = 2;

    // Iterate over our memory. Store the position.
    let mut position: usize = 0;
    loop {
        // Fetch a value from the opcodes register by position. Use escape signal if invalid.
        let code = match Some(memory[position]) {
            Some(x) => x,
            None => break,
        };

        // Get the first, second and store register positions if available. Use get for Option return.
        let (address_a, address_b, address_register) = (
            *memory.get(position + 1).unwrap_or(&99) as usize,
            *memory.get(position + 2).unwrap_or(&99) as usize,
            *memory.get(position + 3).unwrap_or(&99) as usize,
        );

        // Match our desired opcode instruction. We break on any non control opcode.
        match code {
            1 => memory[address_register] = memory[address_a] + memory[address_b],
            2 => memory[address_register] = memory[address_a] * memory[address_b],
            _ => break,
        }

        // Jump four memory to our next opcode.
        position += 4;
    }

    println!("{}", memory[0]);

    Ok(())
}

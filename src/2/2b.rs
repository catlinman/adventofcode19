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
    let core_memory: Vec<usize> = contents
        .trim()
        .split(",")
        .filter_map(|value| value.parse::<usize>().ok())
        .collect();

    // Begin bruteforcing values.
    'outer: for noun in 1..99 {
        for verb in 1..99 {
            // Reset/clone our original memory into our usable memory.
            let mut memory: Vec<usize> = core_memory.clone();
            memory[1] = noun;
            memory[2] = verb;

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

            // If we found our value, stop our loops and print its pair of inputs.
            if memory[0] == 19690720 {
                println!("{}", 100 * noun + verb);

                // Break our nested set of loops.
                break 'outer;
            }
        }
    }

    Ok(())
}

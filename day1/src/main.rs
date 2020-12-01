use anyhow::Result;

fn read_numbers(filename: &str) -> Result<Vec<i32>> {
    let contents = std::fs::read_to_string(filename)?;

    let mut numbers = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }

        numbers.push(line.parse()?);
    }

    Ok(numbers)
}

fn main() -> Result<()> {
    // TODO(jsvana): make this a commandline argument
    let numbers = read_numbers("input.txt")?;

    for (i, first_number) in numbers.iter().enumerate() {
        for (j, second_number) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }

            for (k, third_number) in numbers.iter().enumerate() {
                if i == k {
                    continue;
                }

                if first_number + second_number + third_number == 2020 {
                    println!("{}", first_number * second_number * third_number);
                }
            }
        }
    }

    Ok(())
}

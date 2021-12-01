use std::collections::BTreeMap;

use anyhow::Result;

fn print_layer(layer: &str, width: usize, height: usize) {
    for y in 0..height {
        println!("{}", &layer[y * width..(y + 1) * width]);
    }
}

fn get_layer(image: &str, width: usize, height: usize, layer: usize) -> &str {
    let layer_size = width * height;

    let layer_index = layer_size * layer;

    &image[layer_index..layer_size * (layer + 1)]
}

fn count_digits(layer: &str) -> BTreeMap<char, usize> {
    let mut counts = BTreeMap::new();

    for c in layer.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts
}

fn read_input(filename: &str) -> Result<String> {
    Ok(std::fs::read_to_string(filename)?.clone())
}

fn main() -> Result<()> {
    let image = read_input("input.txt")?;

    let width = 25;
    let height = 6;

    let mut result = String::with_capacity(width * height);

    let layer_count = image.len() / (width * height);

    for y in 0..height {
        for x in 0..width {
            let mut found = false;
            for layer in 0..layer_count {
                let layer_size = width * height;
                let layer_index = layer_size * layer + y * width + x;
                let color = &image[layer_index..layer_index + 1];

                if color == "0" {
                    result.push_str(" ");
                    found = true;
                    break;
                }

                if color == "1" {
                    result.push_str(color);
                    found = true;
                    break;
                }
            }
            if !found {
                result.push_str(" ");
            }
        }
    }

    print_layer(&result, width, height);

    Ok(())
}

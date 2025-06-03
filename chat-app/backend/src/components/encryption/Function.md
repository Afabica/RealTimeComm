# Cheat Sheet of Function in Rust 

`from_str_radix` - function in Rust is used to convert a string to an integer for a specific radix. This function is part of the Num trait and is typically used for converting strings to integers in bases from 2 to 36



## For cycle method 

`for (i, hex_str) in block.iter().enumerate()` - this for-loop in Rust that iterated over a collection (block) while keeping track of both:
`i` index
`hex_str` - currrent element.
`block.iter().enumerate()` - iterates over block, using .enumerate() to get both index and value.

## Defining File Path in Rust.
Example of code:

<mark>

use std::fs;

fn main() {
    let file_path = "path/to/your/file.txt";
    let contents = fs::read_to_string(file_path);
        .expect("Something when wrong reading the file");
    println!("File contents:\n{}", contents);
}

</mark>

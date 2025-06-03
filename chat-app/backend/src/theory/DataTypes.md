# Data types of rust

## Use of arrays

`let mut s_box: Vec<u8> = vec![0; 256];` // Initialize with 256 zeros
Pros:

- Allows dynamic initialization
- Similar lookup speed as an array O(1).
  Cons:
- Stored on the heap (slightly slower than arrays);

## [[u8; 16]; 16] - best for 2d represantion

Somtimes the s-box is represented as a 16x16 matrix.
Pros:

- Matches AES representation (16x16 grid)
- Still provides to iterate over than a flat array.
  Example:
  const S_BOX_2D: [[u8; 16]; 16] = {};

Summary:
[u8; 256] - best for performance & security. Fast lookup, memory efficient, stack allocated. Fixed size;
Vec<u8> - best for dynamic allocation. Dynamic allocation. Dynamic initialization. Heap allocated.
[[u8; 16], 16] - best for 2d representation. Matches AES spec. More complex to index.

For most AES implementation, a fixed [u8; 16] is the best choice, If you need dynamic computation, use Vec<u8>

# A simple function returning a Result

Ok and Err methods use in places where in return is Result
like in this examples:
<mark>

fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, String> {
if divisor == 0.0 {
Err("Division by zero".to_string())
} else {

Ok(dividend / divisor)
}
}

fn main() {
match safe_divide(10.0, 2.0) {
Ok(result) => println!("Result: {}", result),
Err(e) => println!("Error: {}", e),
}

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

}
</mark>

If function calls another function that returns a Result, you can use the ? operator to propogate errors like in given example: 
<mark>

use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // If File::open fails, the error is returned immediately.
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Likewise for read_to_string.
    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File Contents:\n{}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}


</mark>


## Custom Error Types 

It is possible to define own error type of using String or io::Error. For example: 

<mark>

#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

fn custom_divide(dividend: f64, divisor: f64) -> Result<f64, MathError> {
    if divisor == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    match custom_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Custom Error: {:?}", e),
    }
}

</mark>

Summary:
- Ok(value) indicates a successful result.
- Err(error) inidicates an error.
- Use pattern matching to handle both cases.
- The ? operator helps propogate errors automatically.
- You can define custom error types to handle different error scenarios.


# Exporting variables, functions, structs and enums 

`pub` - public for all modules (global).
`pub(crate)` - only accessible inside the same crate.
`pub(self)` - only accessible inside the current module.
`pub(super)` - accessible from the parent module 
`pub(in path)` - accessible only inside a specific module 

## Example like that 
<mark> 
pub struct Example {
    pub(crate) id: u32, // Accessible only within the crate 
    pub(super) name: String, // Accessible from the parent module
    pub(self) private_field: bool, // Only inside this module
}

</mark>

## Operations with files  
`File::open(filename)` - open files
`let mut reader = BufReader::new(file)` - instantiate which has interior buffer.
`let mut buffer = [0u8; BLOCK_SIZE]` - 256-bit block (320byte) with data type like 0 as an unsigned 8-bit integer and use in this context like initialier of array.


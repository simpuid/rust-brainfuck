# rust-brainfuck
A brainfuck interpreter written in rust.

## Compiling
Compile the program with `cargo build` or compile and run with `cargo run`

## Running
Launch the executable in `target` directory and pass the paths to brainfuck sources. Example `$ ./target/debug/rust-brainfuck <source files seperated with spaces>` or `cargo run <source files seperated with spaces>`

## Notes
Cells can be dynamically alloted from both the direction.  
Cell size is of u8 (8-bit) 
Inputs and outputs are treated as chars.  

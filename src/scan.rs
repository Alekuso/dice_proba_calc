/*
This macro is kinda like the equivalent of scanf on C.

USAGE:
let a = scan!(i32); // scan, expect a number
let b = scan!(String); // scan, expect a String
*/

macro_rules! scan {
    ($x:ty) => {{
        use std::io;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse::<$x>().expect("Invalid input")
    }};
}

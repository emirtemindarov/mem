This crate provides macro shorthand for displaying size of given value in bytes. This can be handy when you want to write less code for debugging to determine how much space is allocated for a variable.

Example:
```
#[macro_use]
extern crate mem_println;
use mem_println::mem;

fn foo() {
    let value: i32 = 10;
    mem!(value);      // println!("{}", mem::size_of_val(&value));
}
```
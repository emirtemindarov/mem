This crate provides macro shorthand for displaying size of given value in bytes. This can be handy when you want to write less code for debugging to determine how much space is allocated for a variable.

Examples:

```
#[macro_use]
extern crate mem_println;
use mem_println::mem;

fn foo() {
    let value: i32 = 10;
    mem!(value);      // println!("{}", mem::size_of_val(&value));
}

fn bar() {
    let value: &str = "some_text";
    assert_eq!(16, get_mem!(value));
}
```

Each macro is defined as a `feature`. 
All features are enabled by default.
If you don't want to use all the features, you can disable the `default` features and then enable all the necessary ones.

```
[dependencies]
mem_println = { version = "0.1.5", default-features = false, features = ["get_size_of_val"] }
```

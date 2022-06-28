/// Prints the size of the given value to stdout
#[cfg(feature = "println_mem")]
#[macro_export]
macro_rules! mem {
($value: ident) => {{
        use std::mem;
        println!("{}", mem::size_of_val(&$value));
    }};
}

/// Returns the size of the given number
#[cfg(feature = "get_size_of_val")]
#[macro_export]
macro_rules! get_mem {
($value: ident) => {{
        use std::mem;
        mem::size_of_val(&$value)
    }};
}

#[cfg(test)]
mod tests {
    use std::mem;
    #[test]
    fn int() {
        let value1: i8 = 10;
        let value2: i16 = 10;
        let value3: i32 = 10;
        assert_eq!(1, get_mem!(value1));
        assert_eq!(2, get_mem!(value2));
        assert_eq!(4, get_mem!(value3));
    }
    #[test]
    fn unit_struct() {
        use std::mem;
        #[derive(Debug)]
        struct UnitStruct;
        let value = UnitStruct;
        mem!(value);
        assert_eq!(0, get_mem!(value));
    }
    #[test]
    fn get_mem_size() {
        let value: &str = "some_text";
        assert_eq!(16, get_mem!(value));
    }
}

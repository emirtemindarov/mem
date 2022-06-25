#[cfg(feature = "println_mem")]
#[macro_export]
macro_rules! mem {
($value: ident) => {{
        use std::mem;
        println!("{}", mem::size_of_val(&$value));
    };};
}

#[cfg(test)]
mod tests {
    #[test]
    fn integer() {
        let value1: i8 = 6;
        let value2: i16 = 7;
        let value3: i32 = 8;
        mem!(value1);
        mem!(value2);
        mem!(value3);
    }
    #[test]
    fn structure() {
        #[derive(Debug)]
        struct UnitStruct;
        let value = UnitStruct;
        mem!(value);
        assert_eq!(0, mem::size_of_val(&value));
    }
}

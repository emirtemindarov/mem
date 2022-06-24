#[cfg(feature = "println_mem")]
#[macro_export]
macro_rules! mem {
($value: ident) => {
        use std::mem;
        println!("{}", mem::size_of_val(&$value));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn integer() {
        let number = 7;
        mem!(number);
        assert_eq!(4, mem::size_of_val(&number));
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

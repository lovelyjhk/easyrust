
#[macro_export]
macro_rules! duplicate_function {
    ($($item:tt)*) => ($($item)*);
}

#[macro_export]
macro_rules! my_function {
    () => {
        println!("This is my function");
    };
}
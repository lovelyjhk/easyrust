fn handle_option_value(x: Option<i32>) {
    match x {
        Some(i) => println!("Value is {}", i),
        None => println!("No value"),
    }
}

pub fn var_2_33() {
    let value = Some(42);
    handle_option_value(value); // Output: Value is 42

    let no_value: Option<i32> = None;
    handle_option_value(no_value); // Output: No value
}

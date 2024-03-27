fn handle_result_value(x: Result<i32, &str>) {
    match x {
        Ok(i) => println!("Success: {}", i),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn var_2_34() {
    let success_value: Result<i32, &str> = Ok(42);
    handle_result_value(success_value); // Output: Success: 42

    let error_value: Result<i32, &str> = Err("Something went wrong");
    handle_result_value(error_value); // Output: Error: Something went wrong
}

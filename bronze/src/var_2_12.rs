pub fn var_2_12() {

    //제어문 match
    let day = "Wednesday";
    
    match day {
    "Monday" => println!("It's the start of the week!"),
    "Friday" => println!("Weekend is almost here!"),
    "Saturday" | "Sunday" => println!("It's the weekend!"),
    _ => println!("It's a regular day."),
    }
}
    
    
    
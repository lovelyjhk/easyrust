enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn var_2_32() {
    // 각각의 코인을 value_in_cents 함수에 전달하여 해당하는 가치를 출력합니다.
    println!("Value of Penny: {}", value_in_cents(Coin::Penny));
    println!("Value of Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Value of Dime: {}", value_in_cents(Coin::Dime));
    println!("Value of Quarter: {}", value_in_cents(Coin::Quarter));
}

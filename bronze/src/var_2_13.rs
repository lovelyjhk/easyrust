pub fn var_2_13() {
    
    //1.loop
    let mut count = 0;
    loop {
        println!("무한 반복중 {}", count);
        count += 1;
        if count == 5 {
            break; // count가 5가 되면 탈출
        }
    }

    //2.while
    let mut count = 0;
    while count <= 5 {
        println!("while 반복중 ....{}",count);
        count += 1 ; // count = count + 1
    }

    //3. for in
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("for 반복 중... {}", number);
    }

}
        
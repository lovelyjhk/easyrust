pub fn var_3_21() {
    // 정수 데이터를 힙에 할당하고 Box 포인터에 담기
    let x = Box::new(42);
    
    // Box 포인터로부터 값을 가져오기
    println!("Value of x: {}", *x);
       
    // 함수에 Box 포인터 전달하기
    print_box(x);
   
}

fn print_box(data: Box<i32>) {
    println!("Value inside print_box: {}", *data);
}

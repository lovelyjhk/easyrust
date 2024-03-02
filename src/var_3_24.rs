use std::rc::Rc;

fn main() {
    // Rc 포인터를 사용하여 정수 데이터를 공유하기
    let data = Rc::new(42);
    
    // Rc 포인터를 복제하여 두 개의 소유자를 만들기
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);
    
    // 두 개의 Rc 포인터를 사용하여 데이터에 접근하기
    println!("Value of data1: {}", *data1);
    println!("Value of data2: {}", *data2);
}

use std::ops::Deref;

// MyBox 구조체 정의
struct MyBox<T>(T);

// MyBox에 대한 Deref 트레이트 구현
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    // MyBox를 사용하여 정수 데이터를 감싸기
    let y = MyBox(x);
    
    // MyBox를 참조자처럼 사용하여 값을 해제하고 출력하기
    println!("Value of y: {}", *y);
}

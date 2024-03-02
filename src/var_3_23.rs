// Drop 트레이트를 구현하는 구조체 정의
struct MyStruct {
    data: String,
}

// MyStruct에 대한 Drop 트레이트 구현
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
        // 여기서 추가적인 정리 코드를 실행할 수 있음
    }
}

fn main() {
    {
        // 메모리가 정리되는 순간에 Drop 트레이트가 실행됨
        let _obj = MyStruct { data: String::from("Hello, Rust!") };
    } // 스코프를 벗어나면서 Drop 트레이트가 실행됨
}

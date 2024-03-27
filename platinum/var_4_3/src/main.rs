// main.rs

// proc_macro 폴더의 function_like_macro 모듈을 가져옵니다.
mod proc_macro {
    pub mod attribute_macro;
    pub mod function_like_macro;
    pub mod procedural_macro;
}

// 사용할 매크로를 가져옵니다.
use proc_macro::attribute_macro;
use proc_macro::function_like_macro;
use proc_macro::procedural_macro;

#[derive(Debug)]
struct MyStruct {
    field: i32,
}
fn main() {

    //1. attribute_macro
    let my_struct = MyStruct { field: 42 };

    // debug_print 매크로를 사용하여 구조체를 디버그 출력합니다.
    debug_print!(MyStruct);
    debug_print(&my_struct);


    //2. function_like_macro (사각형 넓이 구하기)
    
    let num = 5;
    let squared_num = square!(num);
    println!("Square of {} is {}", num, squared_num);


    //3. procedural_macro.rs
    // duplicate_function! 매크로를 사용하여 함수를 두 번 호출합니다.
    duplicate_function! {
        fn my_function() {
            println!("This is my function");
        }
    }

    // my_function! 매크로를 호출합니다.
    my_function!();


}


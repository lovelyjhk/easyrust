// src/proc_macro/attribute_macro.rs

#[macro_export]
macro_rules! debug_print {
    ($struct_name:ident) => {
        // 주어진 구조체에 대한 debug_print 함수를 생성합니다.
        pub fn debug_print(instance: &$struct_name) {
            println!("{:?}", instance);
        }
    };
}

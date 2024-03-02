use std::fs::File;

pub fn var_3_11() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

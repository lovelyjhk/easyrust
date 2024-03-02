use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::io::Seek;
use std::fs::metadata;
use std::fs::set_permissions;

/// 파일의 내용을 출력하는 함수입니다.
fn print_file_content(file_path: &PathBuf) {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("File contents: {}", contents),
        Err(error) => println!("Failed to read file: {}", error),
    }
}

/// 파일의 내용을 수정하고 새로운 내용을 추가하는 함수입니다.
fn modify_file_content(file_path: &PathBuf, new_content: &str) {
    let mut file = match OpenOptions::new().read(true).write(true).open(file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            // 파일 내용 수정
            contents.push_str(new_content);

            // 파일 포인터를 파일의 시작으로 이동
            match file.seek(std::io::SeekFrom::Start(0)) {
                Ok(_) => {
                    // 수정된 내용으로 파일에 쓰기
                    match file.write_all(contents.as_bytes()) {
                        Ok(_) => println!("File content modified successfully"),
                        Err(error) => println!("Failed to write to file: {}", error),
                    }
                },
                Err(error) => println!("Failed to seek file pointer: {}", error),
            }
        },
        Err(error) => println!("Failed to read file: {}", error),
    }
}

/// 파일을 생성하고 쓰는 예제
pub fn var_3_20_1() {
    let file_path = PathBuf::from("./new_file.txt");
    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to create file: {}", error);
            return;
        }
    };

    match file.write_all(b"Hello, Rust!") {
        Ok(_) => println!("File created and written successfully"),
        Err(error) => println!("Failed to write to file: {}", error),
    }
}

/// 파일 조회하는 예제
pub fn var_3_20_2() {
    let file_path = PathBuf::from("./new_file.txt");
    println!("File opened successfully");
    print_file_content(&file_path);
}

/// 파일 수정하는 예제
pub fn var_3_20_3() {
    let file_path = PathBuf::from("./new_file.txt");
    modify_file_content(&file_path, "\n러스트는 배울수록 재밌을거에요!");
}


/// 기타 파일 속성 조회 및 변경하는 예제
pub fn var_3_20_4() {
    let file_path = PathBuf::from("./new_file.txt");
    
    match metadata(&file_path) {
        Ok(metadata) => {
            println!("File size: {} bytes", metadata.len());

            match set_permissions(&file_path, metadata.permissions()) {
                Ok(_) => println!("File permissions set successfully"),
                Err(error) => println!("Failed to set file permissions: {}", error),
            }
        },
        Err(error) => println!("Failed to retrieve file metadata: {}", error),
    }
}

/// 파일 삭제하는 예제
pub fn var_3_20_5() {
    let file_path = PathBuf::from("./new_file.txt");
    match fs::remove_file(&file_path) {
        Ok(_) => println!("File deleted successfully"),
        Err(error) => println!("Failed to delete file: {}", error),
    }
}
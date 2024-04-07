use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {

    // 키워드 정의
    let keywords = vec!["rust", "programming", "file", "example"]; // 예시 키워드

    // 파일 경로 설정
    let file_path = "example.txt"; // 메모장 파일 경로

    // 파일 열기
    let file = File::open(file_path).expect("파일을 열 수 없습니다.");
    let reader = BufReader::new(file);

    // 키워드 개수를 저장할 해시맵 생성
    let mut keyword_counts: HashMap<String, usize> = HashMap::new(); // 키워드의 소문자 형태를 저장하기 위해 String 타입으로 변경

    // 파일 내용 읽기 및 키워드 개수 계산
    for line in reader.lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                // 소문자로 변환하여 키워드 확인
                let lowercase_word = word.to_lowercase();
                if let Some(count) = keyword_counts.get_mut(&lowercase_word) {
                    *count += 1;
                } else {
                    keyword_counts.insert(lowercase_word, 1);
                }
            }
        }
    }

    // 결과 출력
    println!("키워드 개수:");
    for (keyword, count) in &keyword_counts {
        println!("{}: {}", keyword, count);
    }
}

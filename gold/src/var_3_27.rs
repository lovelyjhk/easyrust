/*
 13-2. 단어 맞추기 추리 게임 개발
 */
use std::io;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;

const WORDS: [&str; 5] = ["apple", "banana", "orange", "grape", "melon"];
const TIME_LIMIT_SECONDS: u64 = 30;

pub fn var_3_27() {
    let word = WORDS.choose(&mut rand::thread_rng()).unwrap();
    let word_length = word.len();

    println!("게임을 시작합니다!");
    println!("단어의 글자 수: {}", word_length);

    println!("제한 시간: {} 초", TIME_LIMIT_SECONDS);

    println!("단어를 추측하여 입력하세요:");

    let start_time = Instant::now();
    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => {
            let elapsed_time = start_time.elapsed().as_secs();

            if guess.trim() == *word && elapsed_time <= TIME_LIMIT_SECONDS {
                println!("정답입니다! 걸린 시간: {} 초", elapsed_time);
            } else {
                println!("오답이거나 시간 초과입니다.");
            }
        }
        Err(error) => println!("입력 오류: {}", error),
    }
}


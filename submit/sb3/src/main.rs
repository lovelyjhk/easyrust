use std::io;
use rand::Rng;

fn main() {
    println!("게임 시작!");

    // 한 번만 생성되어야 할 무작위 숫자
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        println!("숫자를 입력하세요: ");
        io::stdin().read_line(&mut guess)
            .expect("입력을 읽지 못했습니다.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("올바른 숫자를 입력하세요.");
                continue;
            }
        };

        println!("Input: {}", guess);
        println!("Random: {}", secret_number);

        if guess > secret_number {
            println!("result : down");
        } else if guess < secret_number {
            println!("result : up");
        } else {
            println!("result : true");
            break;
        }
    }
}

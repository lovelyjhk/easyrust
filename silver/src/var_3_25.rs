use std::cell::RefCell;
use std::io;

//1. RefCell<T>를 사용한 내부 가변성 실습
pub fn var_3_25_1() {
    // 벡터 생성 및 RefCell<T>로 래핑
    let data = RefCell::new(vec![1, 2, 3]);

    // 불변 참조를 통해 값을 읽고 출력
    let immutable_ref = data.borrow();
    println!("불변 참조: {:?}", *immutable_ref);

    // try_borrow_mut을 사용하여 불변 참조가 있는지 확인한 후에 가변 참조를 시도함
    if let Ok(mut mutable_ref) = data.try_borrow_mut() {
        mutable_ref.push(4);
        println!("가변 참조: {:?}", *mutable_ref);
    } else {
        println!("불변 참조가 이미 존재하여 가변 참조를 얻을 수 없습니다.");
    };
}



 //2. 내부 가변성 패턴을 적용한 데이터 구조 설계 실습
pub fn var_3_25_2() {
    // 사용자 입력을 받아서 RefCell<T>로 래핑된 데이터 구조 생성
    let data = RefCell::new(Vec::new());

    loop {
        println!("명령을 입력하세요 (add: 값을 추가, print: 값 출력, exit: 종료):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");
        let command = input.trim();

        match command {
            "add" => {
                println!("추가할 값을 입력하세요:");
                let mut value_input = String::new();
                io::stdin().read_line(&mut value_input).expect("입력을 읽을 수 없습니다.");
                let value = value_input.trim().parse::<i32>().expect("유효한 숫자를 입력하세요.");
                data.borrow_mut().push(value);
            }
            "print" => {
                println!("현재 값들: {:?}", *data.borrow());
            }
            "exit" => {
                println!("프로그램을 종료합니다.");
                break;
            }
            _ => {
                println!("알 수 없는 명령입니다.");
            }
        }
    }
}


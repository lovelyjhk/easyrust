// 강아지 구조체 정의
struct Dog {
    name: String,
    age: u8,
    breed: String,
}

// Dog 구조체에 대한 메소드 구현
impl Dog {
    // 새로운 강아지 생성하는 함수
    fn new(name: &str, age: u8, breed: &str) -> Dog {
        Dog {
            name: String::from(name),
            age,
            breed: String::from(breed),
        }
    }

    // 강아지 정보 출력하는 메소드
    fn print_info(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Breed: {}", self.breed);
    }
}

pub fn var_2_8() {
    // 새로운 강아지 인스턴스 생성
    let dog1 = Dog::new("멍멍이", 2, "진도개");
    let dog2 = Dog::new("댕댕이", 4, "시베리안 허스키");

    // 강아지 정보 출력
    println!("Dog 1:");
    dog1.print_info();
    println!("Dog 2:");
    dog2.print_info();
}


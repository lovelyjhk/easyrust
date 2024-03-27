/*
(실습)
 14-2. 반복자: 일련의 아이템들 처리하기
Rust의 Iterator 트레이트를 구현하는 다양한 반복자들이 있습니다. 
각 반복자는 데이터를 생성하고 조작하는 다양한 방법을 제공합니다. 
이러한 반복자들은 표준 라이브러리의 일부로 제공되며, 
Rust 코드에서 주로 사용됩니다. 이제 몇 가지 일반적인 반복자와 각각의 사용법에 대해 살펴보겠습니다.
*/


//1. iter()
//사용법: .iter() 메서드는 컬렉션의 불변 참조를 통해 요소를 반복합니다.
pub fn var_3_32_1(){
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("{}", num);
    }
}
//2. iter_mut()
//사용법: .iter_mut() 메서드는 컬렉션의 가변 참조를 통해 요소를 반복합니다.
pub fn var_3_32_2(){
    let mut numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter_mut() {
        *num *= 2; // 각 요소를 2배로 변경
    }
    println!("{:?}", numbers); // 출력: [2, 4, 6, 8, 10]
}

//3. into_iter()
//사용법: .into_iter() 메서드는 컬렉션의 소유권을 소비하고 요소를 반복합니다.
pub fn var_3_32_3(){

    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.into_iter() {
        println!("{}", num);
    }
}
//4. range()
//사용법: range() 함수는 숫자 범위를 생성합니다.
pub fn var_3_32_4(){
    for num in 1..=5 {
        println!("{}", num);
    }
}


//5. map()
//사용법: .map() 메서드는 각 요소를 변환하여 새로운 반복자를 생성합니다.

pub fn var_3_32_5(){
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled_numbers: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled_numbers); // 출력: [2, 4, 6, 8, 10]
}
//6. filter()
//사용법: .filter() 메서드는 조건에 맞는 요소만 선택하여 새로운 반복자를 생성합니다.

pub fn var_3_32_6(){
    let numbers = vec![1, 2, 3, 4, 5];

    let even_numbers: Vec<_> = numbers.iter().filter(|x| * x % 2 == 0).collect();
    println!("{:?}", even_numbers); // 출력: [2, 4]
}

//7. enumerate()
//사용법: .enumerate() 메서드는 요소의 인덱스와 값을 튜플로 반환합니다.


pub fn var_3_32_7(){
    let numbers = vec![1, 2, 3, 4, 5];

    for (index, num) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, num);
    }
}



//8. 그외 기타 
pub fn var_3_32_8(){    
    // 벡터 생성
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter(): 짝수만 필터링하여 새로운 벡터 생성
    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", even_numbers); // [2, 4, 6, 8, 10]

    // any(): 짝수가 있는지 확인
    let has_even_number = numbers.iter().any(|&x| x % 2 == 0);
    println!("Has even number: {}", has_even_number); // true

    // all(): 모든 요소가 10보다 작은지 확인
    let all_less_than_ten = numbers.iter().all(|&x| x < 10);
    println!("All less than ten: {}", all_less_than_ten); // true

    // fold(): 모든 요소의 합 계산
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum); // 55

    // next(): 다음 요소를 반환합니다. 반복이 끝나면 None을 반환합니다.
    let mut iter = numbers.iter();
    println!("Next element: {:?}", iter.next()); // Some(1)
    println!("Next element: {:?}", iter.next()); // Some(2)
    println!("Next element: {:?}", iter.next()); // Some(3)
    println!("Next element: {:?}", iter.next()); // Some(4)
    println!("Next element: {:?}", iter.next()); // Some(5)
    println!("Next element: {:?}", iter.next()); // Some(6)

    // size_hint(): 이 반복자가 알고 있는 요소의 최소 및 최대 길이를 반환합니다.
    let iter = numbers.iter();
    println!("Size hint: {:?}", iter.size_hint()); // (10, Some(10))

    // count(): 반복자의 요소 수를 계산합니다.
    let iter = numbers.iter();
    let count = iter.count();
    println!("Count: {}", count); // 10

    // collect(): 반복자의 모든 요소를 수집하여 새로운 컬렉션으로 변환합니다.
    let iter = numbers.iter();
    let doubled: Vec<i32> = iter.map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled); // [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]

    // map(): 각 요소에 함수를 적용하여 새로운 반복자를 생성합니다.
    let iter = numbers.iter();
    let mapped: Vec<_> = iter.map(|x| x.to_string()).collect();
    println!("Mapped: {:?}", mapped); // ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
}




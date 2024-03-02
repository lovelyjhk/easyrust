pub fn var_2_14(){
    //1. sort
    let mut numbers = vec![4, 2, 5, 1, 3]; 
    numbers.sort(); 
    for i in numbers.iter(){
        println!("sort 결과는?{}", i);
    }

    //2. binary search 
    let mut numbers = vec![1, 2, 3, 4, 5]; 
    let index = numbers.binary_search(&3);

    match index {
        Ok(i) => println!("찾은 값의 인덱스: {}, 값은 :{}", i, numbers[i]),
        Err(_) => println!("해당 값이 존재하지 않습니다."),
    }

    //3. reserve
    let mut numbers = vec![5, 4, 3, 2, 1]; 
    numbers.reverse(); 
    println!("reverse의 결과는 : {:?}", numbers);

    //4. 중복요소제거 dedup
    let mut numbers = vec![1,2,3,3,3];
    numbers.dedup();
    println!("dedup 중복제거 결과는:{:?}",numbers );

    //5. min max 최소 최대 구하기
    let numbers = vec![1, 2, 3, 4, 5];
    let min_num = numbers.iter().min();
    let max_num = numbers.iter().max();
    match min_num {
        Some(min) => println!("최소: {}", min),
        None => println!("배열이 비어있습니다."),
    }

    match max_num {
        Some(max) => println!("최대: {}", max),
        None => println!("배열이 비어있습니다."),
    }

}
  
//standard library 표준라이브러리 사용법 (주요한것)
use std::mem;
use std::cmp;
use std::marker;
use std::collections::{HashMap, BTreeMap, HashSet};
use std::time;
use std::ops;

pub fn core_modules_example() {
    // std::mem: 메모리 조작과 관련된 함수들을 제공합니다.
    let size_of_int = mem::size_of::<i32>();
    println!("Size of i32: {}", size_of_int);

    // std::cmp: 비교 연산을 위한 함수들을 제공합니다.
    let max_value = cmp::max(5, 10);
    println!("Max value: {}", max_value);

    // std::ops: 연산자 오버로딩과 관련된 트레이트를 제공합니다.
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl ops::Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn operator_overloading_example() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 3, y: 4 };
        let sum = point1 + point2;
        println!("Sum of points: {:?}", sum);
    }

    operator_overloading_example();

    // std::marker: 마커 트레이트와 관련된 모듈을 제공합니다.
    struct MyType;
    trait MyTrait: marker::Send + marker::Sync {}
    impl MyTrait for MyType {}

    // std::vec: 벡터 타입을 제공합니다.
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);

    // std::collections::HashMap: 해시맵 타입을 제공합니다.
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    println!("HashMap: {:?}", map);

    // std::collections::BTreeMap: 이진 탐색 트리 기반의 맵 타입을 제공합니다.
    let mut btree_map = BTreeMap::new();
    btree_map.insert("one", 1);
    btree_map.insert("two", 2);
    println!("BTreeMap: {:?}", btree_map);

    // std::collections::HashSet: 해시셋 타입을 제공합니다.
    let mut hash_set = HashSet::new();
    hash_set.insert(1);
    hash_set.insert(2);
    println!("HashSet: {:?}", hash_set);

    // std::time: 시간 관련 타입 및 함수들을 제공합니다.
    let now = time::Instant::now();
    println!("Current time: {:?}", now);

    // std::test: 테스트 작성 및 실행을 위한 기능을 제공합니다.
    // (테스트 코드는 여기에 작성)
}

pub fn var_2_15() {
    core_modules_example();
}

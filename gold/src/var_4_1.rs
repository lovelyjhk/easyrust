use std::time::Duration; // Duration을 사용하기 위해 추가

// 비동기 실행 패턴 예제:

// 비동기 함수 정의
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    // 비동기적으로 데이터를 가져오는 HTTP 요청
    let response = reqwest::get(url).await?;
    // HTTP 응답의 본문을 문자열로 변환하여 반환
    response.text().await
}

#[tokio::main]
pub async fn var_4_1_1() {
    // 비동기 함수 호출
    let data = fetch_data("https://www.hackershrd.com/").await;
    match data {
        Ok(response) => println!("Response: {}", response),
        Err(err) => eprintln!("Error: {}", err),
    }
}

// 스트림과 파이프라인 패턴 예제:

// 비동기 스트림을 생성하는 함수
async fn generate_data() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[tokio::main]
pub async fn var_4_1_2() {
    // 비동기 스트림 생성
    let stream = generate_data().await.into_iter().map::<Result<i32, reqwest::Error>, _>(Ok);

    // 스트림의 각 요소를 필터링하여 새로운 벡터를 생성합니다.
    let result: Vec<i32> = stream
        .filter_map(|num: Result<i32, _>| { // 타입 어노테이션 추가
            match num {
                Ok(num) => {
                    if num % 2 == 0 {
                        Some(num * 2)
                    } else {
                        None
                    }
                }
                Err(_) => None, // 에러가 발생하면 필터링하여 결과에서 제외합니다.
            }
        })
        .collect::<Vec<_>>(); // 타입 추론 문제 해결을 위해 타입 어노테이션 추가

    println!("Result: {:?}", result);
}

// 확장성과 병렬성 패턴 예제:
use tokio::time::sleep;

// 확장성과 병렬성 패턴 예제:

async fn process_data(data: i32) -> i32 {
    // 간단한 시뮬레이션을 위해 1초 대기
    sleep(Duration::from_secs(1)).await;
    data * 2
}

#[tokio::main]
pub async fn var_4_1_3() {
    // 병렬로 실행할 데이터 리스트
    let data_list = vec![1, 2, 3, 4, 5];

    // 데이터를 병렬로 처리하고 결과를 수집
    let results: Vec<i32> = futures::future::join_all(data_list.into_iter().map(|data| async move {
        process_data(data).await
    })).await;

    println!("Processed results: {:?}", results);
}

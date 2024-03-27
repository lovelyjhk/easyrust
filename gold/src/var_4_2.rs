use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use reqwest;


pub fn var_4_2() -> Result<(), Box<dyn std::error::Error>> {
    // 다운로드할 파일 URL들
    let urls = vec![
        "https://rustacean.net/assets/rustacean-orig-noshadow.png",
        "https://rustacean.net/assets/rustacean-flat-noshadow.png",
        "https://rustacean.net/assets/rustacean-flat-happy.png",
    ];


    // 파일을 저장할 디렉터리 경로
    let output_dir = "./";


    // HTTP 클라이언트 생성
    let client = reqwest::blocking::Client::new();


    // 각 URL로부터 파일 다운로드
    for url in urls {
        // URL로 GET 요청 보내기
        let response = client.get(url).send()?;


        // 요청이 성공하면 파일 저장
        if response.status().is_success() {
            // 파일 이름 추출
            let file_name = url.split('/').last().unwrap_or("unknown_file");


            // 파일 경로 생성
            let file_path = format!("{}/{}", output_dir, file_name);


            // 파일 생성
            let file = File::create(&file_path)?;
            let mut writer = BufWriter::new(file);


            // 응답 본문을 파일에 복사
            io::copy(&mut BufReader::new(response), &mut writer)?;


            println!("다운로드 완료: {}", file_path);
        } else {
            eprintln!("다운로드 실패: {}", url);
        }
    }


    Ok(())
}

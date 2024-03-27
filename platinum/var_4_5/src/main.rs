use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use chrono::prelude::*;


macro_rules! input_task {
    ($num:expr) => {{
        println!("{}번째 업무를 입력하세요:", $num);
        input_task!()
    }};
    () => {{
        let mut task_input = String::new();
        io::stdin().read_line(&mut task_input)?;
        task_input.trim().to_string()
    }};
}


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "./monthly_report.txt";
    let mut file = File::create(file_path)?;


    let mut input_line = String::new();
    let mut input = String::new();
    let mut planning = String::new();


    let mut internal_count = 0; // 내부 카운트를 선언합니다.


    loop {
        println!("금일 업무를 몇 번 입력하시겠습니까?");
        io::stdin().read_line(&mut input_line)?;
        input.clear();
        input.push_str(&input_line);


        let count: usize = match input.trim().parse() {
            Ok(0) => {
                println!("업무 횟수는 0보다 커야 합니다.");
                continue;
            },
            Ok(num) => num,
            Err(_) => {
                println!("숫자를 입력하세요.");
                continue;
            }
        };


        for i in 0..count {
            let task_content = input_task!(i + 1);


            // 내부 카운트를 증가시킵니다.
            internal_count += 1;


            // 현재 날짜를 가져오기
            let current_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();


            // 작성자 정보
            let author = "김해커스";


            // 보고서 내용 작성
            let report_content = format!(
                "\
                ------------------------------------\n\
                일일 회의록\n\
                ------------------------------------\n\
                작성일자: {}\n\
                작성자: {}\n\
                프로젝트 A: 진행 상황 업데이트\n\
                - 고객과의 미팅 완료 ( xy 거래처 담당자 OOO 대면미팅) \n\
                - 기술협의 진행 \n\
                회의 요약\n\
                - 주간 프로젝트 회의 참석\n\
                - 위기 대응 관리 \n\
                행동 항목\n\
                - 고객과 전달물에 대한 후속 조치\n\
                - 다음 주 회의를 위한 발표 자료 준비\n\
                금일 업무 내용\n\
                - {} ({}/{})\n\
                ------------------------------------\n\
                ",
                current_date, author, task_content, i + 1, count
            );


            // 보고서 내용을 파일에 작성
            writeln!(file, "{}", report_content)?;
        }


        println!("금일 업무가 보고서에 추가되었습니다.");


        // 내부 카운트와 외부 카운트를 비교하여 루프를 종료합니다.
        if internal_count >= count {
            break;
        }


        println!("다음주 계획을 입력하세요:");
        io::stdin().read_line(&mut input)?;
        planning.push_str(&input);


        println!("더 입력하시겠습니까? (y/n):");
        input.clear();
        io::stdin().read_line(&mut input)?;
        let continue_input = input.trim().to_lowercase();


        if continue_input != "y" {
            println!("보고서가 저장되었습니다. 프로그램을 종료합니다.");
            break;
        }
    }


    Ok(())
}

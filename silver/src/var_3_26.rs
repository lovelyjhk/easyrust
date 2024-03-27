/*
  13-1. 중복파일 삭제 [유틸리티 제작]
*/
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub fn var_3_26() -> io::Result<()> {
    // 중복 파일을 찾을 디렉토리 설정
    let directory = "C:\\";

    // 중복 파일 찾기
    let duplicate_files = find_duplicates(directory)?;

    // 중복 파일 경로를 파일에 작성
    write_duplicate_paths_to_file("duplicate_files.txt", &duplicate_files)?;

    // 중복 파일 삭제 여부 확인 및 삭제 처리
    if ask_for_deletion() {
        delete_files(&duplicate_files)?;
        println!("중복 파일 삭제 완료");
    }

    Ok(())
}

fn find_duplicates(directory: &str) -> io::Result<HashMap<u64, Vec<PathBuf>>> {
    let mut file_hash_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_hash = hash_file(&path)?;
            file_hash_map.entry(file_hash).or_default().push(path);
        }
    }

    let duplicates: HashMap<u64, Vec<PathBuf>> = file_hash_map.into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect();

    Ok(duplicates)
}

fn hash_file(file_path: &Path) -> io::Result<u64> {
    let file_content = fs::read(file_path)?;
    let file_hash = xx_hash::xxh3::hash(&file_content);
    Ok(file_hash)
}

fn write_duplicate_paths_to_file(filename: &str, duplicate_files: &HashMap<u64, Vec<PathBuf>>) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    writeln!(file, "중복 파일 목록")?;

    for paths in duplicate_files.values() {
        for path in paths {
            writeln!(file, "{}", path.display())?;
        }
    }

    println!("중복 파일 목록을 {} 파일에 작성했습니다.", filename);
    Ok(())
}

fn ask_for_deletion() -> bool {
    print!("중복 파일을 삭제하시겠습니까? (y/n): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    answer.trim().to_lowercase() == "y"
}

fn delete_files(duplicate_files: &HashMap<u64, Vec<PathBuf>>) -> io::Result<()> {
    let mut total_deleted_size: u64 = 0;

    for paths in duplicate_files.values() {
        for path in paths {
            let file_size = path.metadata()?.len();
            fs::remove_file(&path)?;
            println!("{} 삭제됨", path.display());
            total_deleted_size += file_size;
        }
    }

    println!("삭제된 파일의 총 크기: {} 바이트", total_deleted_size);
    Ok(())
}


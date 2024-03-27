use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub fn var_3_39() {
    //make_srt_file(); //사전 srt만들기 
    make_video();
}

fn make_srt_file() {
    // Korean sentences
    let korean_sentences = vec![
        "안녕하세요 Rust를 배우러 오신 여러분들을 환영합니다!!",
        "이제부터 시작해보겠습니다.",
        "레벨업! 축하합니다!!",
    ];

    // English, Japanese, Chinese translations
    let english_sentences = vec![
        "Welcome everyone to learn Rust!!",
        "Let's get started now.",
        "Level up! Congratulations!!",
    ];

    let japanese_sentences = vec![
        "Rustを学びに来てくれてありがとう！",
        "これから始めます。",
        "レベルアップ！おめでとうございます！！",
    ];

    let chinese_sentences = vec![
        "欢迎大家来学习Rust！！",
        "现在开始吧。",
        "升级！恭喜你！！",
    ];

    // Generate SRT files
    generate_srt("korean.srt", &korean_sentences, 10);
    generate_srt("english.srt", &english_sentences, 3);
    generate_srt("japanese.srt", &japanese_sentences, 5);
    generate_srt("chinese.srt", &chinese_sentences, 5);
}

fn generate_srt(filename: &str, sentences: &[&str], duration: u64) {
    let mut file = File::create(filename).expect("Failed to create file");

    let mut start_time = 0;
    for (index, sentence) in sentences.iter().enumerate() {
        let end_time = start_time + duration;

        writeln!(file, "{}", index + 1).expect("Failed to write to file");
        writeln!(
            file,
            "{} --> {}",
            format_time(start_time),
            format_time(end_time)
        )
        .expect("Failed to write to file");
        writeln!(file, "{}", sentence).expect("Failed to write to file");
        writeln!(file).expect("Failed to write to file");

        start_time = end_time;
    }
}

fn format_time(time: u64) -> String {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = (time % 3600) % 60;
    format!("{:02}:{:02}:{:02},000", hours, minutes, seconds)
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
fn make_video() {
    let pool = ThreadPool::new(4);

    let subtitles = vec![
        ("korean.srt", "output_korean.mp4"),
        ("english.srt", "output_english.mp4"),
        ("chinese.srt", "output_chinese.mp4"),
    ];

    for (subtitle_file, output_file) in subtitles {
        let pool_clone = &pool;
        pool.execute(move || {
            add_subtitles_to_video("rust 동영상 자막.mp4", subtitle_file, output_file);
        });
    }
}

use std::process::{Command, Stdio};

fn add_subtitles_to_video(video_file: &str, subtitle_file: &str, output_file: &str) {
    let output = Command::new("C:\\ProgramData\\chocolatey\\bin\\ffmpeg.exe")
        .args(&["-i", video_file, "-vf", &format!("subtitles={}", subtitle_file), output_file])
        .stderr(Stdio::piped()) // stderr를 캡처
        .output();

    match output {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr)); // stderr 출력
        },
        Err(e) => eprintln!("Failed to execute process: {}", e),
    }
}
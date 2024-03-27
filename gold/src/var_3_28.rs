/*
13-3. 내 명함 이미지에 QR코드 합성하기
*/

use qrcodegen::{QrCode, QrCodeEcc};
use image::{Rgb, RgbImage, imageops::overlay, GenericImageView};
use std::io;

pub fn var_3_28() {
    // 사용자로부터 이미지 파일 경로 입력 받기
    println!("이미지 파일 경로를 입력하세요:");
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("입력을 읽을 수 없습니다.");

    // 입력 받은 경로에서 줄 바꿈 문자 제거
    let image_path = input_path.trim();

    // 이미지 파일 열기
    let background_img = image::open(image_path).expect("이미지 파일을 열 수 없습니다.");

    // 이미지의 너비와 높이 가져오기
    let (bg_width, bg_height) = background_img.dimensions();

    // 사용자로부터 URL 입력 받기
    println!("QR 코드에 담을 URL을 입력하세요:");
    let mut input_url = String::new();
    io::stdin().read_line(&mut input_url).expect("입력을 읽을 수 없습니다.");

    // 입력 받은 URL에서 줄 바꿈 문자 제거
    let url = input_url.trim();

    // QR 코드 생성
    let qr = QrCode::encode_text(url, QrCodeEcc::Medium).unwrap();

    // QR 코드 이미지 생성
    let qr_img_size = qr.size() as u32;
    let pixel_size = 10; // QR코드 이미지의 크기 
    let qr_rgb_img_size = qr_img_size * pixel_size;

    // QR 코드 이미지를 생성하고 색상 채우기
    let mut qr_rgb_img = RgbImage::new(qr_rgb_img_size, qr_rgb_img_size);
    for y in 0..qr_img_size {
        for x in 0..qr_img_size {
            let color = if qr.get_module(x as i32, y as i32) {
                Rgb([0, 0, 0]) // 검은색 모듈
            } else {
                Rgb([255, 255, 255]) // 흰색 배경
            };
            // 픽셀 크기만큼 반복하여 색상 채우기
            for i in 0..pixel_size {
                for j in 0..pixel_size {
                    qr_rgb_img.put_pixel(x * pixel_size + i, y * pixel_size + j, color);
                }
            }
        }
    }

    // QR 코드 이미지를 원본 이미지 위에 overlay
    let x = (bg_width - qr_rgb_img_size) - 153  ;
    let y = (bg_height - qr_rgb_img_size) / 2 + 100;

    let mut overlay_img = background_img.to_rgb8();
    overlay(&mut overlay_img, &qr_rgb_img, x.into(), y.into());

    // 이미지 파일로 저장
    let output_path = "output.png";
    overlay_img.save(output_path).expect("이미지를 저장할 수 없습니다.");
    println!("QR 코드를 원본 이미지 위에 output.png 저장했습니다.");
}

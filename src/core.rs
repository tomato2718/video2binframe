use crate::binframe::BinFrame;
use crate::image::yuv420;
use crate::video;
use std::fs;

pub fn run(input: &str, output: &str) {
    let mut capture = match video::VideoCapture::new(input) {
        Ok(capture) => capture,
        Err(e) => panic!("{e}"),
    };
    let (width, height) = (capture.width as usize, capture.height as usize);
    let mut binframe = BinFrame::new();
    binframe.write_signature(capture.width, capture.height, capture.fps);

    loop {
        match capture.extract() {
            Ok(Some(packet)) => {
                let buffers = capture.decode(&packet);
                buffers.into_iter().for_each(|buffer| {
                    let buffer = yuv420::to_binary_buffer(buffer, width, height);
                    binframe.write_run_length(&buffer);
                });
            }
            Ok(None) => {
                println!("File EOF");
                break;
            }
            Err(e) => {
                panic!("{e}");
            }
        };
    }
    fs::write(output, binframe.buffer()).unwrap();
}

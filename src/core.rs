use crate::video;

pub fn run(path: &str) {
    let mut capture = match video::VideoCapture::new(path) {
        Ok(capture) => capture,
        Err(e) => panic!("{e}"),
    };
    loop {
        match capture.extract() {
            Ok(p) => println!("{:?}", p),
            Err(e) => {
                println!("{e}");
                break;
            }
        };
    }
}

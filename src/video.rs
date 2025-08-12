use rsmpeg::{
    avcodec::{AVCodecContext, AVPacket},
    avformat::AVFormatContextInput,
    ffi::AVMEDIA_TYPE_VIDEO,
};
use std::ffi::CString;

type ErrorMessage = &'static str;
const ERR_OPEN_FILE: ErrorMessage = "Failed to open video";
const ERR_FAILED_TO_FIND_VIDEO: ErrorMessage = "Failed to find video stream";
const ERR_FAILED_TO_OPEN_DECODER: ErrorMessage = "Failed to open decoder";
const ERR_READING_FILE: ErrorMessage = "Error reading file";

pub struct VideoCapture {
    input: AVFormatContextInput,
    video_index: i32,
    decoder: AVCodecContext,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl VideoCapture {
    pub fn new(path: &str) -> Result<Self, ErrorMessage> {
        let context = VideoCapture::create_context(path)?;
        let (video_index, decoder) = VideoCapture::find_video_codec(&context)?;

        Ok(VideoCapture {
            input: context,
            video_index: video_index,
            width: decoder.width as u32,
            height: decoder.height as u32,
            fps: decoder.framerate.num as u32,
            decoder: decoder,
        })
    }

    fn create_context(path: &str) -> Result<AVFormatContextInput, ErrorMessage> {
        let cpath = match CString::new(path) {
            Ok(cpath) => cpath,
            Err(_) => return Err(ERR_OPEN_FILE),
        };
        match AVFormatContextInput::open(&cpath) {
            Ok(context) => Ok(context),
            Err(_) => Err(ERR_OPEN_FILE),
        }
    }

    fn find_video_codec(
        input: &AVFormatContextInput,
    ) -> Result<(i32, AVCodecContext), ErrorMessage> {
        let (index, codec) = match input.find_best_stream(AVMEDIA_TYPE_VIDEO) {
            Ok(Some(res)) => res,
            _ => return Err(ERR_FAILED_TO_FIND_VIDEO),
        };
        let mut decoder = AVCodecContext::new(&codec);
        decoder
            .apply_codecpar(&input.streams()[index].codecpar())
            .unwrap();
        match decoder.open(None) {
            Ok(_) => (),
            _ => return Err(ERR_FAILED_TO_OPEN_DECODER),
        };

        Ok((index as i32, decoder))
    }

    pub fn extract(&mut self) -> Result<Option<AVPacket>, &'static str> {
        loop {
            let packet = match self.input.read_packet() {
                Ok(Some(p)) => p,
                Ok(None) => return Ok(None),
                Err(_) => return Err(ERR_READING_FILE),
            };
            if packet.stream_index != self.video_index {
                continue;
            }
            return Ok(Some(AVPacket::from(packet)));
        }
    }

    pub fn decode(&mut self, packet: &AVPacket) -> Vec<Vec<u8>> {
        self.decoder
            .send_packet(Some(packet))
            .expect("Should be ok");
        let mut res = Vec::new();
        while let Ok(frame) = self.decoder.receive_frame() {
            let mut buffer = vec![0u8; frame.image_get_buffer_size(1).unwrap()];
            frame
                .image_copy_to_buffer(&mut buffer, 1)
                .expect("Should be ok");
            res.push(buffer)
        }
        res
    }
}

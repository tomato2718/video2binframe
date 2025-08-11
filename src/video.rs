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
const ERR_FILE_EOF: ErrorMessage = "File EOF";
const ERR_READING_FILE: ErrorMessage = "Error reading file";

pub struct VideoCapture {
    context: AVFormatContextInput,
    video_index: i32,
    decoder: AVCodecContext,
}

impl VideoCapture {
    pub fn new(path: &str) -> Result<Self, ErrorMessage> {
        let context = VideoCapture::create_context(path)?;
        let (video_index, decoder) = VideoCapture::find_video_codec(&context)?;

        Ok(VideoCapture {
            context,
            video_index,
            decoder,
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
        context: &AVFormatContextInput,
    ) -> Result<(i32, AVCodecContext), ErrorMessage> {
        let (index, codec) = match context.find_best_stream(AVMEDIA_TYPE_VIDEO) {
            Ok(Some(res)) => res,
            _ => return Err(ERR_FAILED_TO_FIND_VIDEO),
        };
        let mut decoder = AVCodecContext::new(&codec);
        match decoder.open(None) {
            Ok(_) => (),
            _ => return Err(ERR_FAILED_TO_OPEN_DECODER),
        };

        Ok((index as i32, decoder))
    }

    pub fn extract(&mut self) -> Result<AVPacket, &'static str> {
        loop {
            let packet = match self.context.read_packet() {
                Ok(Some(p)) => p,
                Ok(None) => return Err(ERR_FILE_EOF),
                Err(_) => return Err(ERR_READING_FILE),
            };
            if packet.stream_index != self.video_index {
                continue;
            }
            return Ok(AVPacket::from(packet));
        }
    }
}

pub struct BinFrame {
    buffer: Vec<u8>,
}

impl BinFrame {
    pub fn new() -> Self {
        BinFrame { buffer: Vec::new() }
    }

    pub fn write_signature(&mut self, width: u32, height: u32, fps: u32) {
        let mut signature = format!("{width}x{height}@{fps}fps").as_bytes().to_owned();
        if signature.len() > 24 {
            panic!("Max signature size exceed")
        }
        signature.resize(24, 0x00);
        self.buffer.extend(signature);
    }

    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_signature_given_width_and_height_should_write_signature_to_buffer() {
        let mut bin_frame = BinFrame::new();
        let mut want = b"480x360@30fps".to_vec();
        want.resize(24, 0x00);

        bin_frame.write_signature(480, 360, 30);

        let signature_bytes = &bin_frame.buffer()[..24];

        assert_eq!(signature_bytes, want);
    }
}

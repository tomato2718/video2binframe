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

    pub fn write_run_length(&mut self, buffer: &[u8]) {
        let mut run_length_buffer: Vec<u8> = Vec::new();
        let mut last_pixel = 0xff;
        for (current_index, current_pixel) in buffer.iter().enumerate() {
            if current_pixel != &last_pixel {
                run_length_buffer.extend_from_slice(&(current_index as u32).to_be_bytes());
                last_pixel = *current_pixel;
            }
        }
        run_length_buffer.extend_from_slice(&(buffer.len() as u32).to_be_bytes());
        if run_length_buffer.len() > (u32::MAX >> 2) as usize {
            panic!("Buffer size exceed");
        }
        let mut header_bytes = (run_length_buffer.len() as u32).to_be_bytes();
        header_bytes[0] &= 0x3f;
        header_bytes[0] |= 0x80;
        self.buffer.extend(header_bytes);
        self.buffer.extend(run_length_buffer);
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

    #[test]
    fn write_run_length_given_buffer_should_write_to_frame_buffer() {
        let mut bin_frame = BinFrame::new();

        bin_frame.write_run_length(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ]);

        assert_eq!(
            bin_frame.buffer(),
            &[
                0x80, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
                0x00, 0x18,
            ]
        );
    }
}

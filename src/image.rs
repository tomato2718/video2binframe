pub mod yuv420 {
    pub fn to_binary_buffer(yuv_buffer: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
        let y_size = width * height;
        let y_plane = &yuv_buffer[0..y_size];
        let mut binary_buffer = vec![0u8; width * height];

        for y in 0..height {
            for x in 0..width {
                let y_idx = y * width + x;
                let y_val = y_plane[y_idx];

                if y_val > 0x7f {
                    binary_buffer[y_idx] = 0xff;
                } else {
                    binary_buffer[y_idx] = 0x00;
                }
            }
        }

        binary_buffer
    }
}

#[cfg(test)]
mod test_yuv420 {
    use super::*;

    #[test]
    fn to_binary_buffer_given_yuv_buffer_should_return_binary_buffer() {
        let test_case: Vec<u8> = vec![
            // Y plane
            200, 50, 180, 30, 60, 220, 40, 200, 150, 80, 170, 100, 20, 240, 10, 250,
            // U, V plane
            128, 128, 128, 128, 128, 128, 128, 128,
        ];

        let want: Vec<u8> = vec![
            255, 0, 255, 0, 0, 255, 0, 255, 255, 0, 255, 0, 0, 255, 0, 255,
        ];

        let res = yuv420::to_binary_buffer(test_case, 4, 4);

        res.into_iter()
            .zip(want)
            .for_each(|(res, expect)| assert_eq!(res, expect));
    }
}

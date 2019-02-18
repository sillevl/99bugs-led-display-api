

pub fn check_png_header(data: &[u8]) -> Result<(), String> {
    let png_header: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    for i in 0..8 {
        if data[i] != png_header[i] {
            return Err("headers do not match".to_string());
        }
    }
    Ok(())
}
pub fn read_varint(data: &[u8], index: Option<i32>) -> Option<(u16, i32)> {

    let mut index: i32 = index.unwrap_or(0);
    let mut number_read: i32 = 0;
    let mut result: u16 = 0x00;

    loop {
        let byte: u8 = data[index as usize];
        index += 1;

        let value: u8 = byte & 0x7F; // Removing continuation bit
        result |= (value as u16) << (7 * number_read); // Appending to the variable
        number_read += 1;

        if (byte & 0x80) == 0 { // Checking continutation bit
            break;
        }

        if number_read > 2 {
            return None;
        }
    }
    Some((result, index))
}

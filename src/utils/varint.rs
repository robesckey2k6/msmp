pub fn read_varint(data: &[u8], index: Option<i32>) -> Option<(u8, i32)> {
    let mut index: i32 = index.unwrap_or(0);

    let mut number_read: i32 = 0;
    let mut result: u8 = 0x00;

    loop {
        let byte: u8 = data[index as usize];
        index += 1;
        result = byte & 0b01111111;
        result |= result << (7 * number_read); // TODO: Fix this, as its only 8 bits data is lost
                                               // while parsing protocol version
                                               // might cuase issues later

        number_read += 1;

        if (byte & 0b10000000) == 0 {
            break;
        }

        if number_read > 5 {
            return None;

        }

    }
    Some((result, index))
}

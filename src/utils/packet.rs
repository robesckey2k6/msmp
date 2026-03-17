use super::varint;

/*
*/

#[warn(unused_assignments)]
pub fn parse_handshake_data(pak_buffer: &[u8]) -> (u64, u64, u64, String, u64, u64) {

    let mut index_ = 0;
    let packet_length: u64;
    let packet_id: u64;
    let protocol_version: u64;
    let address_length: u64;
    let port: u64 = 000;
    let intent: u64;

    (packet_length, index_) = varint::read_varint(pak_buffer, Some(index_)).unwrap();
    (packet_id, index_) = varint::read_varint(pak_buffer, Some(index_)).unwrap();
    (protocol_version, index_) = varint::read_varint(pak_buffer, Some(index_)).unwrap();
    (address_length, index_) = varint::read_varint(pak_buffer, Some(index_)).unwrap();

    let end_index = (index_ + (address_length as i32)) as usize;
    let us_index = index_ as usize;

    let raw_svaddr = pak_buffer[us_index .. end_index].to_vec();

    let sv_address = String::from_utf8(raw_svaddr).unwrap();

    let port_end_index = (end_index + 2) as usize;

     // TODO get port number let portasb: [u8; 8] = pak_buffer[end_index .. port_end_index].try_into().unwrap();
    // port = u64::from_le_bytes(portasb);

    let (intent, _) = varint::read_varint(pak_buffer, Some(port_end_index as i32)).unwrap();

    (
        packet_length,
        packet_id,
        protocol_version,
        sv_address,
        port,
        intent
    )

}


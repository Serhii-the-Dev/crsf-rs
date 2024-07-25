//! ParameterRead packet and related functions/implementations

/// ParameterRead payload length
pub const LEN: usize = 2;

/// Represents a ParameterRead packet
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ParameterRead {
    pub field: u8,
    pub chunk: u8
}

/// The raw decoder (parser) for the ParameterRead packet.
pub fn raw_decode(data: &[u8; LEN]) -> ParameterRead {
    return ParameterRead {
        field: data[0],
        chunk: data[1],
    }
}

/// The raw encoder (serializer) for the ParameterRead packet.
pub fn raw_encode(device_ping: &ParameterRead, data: &mut [u8; LEN]) {
    data[0] = device_ping.field;
    data[1] = device_ping.chunk;
}

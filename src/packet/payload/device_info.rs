//! DeviceInfo packet and related functions/implementations

/// DeviceInfo payload length
pub const LEN: usize = 58;

const DEVICE_NAME_MAX_LEN: usize = 44;

/// Represents a DeviceInfo packet
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceInfo {
    pub display_name: &'static str,
    pub serial_number: u32,
    pub hardware_version: u32,
    pub software_version: u32,
    pub parameter_count: u8,
    pub parameter_protocol_version: u8,
}

/// The raw decoder (parser) for the DeviceInfo packet.
pub fn raw_decode(data: &[u8; LEN]) -> DeviceInfo {
    let name_bytes: &mut [u8] = &mut [];
    for i in 0..DEVICE_NAME_MAX_LEN {
        if data[i] == 0 {
            break;
        }
        name_bytes[i] = data[i];
    }

    return DeviceInfo {
        display_name: core::str::from_utf8(name_bytes).unwrap(),
        serial_number: 0,
        hardware_version: 0,
        software_version: 0,
        parameter_count: 0,
        parameter_protocol_version: 0,
    };
}

/// The raw encoder (serializer) for the DeviceInfo packet.
pub fn raw_encode(packet: &DeviceInfo, data: &mut [u8; LEN]) {}

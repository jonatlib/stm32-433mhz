use bitfield_struct::bitfield;
use defmt::trace;

use sequence_number::SequenceNumber;

#[cfg(feature = "packet-32")]
pub type PacketType = Packet32;
#[cfg(feature = "packet-64")]
pub type PacketType = Packet64;

#[cfg(feature = "packet-32")]
pub const PACKET_TYPE_SN_SIZE: u8 = 8;
#[cfg(feature = "packet-64")]
pub const PACKET_TYPE_SN_SIZE: u8 = 32;

#[derive(Debug, Eq, PartialEq, Clone, defmt::Format)]
#[repr(u8)]
pub enum PacketKind {
    SelfContained = 0,
    Start = 1,
    Continue = 2,
    End = 3,
    Unsupported,
}

impl From<PacketKind> for u32 {
    fn from(value: PacketKind) -> Self {
        value as u32
    }
}

impl From<PacketKind> for u64 {
    fn from(value: PacketKind) -> Self {
        value as u64
    }
}

impl From<u32> for PacketKind {
    fn from(value: u32) -> Self {
        match value {
            0 => PacketKind::SelfContained,
            1 => PacketKind::Start,
            2 => PacketKind::Continue,
            3 => PacketKind::End,
            _ => PacketKind::Unsupported,
        }
    }
}

impl From<u64> for PacketKind {
    fn from(value: u64) -> Self {
        match value {
            0 => PacketKind::SelfContained,
            1 => PacketKind::Start,
            2 => PacketKind::Continue,
            3 => PacketKind::End,
            _ => PacketKind::Unsupported,
        }
    }
}

#[bitfield(u32)]
#[derive(PartialEq)]
pub struct Packet32 {
    #[bits(2)]
    pub kind: PacketKind,
    #[bits(3)] // Up to 8 packets
    pub sequence_number: SequenceNumber<8>,
    #[bits(2)] // Up to 4 streams
    pub stream_id: SequenceNumber<8>,

    #[bits(4)] // Up to 16 devices
    pub source_address: u8,
    #[bits(4)]
    pub destination_address: u8,

    #[bits(16)]
    pub payload: u16,

    #[bits(1)]
    pub payload_used_index: u8,
}

impl Packet32 {
    pub fn to_le_bytes(self) -> [u8; 4] {
        Into::<u32>::into(self).to_le_bytes()
    }

    pub fn to_be_bytes(self) -> [u8; 4] {
        Into::<u32>::into(self).to_be_bytes()
    }

    pub fn update_crc(&mut self) {}

    pub fn validate(&self) -> bool {
        true
    }

    #[inline]
    pub const fn size() -> usize {
        4
    }
}

impl From<u64> for Packet32 {
    fn from(value: u64) -> Self {
        (value as u32).into()
    }
}

impl defmt::Format for Packet32 {
    fn format(&self, fmt: defmt::Formatter) {
        let payload = self.payload().to_le_bytes();
        defmt::write!(
            fmt,
            "Packet32 {{ kind: {:?}, sequence_number: {}, stream_id: {}, source_address: {:#04x}, destination_address: {:#04x}, payload: [{:#04x}, {:#04x}], payload_used_index: {} }}",
            self.kind(),
            self.sequence_number(),
            self.stream_id(),
            self.source_address(),
            self.destination_address(),
            payload[0],
            payload[1],
            self.payload_used_index(),
        )
    }
}

#[bitfield(u64)]
#[derive(PartialEq)]
pub struct Packet64 {
    #[bits(2)]
    pub kind: PacketKind,
    #[bits(4)] // Up to 16 packets
    pub sequence_number: SequenceNumber<16>,
    #[bits(3)] // Stream identification
    pub stream_id: SequenceNumber<8>,

    #[bits(4)] // Up to 16 devices
    pub source_address: u8,
    #[bits(4)]
    pub destination_address: u8,

    #[bits(40)] // 5bytes
    pub payload: u64,

    #[bits(3)]
    pub payload_used_index: u8,

    #[bits(4)]
    pub crc4: u8,
}

impl Packet64 {
    pub fn compute_crc4(&self) -> u8 {
        const CRC4_TABLE: [u8; 16] = [
            0x0, 0x7, 0xe, 0x9, 0xb, 0xc, 0x5, 0x2, 0x1, 0x6, 0xf, 0x8, 0xa, 0xd, 0x4, 0x3,
        ];
        const CRC4_START: u8 = 0x01;
        const CRC4_BITS_TO_CHECK: u8 = 60;

        let mut value: u64 = self.with_crc4(0x0).into();
        // mask off anything above the top bit
        value &= (1u64 << CRC4_BITS_TO_CHECK) - 1;
        // Align to 4-bits
        let bits: u8 = (CRC4_BITS_TO_CHECK + 3) & !0x3;

        // Calculate crc4 over four-bit nibbles, starting at the MSbit
        let mut index: i16 = bits as i16 - 4;
        let mut crc = CRC4_START;
        while index >= 0 {
            crc = CRC4_TABLE[(crc ^ ((value >> index) & 0xfu64) as u8) as usize];
            index -= 4;
        }

        return crc;
    }

    pub fn with_updated_crc(&self) -> Self {
        let mut packet: Packet64 = self.clone();
        packet.set_crc4(self.compute_crc4());
        return packet;
    }

    pub fn update_crc(&mut self) {
        self.set_crc4(self.compute_crc4());
    }

    pub fn validate(&self) -> bool {
        let expected = self.compute_crc4();
        expected == self.crc4()
    }

    pub fn to_le_bytes(self) -> [u8; 8] {
        Into::<u64>::into(self).to_le_bytes()
    }

    #[inline]
    pub const fn size() -> usize {
        8
    }
}

impl defmt::Format for Packet64 {
    fn format(&self, fmt: defmt::Formatter) {
        let payload = self.payload().to_le_bytes();
        defmt::write!(
            fmt,
            "Packet64 {{ kind: {:?}, sequence_number: {}, stream_id: {}, source_address: {:#04x}, destination_address: {:#04x}, payload: [{:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}], payload_used_index: {}, crc4: {:#04x} }}",
            self.kind(),
            self.sequence_number(),
            self.stream_id(),
            self.source_address(),
            self.destination_address(),
            payload[0],
            payload[1],
            payload[2],
            payload[3],
            payload[4],
            self.payload_used_index(),
            self.crc4(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_packet64() {
        let packet_data = 0x1234_1234_1234_1234u64;
        let mut packet64 = Packet64::from(packet_data);
        assert!(!packet64.validate());

        packet64.update_crc();
        assert!(packet64.validate());

        let crc = packet64.crc4();
        packet64.update_crc();
        assert_eq!(crc, packet64.crc4());
    }

    #[test]
    fn test_packet32_from_u64() {
        let original_packet = Packet32::new()
            .with_source_address(0x05)
            .with_destination_address(0x08);

        let packet_data_32: u32 = original_packet.into();
        let packet_data_64: u64 = (packet_data_32 as u64) << 32;

        let received_packet = Packet32::from(packet_data_64);
        assert_eq!(received_packet, original_packet);
    }
}

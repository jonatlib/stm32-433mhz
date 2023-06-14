use bitfield_struct::bitfield;

use sequence_number::SequenceNumber;

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

#[bitfield(u32)]
pub struct Packet32 {
    #[bits(2)]
    pub kind: PacketKind,
    #[bits(3)] // Up to 8 packets
    pub sequence_number: SequenceNumber<8>,

    #[bits(5)] // Up to 32 devices
    pub source_address: u8,
    #[bits(5)]
    pub destination_address: u8,

    #[bits(16)]
    pub payload: u16,

    #[bits(1)]
    pub both_bytes_used: bool,
}

impl defmt::Format for Packet32 {
    fn format(&self, fmt: defmt::Formatter) {
        let payload = self.payload().to_be_bytes();
        defmt::write!(
            fmt,
            "Packet32 {{ kind: {:?}, sequence_number: {}, source_address: {:#04x}, destination_address: {:#04x}, payload: [{:#04x}, {:#04x}], both_bytes_used: {} }}",
            self.kind(),
            self.sequence_number(),
            self.source_address(),
            self.destination_address(),
            payload[0],
            payload[1],
            self.both_bytes_used(),
        )
    }
}

#[bitfield(u64)]
pub struct Packet64 {
    #[bits(2)]
    pub kind: PacketKind,
    #[bits(5)] // Up to 32 packets
    pub sequence_number: SequenceNumber<32>,

    #[bits(5)] // Up to 32 devices
    pub source_address: u8,
    #[bits(5)]
    pub destination_address: u8,

    #[bits(40)] // 5bytes
    pub payload: u16,

    #[bits(3)]
    pub payload_used_index: u8,

    #[bits(4)]
    pub crc4: u8,
}

impl Packet64 {
    pub fn update_crc(&mut self) {
        todo!()
    }
}

impl defmt::Format for Packet64 {
    fn format(&self, fmt: defmt::Formatter) {
        let payload = self.payload().to_be_bytes();
        defmt::write!(
            fmt,
            "Packet64 {{ kind: {:?}, sequence_number: {}, source_address: {:#04x}, destination_address: {:#04x}, payload: [{:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}], payload_used_index: {}, crc4: {:#04x} }}",
            self.kind(),
            self.sequence_number(),
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

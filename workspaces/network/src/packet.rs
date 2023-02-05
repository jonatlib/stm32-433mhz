use bitfield_struct::bitfield;

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
    pub sequence_number: u8,

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

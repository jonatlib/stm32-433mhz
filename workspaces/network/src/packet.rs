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
        let mut index = bits - 4;
        let mut crc = CRC4_START;
        while index >= 0 {
            crc = CRC4_TABLE[(crc ^ ((value >> index) & 0xfu64) as u8) as usize];
            index -= 4;
        }

        return crc;
    }

    pub fn update_crc(&mut self) {
        self.set_crc4(self.compute_crc4());
    }

    pub fn validate(&self) -> bool {
        let expected = self.compute_crc4();
        expected == self.crc4()
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

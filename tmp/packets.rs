use postcard::{from_bytes, to_vec};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
pub enum PacketTypes {
    START,
    DATA([u8; 8]),
    RST,
    FIN,
}

impl PacketTypes {
    fn get_number(&self) -> u8 {
        match self {
            Self::START => 1,
            Self::DATA(_) => 2,
            Self::RST => 3,
            Self::FIN => 4,
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Packet {
    source_address: u8,
    destination_address: u8,

    stream_number: u8,
    sequence_number: u8,

    packet_type: PacketTypes,
    checksum: u8,
}

impl Packet {
    pub fn new(
        source_address: u8,
        destination_address: u8,
        stream_number: u8,
        sequence_number: u8,
        packet_type: PacketTypes,
    ) -> Self {
        let data: [u8; 5] = [
            source_address,
            destination_address,
            stream_number,
            sequence_number,
            packet_type.get_number(),
        ];
        let checksum: u8 = crc8_rs::fetch_crc8(data, 0xd5);

        return Self {
            source_address,
            destination_address,
            stream_number,
            sequence_number,
            packet_type,
            checksum,
        };
    }

    pub fn validate(&self) -> bool {
        let data: [u8; 5] = [
            self.source_address,
            self.destination_address,
            self.stream_number,
            self.sequence_number,
            self.packet_type.get_number(),
        ];
        return crc8_rs::fetch_crc8(data, 0xd5) == self.checksum;
    }
}

pub fn serialize_packets_builder(
    source_address: u8,
    destination_address: u8,
    stream_number: u8,
    data: &impl Serialize,
) -> impl Iterator<Item = Packet> {
    let serialized_data: heapless::Vec<u8, heapless::consts::U16> = to_vec(data).unwrap();

    return BytesPacketBuilder::new(
        source_address,
        destination_address,
        stream_number,
        serialized_data.into_iter(),
    );
}


struct BytesPacketBuilder<T>
where
    T: Iterator<Item = u8>
{
    source_address: u8,
    destination_address: u8,
    stream_number: u8,

    sequence_number: u8,
    last_sent: bool,
    data: T
}

impl<T> BytesPacketBuilder<T>
where
    T: Iterator<Item = u8>
{
    pub fn new(
        source_address: u8,
        destination_address: u8,
        stream_number: u8,
        data: T,
    ) -> Self {
        BytesPacketBuilder {
            source_address,
            destination_address,
            stream_number,

            sequence_number: 0,
            last_sent: false,
            data,
        }
    }
}

impl<T> Iterator for BytesPacketBuilder<T>
where
    T: Iterator<Item = u8>
{
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sequence_number == 0 {
            let packet = Some(Packet::new(
                self.source_address,
                self.destination_address,
                self.stream_number,
                0,
                PacketTypes::START,
            ));
            self.sequence_number += 1;
            return packet;
        }

        let mut packet_data: [u8; 8] = [0; 8];
        let mut zeros: u8 = 0;

        for index in 0..8 {
            if let Some(v) = self.data.next() {
                packet_data[index] = v;
            } else {
                packet_data[index] = 0;
                zeros += 1;
            }
        }

        if zeros < 8 {
            if self.sequence_number > 250 {
                panic!("Maximum number of packets reached!");
            }

            let packet = Some(Packet::new(
                self.source_address,
                self.destination_address,
                self.stream_number,
                self.sequence_number,
                PacketTypes::DATA(packet_data),
            ));
            self.sequence_number += 1;
            return packet;
        } else {
            if !self.last_sent {
                let packet = Some(Packet::new(
                    self.source_address,
                    self.destination_address,
                    self.stream_number,
                    self.sequence_number,
                    PacketTypes::FIN,
                ));
                self.last_sent = true;
                return packet;
            }
        }

        return None;
    }
}


pub fn packets_get_bytes(packets: impl Iterator<Item = Packet>) -> heapless::Vec<u8, heapless::consts::U16> {
    let mut result = heapless::Vec::new();
    for packet in packets {
        if !packet.validate() {
            panic!("Invalid packet!");
        }
        match packet.packet_type {
            PacketTypes::DATA(bytes) => {
                for v in &bytes[..] {
                    result.push(*v).unwrap();
                }
            }
            _ => {}
        };
    }
    return result;
}

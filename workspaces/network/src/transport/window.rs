use crate::error::NetworkError;
use crate::packet::{Packet32, PacketKind};

pub struct Window<const SIZE: usize> {
    buffer: heapless::Vec<Packet32, SIZE>,
}

impl<const SIZE: usize> Window<SIZE> {
    pub fn new() -> Self {
        Self {
            buffer: heapless::Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn push_packet(&mut self, packet: Packet32) -> Result<Option<usize>, NetworkError> {
        if self.buffer.is_empty() {
            self.buffer
                .push(packet)
                .expect("We now the vector is empty");
        }

        let current_packet_sequence_number = packet.sequence_number();
        let index = self
            .buffer
            .iter()
            .map(|packet| packet.sequence_number())
            .filter(|sequence_number| )
            .enumerate()
            .fold(0usize, |acc, (index, sequence_number)| {
                if sequence_number > current_packet_sequence_number {
                    return index;
                }

                return acc;
            });

        todo!();

        if self.is_completely_received() {
            return Ok(Some(self.buffer.len()));
        }

        Ok(None)
    }

    pub fn write_buffer(&self, buffer: &mut [u8]) -> Result<(), ()> {
        if !self.is_completely_received() {
            return Err(());
        }

        for (index, packet) in self.buffer.iter().enumerate() {
            let bytes = packet.payload().to_be_bytes();

            buffer[index * 2] = bytes[0];
            if packet.both_bytes_used() {
                buffer[(index * 2) + 1] = bytes[1];
            }
        }

        Ok(())
    }

    fn is_completely_received(&self) -> bool {
        if self.buffer.len() < 1 {
            return false;
        }

        if self.buffer.len() == 1 && matches!(self.buffer[0].kind(), PacketKind::SelfContained) {
            return true;
        }

        if self.buffer.len() > 1 {
            if matches!(self.buffer[0].kind(), PacketKind::Start)
                && matches!(
                    self.buffer
                        .last()
                        .expect("There should be at least one element.")
                        .kind(),
                    PacketKind::End
                )
            {
                let expected_offset = self.buffer[0].sequence_number() as isize;
                // We have first and last packet.
                // Be we should also have the intermittent packets
                return self
                    .buffer
                    .iter()
                    .enumerate()
                    .map(|(index, packet)| {
                        // This offset should be constant if we have all packets
                        // And also positive
                        (packet.sequence_number() as isize - index as isize)
                    })
                    .all(|offset| offset > 0 && offset == expected_offset);
            }
        }

        false
    }
}

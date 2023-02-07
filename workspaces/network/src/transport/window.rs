use crate::error::{DataConstructionError, NetworkError};
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
        // FIXME we can see single packet multiple times, then just ignore the retransmission.

        if self.buffer.is_empty() {
            self.buffer
                .push(packet)
                .expect("We know the vector is empty");
        } else {
            // TODO maybe when receiving packet with SN so high it can't be in current sequence,
            // TODO just return error

            // FIXME sequence numbers should be extracted to struct so they can
            // FIXME wrap around, use struct with cmp implemented
            // FIXME if we implement From<u32> for that struct we can use it in packet
            let current_packet_sequence_number = packet.sequence_number();
            let last_sequence_number = self
                .buffer
                .last()
                .expect("We have at least one element.")
                .sequence_number();

            if last_sequence_number < current_packet_sequence_number {
                self.buffer.push(packet).map_err(|_| {
                    NetworkError::DataConstructingError(DataConstructionError::FullWindow)
                })?;
            } else {
                let index = self
                    .buffer
                    .iter()
                    .map(|packet| packet.sequence_number())
                    .enumerate()
                    .fold(0usize, |acc, (index, sequence_number)| {
                        if current_packet_sequence_number > sequence_number {
                            return index;
                        }

                        acc
                    });

                self.buffer.insert(index, packet).map_err(|_| {
                    NetworkError::DataConstructingError(DataConstructionError::FullWindow)
                })?;
            }
        }

        // defmt::info!("--------------------------------");
        // for packet in self.buffer.iter() {
        //     defmt::info!("{:?}", packet);
        // }
        // defmt::info!("--------------------------------");

        if self.is_completely_received() {
            return Ok(Some(self.received_bytes()));
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

    fn received_bytes(&self) -> usize {
        self.buffer
            .iter()
            .map(|v| if v.both_bytes_used() { 2 } else { 1 })
            .sum()
    }

    fn is_completely_received(&self) -> bool {
        if self.buffer.is_empty() {
            return false;
        }

        if self.buffer.len() == 1 && matches!(self.buffer[0].kind(), PacketKind::SelfContained) {
            return true;
        }

        if self.buffer.len() > 1
            && matches!(self.buffer[0].kind(), PacketKind::Start)
            && matches!(
                self.buffer
                    .last()
                    .expect("There should be at least one element.")
                    .kind(),
                PacketKind::End
            )
        {
            // We have first and last packet.
            // Be we should also have the intermittent packets
            // FIXME compute distances between sequence numbers are all exactly `1`
            return true;
        }

        false
    }
}

use crate::error::{DataConstructionError, NetworkError};
use crate::packet::{PacketKind, PacketType, PACKET_TYPE_SN_SIZE};
use sequence_number::SequenceNumber;

pub struct Window<const SIZE: usize> {
    buffer: heapless::Vec<PacketType, SIZE>,
    base_received: bool,
}

impl<const SIZE: usize> Window<SIZE> {
    pub fn new() -> Self {
        Self {
            buffer: heapless::Vec::new(),
            base_received: false,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn push_packet(&mut self, packet: PacketType) -> Result<Option<usize>, NetworkError> {
        // FIXME when to return Error earlier then when the buffer is full?

        if self.buffer.is_empty() {
            // TODO? || matches!(packet.kind(), PacketKind::End)
            self.buffer.push(packet).map_err(|_| {
                NetworkError::DataConstructingError(DataConstructionError::FullWindow)
            })?;
            // TODO?
            // } else if matches!(packet.kind(), PacketKind::Start) {
            //     self.buffer.insert(0, packet).map_err(|_| {
            //         NetworkError::DataConstructingError(DataConstructionError::FullWindow)
            //     })?;
        } else {
            let base = self.get_base_sequence_number();

            // Base received for the first time
            // Check if have packet sorted correctly
            if !self.base_received {
                if let Some(ref base_sequence_number) = base {
                    if !base_sequence_number
                        .is_sorted_asc(self.buffer.iter().map(|v| v.sequence_number()))
                    {
                        self.buffer.sort_unstable_by(|a, b| {
                            a.sequence_number()
                                .compare(&b.sequence_number(), base_sequence_number)
                        });
                    }
                    self.base_received = true;
                }
            }

            let sequence_numbers = self.buffer.iter().map(|v| v.sequence_number());
            let index = packet
                .sequence_number()
                .get_insertion_order_ascending(sequence_numbers, base.as_ref());

            if let Some(i) = index {
                self.buffer.insert(i, packet).map_err(|_| {
                    NetworkError::DataConstructingError(DataConstructionError::FullWindow)
                })?;
            } else {
                // TODO received same packet again
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

        let mut index = 0usize;
        for packet in self.buffer.iter() {
            let used_bytes_len = packet.payload_used_index() as usize + 1;
            let all_bytes = packet.payload().to_le_bytes();
            let bytes = &all_bytes[..used_bytes_len];

            for packet_index in 0usize..used_bytes_len {
                buffer[index] = bytes[packet_index];
                index += 1;
            }
        }

        Ok(())
    }

    fn get_base_sequence_number(&self) -> Option<SequenceNumber<PACKET_TYPE_SN_SIZE>> {
        if matches!(self.buffer[0].kind(), PacketKind::Start) {
            Some(self.buffer[0].sequence_number())
        } else {
            None
        }
    }

    fn received_bytes(&self) -> usize {
        self.buffer
            .iter()
            .map(|v| v.payload_used_index() as usize + 1)
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
            let mut sequence_numbers = self.buffer.iter();
            let mut prev_sequence_number = sequence_numbers
                .next()
                .expect("There is at least one packet")
                .sequence_number();

            return sequence_numbers
                .map(|v| v.sequence_number())
                .map(|v| -> u8 {
                    let distance = prev_sequence_number.positive_distance(&v);
                    prev_sequence_number = v;
                    distance
                })
                .all(|v| v == 1);
        }

        false
    }
}

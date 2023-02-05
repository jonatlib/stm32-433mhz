use crate::transport::window::Window;
use crate::Address;

pub struct TransportReader<'a, R, C> {
    address: Address,
    window: Window<8>,

    codec: &'a C,
    reader: &'a mut R,
}

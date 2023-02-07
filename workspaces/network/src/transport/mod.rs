use crate::error::NetworkError;

mod window;

pub mod reader;
pub mod writer;

pub trait TransportReceiver {
    async fn receive_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;

    async fn receive_struct<P>(&mut self) -> Result<P, NetworkError>
    where
        P: for<'a> serde::Deserialize<'a>,
    {
        let mut buffer = [0u8; 16]; // Packet32 can hold only up to 16bytes
        let read_bytes = self.receive_bytes(&mut buffer).await?;

        postcard::from_bytes(&buffer[..read_bytes]).map_err(NetworkError::ReceiverEncodingError)
    }
}

pub trait TransportSender {
    async fn send_bytes(&mut self, payload: &[u8]) -> Result<usize, NetworkError>;

    async fn send_struct<P>(&mut self, payload: &P) -> Result<usize, NetworkError>
    where
        P: serde::Serialize,
    {
        let mut buffer = [0u8; 16]; // With Packet32 we can encode only up to 16bytes

        let data_slice =
            postcard::to_slice(payload, &mut buffer).map_err(NetworkError::SenderEncodingError)?;

        self.send_bytes(data_slice).await
    }
}

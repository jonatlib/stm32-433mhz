use crate::error::NetworkError;

mod window;

pub mod reader;
pub mod writer;

pub trait TransportReceiver {}

pub trait TransportSender {
    async fn send_bytes(&mut self, payload: &[u8]) -> Result<usize, NetworkError>;

    async fn send_struct<P, const SIZE: usize>(
        &mut self,
        payload: &P,
    ) -> Result<usize, NetworkError>
    where
        P: serde::Serialize,
    {
        let mut buffer: [u8; SIZE] = [0; SIZE];

        postcard::to_slice(payload, &mut buffer)
            .map_err(|v| NetworkError::SenderEncodingError(v))?;

        self.send_bytes(&buffer).await
    }
}

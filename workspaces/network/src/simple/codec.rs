use codec::reed_solomon::ReedSolomon;
use codec::Identity;

pub type CodecFactoryType = ReedSolomon<4, 4>;
pub type CompressionFactoryType = Identity;

pub fn create_codec() -> CodecFactoryType {
    CodecFactoryType::default()
}

pub fn create_compression() -> CompressionFactoryType {
    CompressionFactoryType::default()
}

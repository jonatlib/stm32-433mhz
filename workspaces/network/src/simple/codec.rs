use codec::reed_solomon::ReedSolomon;

pub type CodecFactoryType = ReedSolomon<4>;

pub fn create_codec() -> CodecFactoryType {
    CodecFactoryType::default()
}

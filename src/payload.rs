#[derive(serde::Serialize, serde::Deserialize, defmt::Format, Debug, Clone)]
pub struct SensorPayload {
    pub timestamp: u32, // 4bytes

    pub temperature_1: f32, // 4bytes
    pub temperature_2: f32, // 4bytes
    pub humidity: u8,       // 1bytes
}

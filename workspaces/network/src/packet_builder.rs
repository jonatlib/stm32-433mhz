#[cfg(feature = "packet-32")]
pub mod packet32_builder;
#[cfg(feature = "packet-64")]
pub mod packet64_builder;

#[cfg(feature = "packet-32")]
pub use packet32_builder::PacketBuilder;
#[cfg(feature = "packet-64")]
pub use packet64_builder::PacketBuilder;

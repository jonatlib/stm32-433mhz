#[cfg(feature = "stm32l053r8")]
pub mod stm32l053r8;
#[cfg(feature = "stm32l053r8")]
pub use stm32l053r8 as enabled_chip;
// End of chip definitions

// Re-exporting
pub use enabled_chip::io;
pub use enabled_chip::Hardware;

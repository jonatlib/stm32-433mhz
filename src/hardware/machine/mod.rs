#[cfg(feature = "nucleo_stm32l053r8")]
pub mod nucleo_stm32l053r8;
#[cfg(feature = "nucleo_stm32l053r8")]
pub use nucleo_stm32l053r8 as enabled_chip;
// End of chip definitions

// Re-exporting
pub use enabled_chip::io;
pub use enabled_chip::Hardware;

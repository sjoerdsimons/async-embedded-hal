//! Re-exports of all of the async extension traits.
pub use crate::gpio::InputPinExt as _;
pub use crate::gpio::IntoFloatingInputPin as _;
pub use crate::gpio::IntoOpenDrainOutputPin as _;
pub use crate::gpio::IntoPullDownInputPin as _;
pub use crate::gpio::IntoPullUpInputPin as _;
pub use crate::gpio::IntoPushPullOutputPin as _;
pub use crate::gpio::OutputPinExt as _;
pub use crate::i2c::I2cBusMappingExt as _;
pub use crate::i2c::I2cReadExt as _;
pub use crate::i2c::I2cWriteExt as _;
pub use crate::io::ReadExt as _;
pub use crate::io::WriteExt as _;
pub use crate::timer::IntoOneshotTimer as _;
pub use crate::timer::IntoPeriodicTimer as _;
pub use crate::timer::TimerExt as _;

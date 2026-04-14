use core::convert::Infallible;

use jelly_lib::i2c_hal::I2cHal;
use jelly_mem_access::MemAccess;

use crate::i2c::{I2cAccess, JellyI2c};

/// Device-oriented adapter for `JellyI2c` with a fixed I2C slave address.
pub struct JellyI2cDevice<const DEV_ADR: u8, T: MemAccess> {
    bus: JellyI2c<T>,
}

impl<const DEV_ADR: u8, T: MemAccess> JellyI2cDevice<DEV_ADR, T> {
    pub const fn new(bus: JellyI2c<T>) -> Self {
        Self { bus }
    }

    pub const fn from_parts(reg_acc: T, wait_irq: Option<fn()>) -> Self {
        Self {
            bus: JellyI2c::new(reg_acc, wait_irq),
        }
    }

    pub fn bus(&self) -> &JellyI2c<T> {
        &self.bus
    }

    pub fn into_inner(self) -> JellyI2c<T> {
        self.bus
    }
}

impl<const DEV_ADR: u8, T: MemAccess> I2cHal for JellyI2cDevice<DEV_ADR, T> {
    type Error = Infallible;

    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        Ok(self.bus.write(DEV_ADR, data))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(self.bus.read(DEV_ADR, buf))
    }
}
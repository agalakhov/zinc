// Zinc, the bare metal stack for rust.
// Copyright 2016 Alexey "Gall" Galakhov <agalakhov@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Peripheral clock management routines.
//!
//! This module should be considered private until further notice.
//!
//! Note: this module is used as part of initial setup if PLL is used.

use super::init::{ClockConfig, reg};
use core::marker::Copy;

pub use self::PeripheralClock::*;

#[path="../../util/ioreg.rs"] mod ioreg;

#[allow(missing_docs)]
#[repr(u8)]
#[derive(Clone)]
pub enum BusAhb {
  Dma1,
  Dma2,
  Sram,
  Flitf,
  Fmc,
  Crc,
  GpioH,
  GpioA,
  GpioB,
  GpioC,
  GpioD,
  GpioE,
  GpioF,
  GpioG,
  Tsc,
  Adc12,
  Adc34,
}

impl Copy for BusAhb {}

impl BusAhb {
  fn to_reg_bit(self) -> u32 {
    use self::BusAhb::*;
    1 << match self {
      Dma1       => 0,
      Dma2       => 1,
      Sram       => 2,
      Flitf      => 4,
      Fmc        => 5,
      Crc        => 6,
      GpioH      => 16,
      GpioA      => 17,
      GpioB      => 18,
      GpioC      => 19,
      GpioD      => 20,
      GpioE      => 21,
      GpioF      => 22,
      GpioG      => 23,
      Tsc        => 24,
      Adc12      => 28,
      Adc34      => 29,
    }
  }

  fn set_reg(self, enable: bool) {
    let reg_bit = self.to_reg_bit();
    let mask: u32 = !reg_bit;
    let bit: u32 = if enable {reg_bit} else {0};
    let val = reg::RCC.ahbenr.enable() & mask;
    reg::RCC.ahbenr.set_enable(val | bit);
  }
}

#[allow(missing_docs)]
#[repr(u8)]
#[derive(Clone)]
pub enum BusApb1 {
  Tim2,
  Tim3,
  Tim4,
  Tim6,
  Tim7,
  Wwdg,
  Spi2,
  Spi3,
  Usart2,
  Usart3,
  Uart4,
  Uart5,
  I2C1,
  I2C2,
  Usb,
  Can,
  Dac2,
  Pwr,
  Dac1,
  I2C3,
}

impl BusApb1 {
  fn to_reg_bit(self) -> u32 {
    use self::BusApb1::*;
    1 << match self {
      Tim2       => 0,
      Tim3       => 1,
      Tim4       => 2,
      Tim6       => 4,
      Tim7       => 5,
      Wwdg       => 11,
      Spi2       => 14,
      Spi3       => 15,
      Usart2     => 17,
      Usart3     => 18,
      Uart4      => 19,
      Uart5      => 20,
      I2C1       => 21,
      I2C2       => 22,
      Usb        => 23,
      Can        => 25,
      Dac2       => 26,
      Pwr        => 28,
      Dac1       => 29,
      I2C3       => 30,
    }
  }

  fn set_reg(self, enable: bool) {
    let reg_bit = self.to_reg_bit();
    let mask: u32 = !reg_bit;
    let bit: u32 = if enable {reg_bit} else {0};
    let val = reg::RCC.apb1enr.enable() & mask;
    reg::RCC.apb1enr.set_enable(val | bit);
  }
}

impl Copy for BusApb1 {}

#[allow(missing_docs)]
#[repr(u8)]
#[derive(Clone)]
pub enum BusApb2 {
  SysCfg,
  Tim1,
  Spi1,
  Tim8,
  Usart1,
  Spi4,
  Tim15,
  Tim16,
  Tim17,
  Tim20,
}

impl BusApb2 {
  fn to_reg_bit(self) -> u32 {
    use self::BusApb2::*;
    1 << match self {
      SysCfg     => 0,
      Tim1       => 11,
      Spi1       => 12,
      Tim8       => 13,
      Usart1     => 14,
      Spi4       => 15,
      Tim15      => 16,
      Tim16      => 17,
      Tim17      => 18,
      Tim20      => 20,
    }
  }

  fn set_reg(self, enable: bool) {
    let reg_bit = self.to_reg_bit();
    let mask: u32 = !reg_bit;
    let bit: u32 = if enable {reg_bit} else {0};
    let val = reg::RCC.apb2enr.enable() & mask;
    reg::RCC.apb2enr.set_enable(val | bit);
  }
}

impl Copy for BusApb2 {}

/// Configures the state of peripheral clock.
///
/// This enum contains all available clocks from both AHB and APB.
#[allow(missing_docs)]
#[derive(Clone)]
pub enum PeripheralClock {
  Ahb(BusAhb),
  Apb1(BusApb1),
  Apb2(BusApb2),
}

impl Copy for PeripheralClock {}

impl PeripheralClock {
  /// Enables the given peripheral clock.
  pub fn enable(self) {
    self.set_enable(true);
  }

  /// Disables the given peripheral clock.
  pub fn disable(self) {
    self.set_enable(false);
  }

  /// Enables or disables the clock.
  fn set_enable(self, enable: bool) {
    match self {
        Ahb(ahb)  => ahb.set_reg(enable),
        Apb1(apb) => apb.set_reg(enable),
        Apb2(apb) => apb.set_reg(enable),
    }
  }

  /// Returns the clock freqency
  pub fn frequency(self, cc: &ClockConfig) -> u32 {
    match self {
      Ahb(_)  => cc.get_ahb_frequency(),
      Apb1(_) => cc.get_apb1_frequency(),
      Apb2(_) => cc.get_apb2_frequency(),
    }
  }
}

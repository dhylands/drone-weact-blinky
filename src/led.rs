use drone_stm32_map::periph::gpio::pin::{GpioPinPeriph, GpioPinMap};
use drone_cortexm::reg::prelude::*;

pub enum Active {
  Low,
  High,
}

pub trait Led {
  fn on(&self);
  fn off(&self);
}

pub struct GpioLed<PIN>
where
  PIN: GpioPinMap,
{
  pin: GpioPinPeriph<PIN>,
  active: Active,
}

impl<PIN: GpioPinMap> GpioLed<PIN> {
  pub fn init(pin: GpioPinPeriph<PIN>, active: Active) -> Self {
      pin.gpio_moder_moder.write_bits(0b01); // 0b01 - General purpose output mode.
      pin.gpio_otyper_ot.clear_bit(); // 0 = Push-Pull
      pin.gpio_ospeedr_ospeedr.write_bits(0b11); // 0b11 - High Speed

      Self { pin, active }
  }
}

impl<PIN: GpioPinMap> Led for GpioLed<PIN> {
  fn on(&self) {
      match self.active {
          Active::Low => self.pin.gpio_bsrr_br.set_bit(),
          Active::High => self.pin.gpio_bsrr_bs.set_bit(),
      }
  }

  fn off(&self) {
      match self.active {
          Active::Low => self.pin.gpio_bsrr_bs.set_bit(),
          Active::High => self.pin.gpio_bsrr_br.set_bit(),
      }
  }
}

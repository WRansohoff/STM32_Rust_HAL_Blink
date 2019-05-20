#![no_std]
#![no_main]

// Halt when the program panics.
extern crate panic_halt;

// Includes.
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{ entry, exception };
use stm32f30x_hal as hal;
use hal::prelude::*;
use hal::stm32f30x;

// Use a static mutable variable to share state
// between the SysTick interrupt and 'main'.
static mut TOGGLE: u32 = 0;

#[entry]
fn main() -> ! {
  // Set up SysTick interrupt.
  let cmp = cortex_m::Peripherals::take().unwrap();
  let mut syst = cmp.SYST;
  syst.set_clock_source( SystClkSource::Core );
  syst.set_reload( 8_000_000 );
  syst.enable_counter();
  syst.enable_interrupt();

  // Set up GPIO pin E8 (LED #4)
  let p = stm32f30x::Peripherals::take().unwrap();
  let mut rcc = p.RCC.constrain();
  let mut gpioe = p.GPIOE.split( &mut rcc.ahb );
  let mut ld4 = gpioe.pe8.into_push_pull_output( &mut gpioe.moder, &mut gpioe.otyper );

  loop {
    // TODO: Remove unsafe block.
    unsafe {
      if TOGGLE == 0 {
        ld4.set_high();
      } else {
        ld4.set_low();
      }
    }
  }
}

#[exception]
fn SysTick() {
  // TODO: Remove unsafe block.
  unsafe {
    if TOGGLE == 0 {
      TOGGLE = 1;
    } else {
      TOGGLE = 0;
    }
  }
}

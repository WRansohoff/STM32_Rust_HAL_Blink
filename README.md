A simple 'blinking LED' program for the STM32F3 Discovery board which the 'Embedded Rust' ebook targets.

It looks like the ebook is a work-in-progress, since the code samples stop a bit after the 'Hello World' section. But that's okay; what's there is well-written and it provides enough information to get started with the core peripherals.

This project uses the ['cortex-m-quickstart' template](https://github.com/rust-embedded/cortex-m-quickstart) and the [stm32f30x\_hal crate](https://crates.io/crates/stm32f30x-hal)

There are two compiler warnings about deprecated GPIO pin methods. I'm not sure whether I'm calling it wrong or the HAL crate is out of date, but suggestions are welcome.

This is your basic "Hello World" program for the [WeACT STM32F411CEU6](https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0.html)
setup to work with [Drone OS](https://www.drone-os.com/)

It sets up the system clock to run at 96 MHz (a bit lower than the max 100 MHz so that the USB peripheral gets a nice clock)

This build uses a patched version of drone-os v0.11

## WeAct F411 notes
* STM32F411CEU6
* HSI - 16 MHz
* HSE - 25 MHz
* LSE - 32.768 kHz
* 512K Flash
* 128K RAM
* has a LED connected to PC13 (active low)
* has a push button connected to PA0 (active low) (no external pullups)
* USB-C connector

## Other modifications
* `rust-toolchain` set to `nightly-2019-11-06`. This should
be resolved when drone-os v0.12 is released and then it can revert to `nightly`.
* `Drone.toml` changed `gdb-client` to be `arm-none-eabi-gdb`.
* `Drone.toml` changed `uart-endpoint` and `gdb-endpoint` to match the USB names that my Black Magic Probe shows up as under MacOS.
* `Cargo.toml` points `drone-stm32-map` to locally patched `drone-stm32-map` repository.

To build - assuming other setup has been done as per the [Drone Book](https://book.drone-os.com/)
```
git clone https://github.com/dhylands/drone-weact-blinky.git weact-blinky
mkdir drone-os-patched
cd drone-os-patched
git clone --single-branch --branch dhylands-patches https://github.com/dhylands/drone-stm32-map.git
cd ../weact-blinky
just build
```

I have a [PR](https://github.com/drone-os/drone-stm32-map/pull/9) to do some fixups on the RCC_PLLCFGR register.
The [dhylands-patches](https://github.com/dhylands/drone-stm32-map/tree/dhylands-patches) includes this PR
plus a bunch of edits to the `Cargo.toml` files to make them use crates from github rather than locally.

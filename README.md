This is your basic "Hello World" program for the [WeACT STM32F411CEU6](https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0.html)
setup to work with [Drone OS](https://www.drone-os.com/)

It sets up the system clock to run at 96 MHz (a bit lower than the max 100 MHz so that the USB peripheral gets a nice clock)

This example uses drone-os v0.12.1

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

## Modifications
* `Drone.toml` changed `gdb-client` to be `arm-none-eabi-gdb`, since I haven't found gdb-multiarch for MacOS.
* `Drone.toml` changed `serial-endpoint` and `gdb-endpoint` to match the USB names that my Black Magic Probe shows up as under MacOS.

To build - assuming other setup has been done as per the [Drone Book](https://book.drone-os.com/)
```
git clone https://github.com/dhylands/drone-weact-blinky.git weact-blinky
cd weact-blinky
just build
```

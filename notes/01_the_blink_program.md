# Baremetal Blink

## The Clock System

When our MCU first powers on, most peripherals are in a low-power or disabled state to save energy. They're physically present on the chip, but they are not receiving any clock signal. Without a clock, a digital peripheral can't do anything, the flip-flops and state machines inside are frozen.

The Reset and Clock Control (RCC) peripheral is special because it's always powered and always has a clock. Its job is to manage the Clock tree. It distributes the clock signal to other peripherals. We can think of it like the electrial panel in a building where we can turn individual circuits on or off.

When we wriet to `RCC_APB2PCENR` (RCC APB2 Peripheral Clock Enable Register) and set bit 5, We're telling the RCC "Start sending clock pulses to the GPIOD peripheral". Only after this bit is set does the GPIOD peripheral "wake up" and become responsive.

If we tried to write to the GPIOD registers before enabling its clock, one of two things would happen:

1. Our write would be sliently ignored (the peripheral isn't listening).
2. The bus might hang waiting for a response that never comes.

Ultimately, we must:

1. **FIRST**: Enable the peripheral's clock in the RCC.
2. **THEN**: Configure the peripheral's registers.
3. **FINALLY**: Use the peripheral for its intended function.

## The Register Model

Every peripheral is a collection of registers mapped to specific memory addresses. When we write `GPIOD_CFGLR.write_volatile(config)`, we're not calling a function in the traditional sense, we are directly writing a value to a specific location in the processor's memory map that happens to be connected to hardware.

For the GPIOD peripheral on the ch32v003, the peripheral has several registers at consecutive adderesses:

- 0x40011400: CFGLR (Configuration Low Register) - controls pins 0-7
- 0x40011404: Reserved
- 0x40011408: INDR (Input Data Register) - read pin states
- ... and so on.

Each register is 32 bits, and different bits or groups of bits control different aspects of the peripheral.

## GPIOD Configuration

When we wrote to `GPIOD_CFGLR`, we were specifically configuring pin 6. The CFGLR riegister dedicates 4 bits to each pin (that's why its called configuration Low - it only handles pins 0-7, there's CFGHR for pins 8-15 on larger chips).

For pin 6, those 4 bits are [27:24]:

- Bits [25:24] = *MODE* bits: control the speed/direction

  - `00` = Input
  - `01` = Output 10MHz
  - `10` = Output 2MHz
  - `11` = Output 50MHz

- Bits [27:26] = *CNF* bits: control the configuration

  - For outputs: `00` = Push-pull, `01` = Open-drain, `10` = Alternate function push-pull, `11` alternate function open-drain.
  - For inputs: different meanings (floating, pull-up, pull-down, e.t.c)

When we set those 4 bits to `0011` (0x3), we configured:

- MODE = `11`: output at 50MHz (fast switching)
- CNF = `00`: Push-PUll output (can actively drive both high and low)

## The BIGGER Picture

This pattern - enable clock, configure registers, use peripheral - applies to every peripheral you'll work with:

- Timers: Enable clock -> set prescaler and period -> Start counting
- UART: Enable clock -> set baud rate and frame format -> Transmit/receive
- SPI: Enable clock -> set clock polarity and phase -> Transfer Data
- ADC: Enable clock -> set sampling time -> Start conversion
- ... and so on.

**Each time we are just writing values to memory-mapped registers to tell the hard ware what to do. The datahseet is our bible here, it's a reference manual, it tells us which addresses and which bit patterns control what behavior.**

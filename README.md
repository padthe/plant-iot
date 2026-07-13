# Plant IoT

A learning project for building a plant monitoring system using Rust.

## Technology

- Raspberry Pi Pico WH for reading sensors
- Embedded Rust and Embassy for the Pico firmware
- Raspberry Pi 3B/3B+ as the server
- Docker for running the server
- ntfy for watering notifications

## Project structure

```text
plant-iot/
├── firmware/     # Embedded Rust firmware for the Pico
└── README.md
```

## Building the firmware

Run the following commands from the project root:

```bash
cd firmware
cargo build --release
```

## Flashing the Pico

Disconnect the Pico from USB.

Hold down the `BOOTSEL` button and connect the USB cable while continuing to hold the button. Release the button after the Pico is connected.

Verify that the Pico is detected:

```bash
lsusb
```

It should appear as:

```text
Raspberry Pi RP2 Boot
```

Find the device and partition:

```bash
lsblk -f
```

The Pico should have the label `RPI-RP2`. Its partition will usually be `/dev/sda1`, but this may vary.

If Arch Linux does not mount it automatically, mount it manually:

```bash
udisksctl mount -b /dev/sda1
```

Then build and flash the firmware:

```bash
cargo run --release
```

After flashing, the Pico automatically restarts and runs the firmware. The `RPI-RP2` drive will disappear. This is normal.

## Current milestone

The Pico runs an embedded Rust program that blinks an external LED connected to GPIO 16.

## Planned next steps

- Connect the soil moisture sensor
- Read the sensor using the Pico ADC
- Calibrate dry and wet sensor values
- Connect the Pico W to Wi-Fi
- Send measurements to the Raspberry Pi server
- Send watering notifications through ntfy
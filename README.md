# rust_gpiozero_htf

Hacktoberfest rust_gpiozero notes and examples

2023-10-20

This repo contains documentation and an example project to help with `rust_gpiozero` development. Although developing for the library needs access to RaspberryPi hardware, it's a great project for Hacktoberfest because:

1. The project mirrors the Python GPIOZero library, so there's a reference to compare against.
2. Each sensor has its own implementation, so there are plenty of self-contained implementation tasks.
3. Rust! RasperryPis! Robots!

## Useful links:

1. [rust_gpiozero repository](https://github.com/rahul-thakoor/rust_gpiozero.git)
2. [GPIOZero docs](https://gpiozero.readthedocs.io/en/stable/index.html)
3. [GPIOZero repository](https://github.com/gpiozero/gpiozero)
4. [ELEGOO sensor datasheets](https://download.elegoo.com/?t=Upgraded_37_in_1_Sensor_Modules_Kit)
5. [cross](https://github.com/cross-rs/cross)

## Documentation

1. [User instructions](./blob/main/docs/user_instructions.md): how to access a Raspberry PI.
2. [Cross compile](./blob/main/docs/cross_compile.md): how to cross compile Rust for the Raspberry Pi.
3. [Sensors and actuators](./blob/main/docs/sensors.md): info about the input and output devices.
4. [Network configuration](./blob/main/docs/network_configuration.md): reference for how the tunnelling is configured.


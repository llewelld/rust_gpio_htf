# rust_gpiozero_htf

Hacktoberfest rust_gpiozero notes and examples

2023-10-20

This repo contains documentation and an example project to help with `rust_gpiozero` development. Although developing for the library needs access to RaspberryPi hardware, it's a great project for Hacktoberfest because:

1. The project mirrors the Python GPIOZero library, so there's a reference to compare against.
2. Each sensor has its own implementation, so there are plenty of self-contained implementation tasks.
3. Rust! RasperryPis! Robots!

## Quickstart
Instructions to get started with a complete version available in [docs/](docs/).

### Compiling
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the fork of `rust_gpiozero`:
```bash
git clone https://github.com/llewelld/rust_gpiozero.git
```
3. Start [docker](https://www.docker.com/get-started/) on your machine.
4. Install cross:
```bash
cargo install cross --git https://github.com/cross-rs/cross
```
5. Build an example (e.g. blink):
```
cross build --release --example blink --target arm-unknown-linux-gnueabi
```
6. The binary should now be available:
```
ls target/arm-unknown-linux-gnueabi/release/examples/blink
```

### Connecting to a Raspberry PI and running the example
1. Create ssh key and send David your *public* key
2. Open the tunnel to [one of the Raspberry PIs](docs/user_instructions.md) e.g. **fiacre*:
```bash
ssh -N -f -M -S /tmp/file-sock -L 8000:localhost:8001 cecilia@34.249.167.137
```
3. Now you will be able to copy your example binary (`blink`) to the Raspberry PI (fiacre):
```
scp -P 8000 target/arm-unknown-linux-gnueabi/release/examples/blink pi@localhost:./
```
4. Finally ssh into the Raspberry PI and run the example:
```
ssh pi@localhost -p 8000
./blink
```

## Useful links:

1. [rust_gpiozero repository](https://github.com/rahul-thakoor/rust_gpiozero.git)
2. [GPIOZero docs](https://gpiozero.readthedocs.io/en/stable/index.html)
3. [GPIOZero repository](https://github.com/gpiozero/gpiozero)
4. [ELEGOO sensor datasheets](https://download.elegoo.com/?t=Upgraded_37_in_1_Sensor_Modules_Kit)
5. [cross](https://github.com/cross-rs/cross)

## Documentation

1. [User instructions](./docs/user_instructions.md): how to access a Raspberry PI.
2. [Cross compile](./docs/cross_compile.md): how to cross compile Rust for the Raspberry Pi.
3. [Sensors and actuators](./docs/sensors.md): info about the input and output devices.
4. [Network configuration](./docs/network_configuration.md): reference for how the tunnelling is configured.


# Cross Compiling Rust

The version of Rust on the RaspberryPi is old and compilation on the device is slow, so it's preferable to cross compile on your development device (e.g. laptop) using [cross](https://github.com/cross-rs/cross) and then copy the binaries over to a RaspberryPi to test them.

## Setting up rust_gpiozero

First you should fork your own copy of [the repository](https://github.com/rahul-thakoor/rust_gpiozero). This is so you can make changes which you can then contribute back. The steps below use my form of the repository, so you will need to change the URLs to suit your needs.

```
git clone git@github.com:llewelld/rust_gpiozero.git
cd rust_gpiozero
git remote add upstream https://github.com/rahul-thakoor/rust_gpiozero.git
```

By creating a separate `upstream` remote it means you have read access to synchronise your copies with upstream, but still have write access to your fork.

To update your local copy with changes from upstream:

```
git fetch upstream
git rebase upstream/master
```

The push to your own fork

```
git push origin
```

## Setting up cross compilation

You'll need a working `docker` installation. The [following steps](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-22-04) are for a Linux machine.

```
sudo apt install -y apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
  sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture)" \
  "signed-by=/usr/share/keyrings/docker-archive-keyring.gpg]" \
  "https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
apt-cache policy docker-ce
sudo apt install -y docker-ce
sudo usermod -aG docker ${USER}
# Restart the shell
```

Install `cross`:

```
cargo install cross --git https://github.com/cross-rs/cross
```

## Building the library

For a local build:
```
cargo build
```
This will produce a binary which can be executed locally but won't run on the RaspberryPi:
```
./target/debug/rust-server
```

To build for a 32-bit Raspberry Pi:
```
cross build --release --target arm-unknown-linux-gnueabi
```

To build for a 64-bit Raspberry Pi:

```
cross build --release --target aarch64-unknown-linux-gnu
```

## Transferring the result to the Pi

On your development machine:
```
scp -P 8000 ./target/arm-unknown-linux-gnueabi/release/gpiozero-test pi@localhost:~
```

## Execute on the RaspberryPi

Ensure you have the appropriate hardware input/output sensors attached.
```
~/gpiozero-test
```




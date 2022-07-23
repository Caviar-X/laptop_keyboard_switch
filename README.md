# Laptop keyboard switch

a laptop keyboard switch on linux based on xinput

## Usage

install `xinput` on your linux distro

Manjaro :
```bash
sudo pacman -S xinput
```

execute `cargo build --release` to build the project

find the binary in `./target/release/`

you can just use `./laptop_keyboard_switcher` to input the operation or you can use `--enable`,`--disable` to control.

A toy project,maybe won't commit from now on.


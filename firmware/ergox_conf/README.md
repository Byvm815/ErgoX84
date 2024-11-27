# RMK

Keyboard firmware for cortex-m, with layer/dynamic keymap/[vial](https://get.vial.today) support, written in Rust.

## Checklist after generating the firmware project

- [ ] Update `memory.x` for your microcontroller(if needed)

- [ ] Update your `keyboard.toml`

- [ ] Create your `vial.json` by your layout: https://get.vial.today/docs/porting-to-via.html, replace the default one

- [ ] (Optional) Check the chip name of `probe-rs` is right, if you don't use `cargo run`, you can skip this step

- [ ] Update the family ID of your microcontroller in `Makefile.toml`, if you want to generate .uf2 firmware. The available family ID can be found in `scripts/uf2conv.py`

##Build && convert format

`rustup target add thumbv6m-none-eabi`

`cargo build --release --target=thumbv6m-none-eabi`

`elf2uf2-rs target/thumbv6m-none-eabi/release/peripheral target/thumbv6m-none-eabi/release/peripheral.uf2`

`elf2uf2-rs target/thumbv6m-none-eabi/release/central target/thumbv6m-none-eabi/release/central.uf2`
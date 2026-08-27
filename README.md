# steam-controller-bootloader

Steam Controller LPC11U37F boot block (Valve HID programming mode), forked from
[roblabla’s reverse‑engineered bootloader](https://github.com/roblabla/steam_controller_custom_firmware) but with just the bootloader as a standalone crate.

Changes from upstream (in order):

* Ported to a **stable** toolchain and **cortex-m 0.7** — that alone pushed the
  release image **over the 8 KiB** boot slot on a modern rustc
* Fixed PLL wait polarity (`is_pll_not_locked`)
* Dropped `lpc11uxx-hal` (panic LED uses a busy-wait) to save space
* Dropped trampolines thinking we could use VTOR to save space, but restored
  to be compatible with stock valve firmware (per-vector style: most IRQs
  always forward to `0x2000`; only PendSV/CT32B1/USART/USB mux on GPREG1;
  HardFault forwards to the app)
* Dropped led intensity table to save space instead

## Dependencies

To build this bootloader you'll need:

- A recent **stable** Rust toolchain (see `rust-toolchain.toml`) with the
  Cortex-M0 target:

``` console
$ rustup target add thumbv6m-none-eabi
```

- [`cargo-binutils`](https://github.com/rust-embedded/cargo-binutils) (for
  `cargo objcopy`) and the `llvm-tools` rustup component:

``` console
$ rustup component add llvm-tools
$ cargo install cargo-binutils
```

- [`lpc_checksum`](https://crates.io/crates/lpc_checksum) to patch the LPC
  vector-table checksum into the `.bin`:

``` console
$ cargo install lpc_checksum
```

- Optional: [cargo-make](https://github.com/sagiegurari/cargo-make) — plain
  `cargo make` runs release objcopy, checksum, and the version stamp at
  offset `0x24` (debug does not fit the 8 KiB slot).

- [`lpc11uxx`](https://github.com/lpc-rs/lpc-pac) from git (not crates.io
  0.3.0) so overlapping USART registers are available for `nrf_comms`. The
  vendored `lpc11uxx-rom` crate in this repo is used as-is (NXP ROM USB + IAP).

## Build

```bash
cargo build --release
cargo objcopy --release --bin=steam-controller-bootloader -- -O binary bootloader-release.bin
lpc_checksum -p LPC11U37_501 bootloader-release.bin
```

Or with cargo-make:

``` console
$ cargo make
```

## Flash

Hold **Right Trigger** and plug in USB to get the `CRP DISABLD` drive, then either:

**Bootloader only** (leaves any existing app in place):

```bash
sudo dd conv=nocreat,notrunc oflag=direct bs=512 if=bootloader-release.bin of="/media/$USER/CRP DISABLD/firmware.bin"
sync
```

**Bootloader + wipe app** (zeros 4 KiB after the boot block so the app token at `0x2024` is cleared):

```bash
cat bootloader-release.bin <(dd if=/dev/zero bs=4096 count=1 status=none) | sudo dd conv=nocreat,notrunc oflag=direct bs=512 of="/media/$USER/CRP DISABLD/firmware.bin"
sync
```

Unplug/replug normally.

# License

This crate is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

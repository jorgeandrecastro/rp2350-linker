[![Crates.io](https://img.shields.io/crates/v/rp2350-linker.svg)](https://crates.io/crates/rp2350-linker)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL_2.0_or_later-blue.svg)](https://opensource.org/licenses/GPL-2.0)
[![docs.rs](https://docs.rs/rp2350-linker/badge.svg)](https://docs.rs/rp2350-linker)

# 🦅 rp2350-linker

Zero-config linker and boot support for the RP2350 (Raspberry Pi Pico 2) in Rust.

Stop fighting with `memory.x` and custom linker scripts. `rp2350-linker` automatically injects the correct memory layout and boot metadata so your `no_std` Rust applications can run cleanly on the RP2350.

> [!IMPORTANT]
> Version `0.3` was an experimental attempt to add RISC-V (Hazard3) support. Due to lack of stabilization in upstream runtimes (`riscv-rt`) for the RP2350, RISC-V support has been completely dropped.
>
> Please avoid version `0.3`. Use version `0.4` (or earlier stable versions like `0.1` and `0.2`), which focus exclusively on a robust, rock-solid ARM Cortex-M33 experience. RISC-V support will return when the ecosystem stabilizes.

## ✨ Key features

- Zero-setup memory layout injection at compile time
- Dedicated ARM Cortex-M33 support
- Boot-ready support for the required `.start_block` and `.boot_info` sections
- `picotool` compatibility via `.bi_entries` mapping
- Seamless integration with `cortex-m-rt` and `embassy`
- Optimized defaults for 4 MB flash and 512 KB RAM

## 📦 Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
rp2350-linker = "0.4"
```

## 🛠️ Usage

Import the crate in your application entry point. The build script automatically injects the required linker rules for ARM Cortex-M33.

```rust
#![no_std]
#![no_main]

use rp2350_linker as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
```

## ⚙️ Recommended `.cargo/config.toml` for ARM

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs -d"

rustflags = [
    "-C", "linker=flip-link",
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=--nmagic",
]

[build]
target = "thumbv8m.main-none-eabihf"
```

## 🚀 Flashing workflow

To automate build, UF2 conversion, and flashing, create a `flash.sh` script at the root of your project:

```bash
#!/bin/bash
set -e

export PATH="$HOME/.cargo/bin:$PATH"

TARGET="thumbv8m.main-none-eabihf"
FAMILY="rp2350-arm-s"

echo "🦅 Target: ARM Cortex-M33"

CRATE_NAME=$(cargo metadata --no-deps --format-version 1 | python3 -c 'import sys, json; print(json.load(sys.stdin)["packages"][0]["name"])')
TARGET_PATH="target/$TARGET/release/$CRATE_NAME"
UF2_NAME="${CRATE_NAME}.uf2"

echo "🦀 Step 1: Compiling release for $TARGET..."
cargo build --release --target "$TARGET" || {
    echo "❌ Compilation failed"
    exit 1
}

echo "📦 Step 2: Converting ELF to UF2 ($FAMILY)..."
picotool uf2 convert -t elf "$TARGET_PATH" "$UF2_NAME" --family "$FAMILY"

echo "⚡ Step 3: Flashing to device..."
sudo picotool load "$UF2_NAME" -x

echo "✅ Done! ${CRATE_NAME} is flying on ${FAMILY}! 🚀"
```

Then make it executable and run it:

```bash
chmod +x flash.sh
./flash.sh
```

> [!NOTE]
> This workflow requires `picotool` to be installed and available in your `PATH`.

## 🛡️ License

This project is licensed under the GPL-2.0-or-later.

## 🦅 About

Developed by Jorge Andre Castro.
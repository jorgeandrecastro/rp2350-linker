# 🦅 rp2350-linker

Zero-config linker and boot support for the RP2350 (Raspberry Pi Pico 2) in Rust.

[![Crates.io](https://img.shields.io/crates/v/rp2350-linker.svg)](https://crates.io/crates/rp2350-linker)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL_2.0_or_later-blue.svg)](https://opensource.org/licenses/GPL-2.0)

Stop fighting with `memory.x` and custom linker scripts. `rp2350-linker` provides an automated, transparent way to boot your `no_std` Rust applications on the **RP2350** (Pico 2) with correct memory mapping and boot blocks.

---

## 🚀 Key Features

* **Zero-Setup:** Automatically injects the correct `memory.x` for the RP2350.
* **Boot-Ready:** Includes the required `.start_block` and `.boot_info` for the RP2350 BootROM.
* **Picotool Compatible:** Maps `.bi_entries` so `picotool` can identify your binary.
* **Invisible Overhead:** Uses `INSERT AFTER` to work seamlessly with `cortex-m-rt` and `embassy`.
* **Optimized:** Pre-configured for 4MB Flash and 512K RAM.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rp2350-linker = "0.2.1"
🛠 Usage
In your main.rs, simply import the crate to activate the linker automation:

Rust
#![no_std]
#![no_main]

// 🦅 The Magic Line: This handles memory layout and boot blocks
use rp2350_linker as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Your elite RP2350 code here...
    loop {}
}
Recommended .cargo/config.toml
To take full advantage of the Pico 2, use these flags:

Ini, TOML
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs -d"

rustflags = [
  "-C", "linker=flip-link",
  "-C", "link-arg=-Tlink.x", # This triggers the cortex-m-rt link process
  "-C", "link-arg=--nmagic",
]

[build]
target = "thumbv8m.main-none-eabihf"

🚀 Pro Tips: Workflow Automation
For those who prefer a manual control or have permission issues with direct runners, you can use this flash.sh script. It automates the compilation, conversion to UF2 (specific to RP2350 ARM-S), and flashing using picotool.

📄 flash.sh
Create a file named flash.sh in your project root:

Bash
#!/bin/bash
# Ensure Cargo is in the PATH (especially for sudo environments)
export PATH="$HOME/.cargo/bin:$PATH"

echo "🦀 Step 1: Compiling in release mode..."
cargo build --release || { echo "❌ Compilation failed"; exit 1; }


BINARY_NAME="yourse"
TARGET_PATH="target/thumbv8m.main-none-eabihf/release/$BINARY_NAME"

echo "📦 Step 2: Converting ELF to UF2 for RP2350..."
picotool uf2 convert -t elf "$TARGET_PATH" nameofyourwantedfile.uf2 --family rp2350-arm-s

echo "⚡ Step 3: Flashing to device (requires sudo for USB access)..."
sudo picotool load nameofyourwantedfile.uf2 -x

echo "✅ Done!  is flying! 🦅"
🛠️ How to use it:
Make the script executable: chmod +x flash.sh

Run it: ./flash.sh

Note: This script uses picotool. Make sure you have it installed on your system. The --family rp2350-arm-s flag is mandatory for the new Raspberry Pi Pico 2 architecture.


🛡 License
This project is licensed under the GPL-2.0-or-later.
Protecting open-source infrastructure for the community.

🦅 About
Developed by Jorge Andre Castro.
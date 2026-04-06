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
rp2350-linker = "0.1.0"
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
🛡 License
This project is licensed under the GPL-2.0-or-later.
Protecting open-source infrastructure for the community.

🦅 About
Developed by Jorge Andre Castro.
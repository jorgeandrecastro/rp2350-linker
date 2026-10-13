use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Détection automatique de l'architecture via la variable d'environnement TARGET de Cargo
    let target = env::var("TARGET").unwrap_or_default();
    let is_riscv = target.contains("riscv32");

    // Adaptation automatique du point d'insertion du linker
    let insert_section = if is_riscv {
        ".text.init"
    } else {
        ".vector_table"
    };

    let memory_x_content = format!(
        r#"
MEMORY
{{
    FLASH : ORIGIN = 0x10000000, LENGTH = 4096K
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}}

SECTIONS {{
    .start_block : ALIGN(4)
    {{
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    }} > FLASH

    .bi_entries : ALIGN(4)
    {{
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    }} > FLASH
}} INSERT AFTER {insert_section};

_stext = ADDR(.start_block) + SIZEOF(.start_block);
"#,
        insert_section = insert_section
    );

    fs::write(out.join("memory.x"), memory_x_content).unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");
}
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // On déploie les fichiers de config dans le dossier de build
    fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
    fs::write(out.join("link-rp2350.x"), include_bytes!("link-rp2350.x")).unwrap();

    // On dit au linker de chercher ici
    println!("cargo:rustc-link-search={}", out.display());

    // On force l'utilisation du script spécifique RP2350
    println!("cargo:rustc-link-arg=-Tlink-rp2350.x");

    // On surveille les changements pour re-build si besoin
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link-rp2350.x");
}
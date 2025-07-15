fn main() {
    // Tell bootloader where the linker script lives
    println!("cargo:rerun-if-changed=linker.ld");
}

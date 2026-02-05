fn main() {
    // Simple build - no C++ linking needed!
    // We use subprocess IPC instead of FFI
    println!("cargo:rerun-if-changed=src/native.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
}

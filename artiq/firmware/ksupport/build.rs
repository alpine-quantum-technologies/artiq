fn main() {
    build_misoc::cfg();
    println!("cargo:rustc-cfg={}", "ksupport");

    build_ksupport::generate_peripherals_code();
}

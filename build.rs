include!(".meta/gather_build_info.rs");

fn main() {
    // Make sure to call this somewhere inside the buildscript
    gather_build_info();
}
use accel_derive::kernel;

#[kernel]
#[dependencies("accel-core" = { git = "https://gitlab.com/termoshtt/accel" })]
unsafe fn git() {}

fn main() {}

#[cfg(feature = "headers")]
fn main() -> ::std::io::Result<()> {
    ::zmanim_core::generate_headers()
}

#[cfg(not(feature = "headers"))]
fn main() {
    panic!("Headers feature is not enabled");
}

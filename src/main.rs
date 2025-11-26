fn main() {
    if let Err(e) = rust_start::logger::init(tracing::Level::DEBUG) {
        eprintln!("{e:?}");
    }
}

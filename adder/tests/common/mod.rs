// common.rs moved inside common/mod.rs so we don't see output 'common' from `cargo test`
// Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

pub fn setup() {
    // setup code specific to your library's tests would go here
}
use std::error::Error;

use shared_library_builder::build_standalone;

use libboxer_library::latest_libboxer;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libboxer())))
}

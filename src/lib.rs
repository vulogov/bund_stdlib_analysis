use easy_error::{Error};

use bundcore::bundcore::Bund;

pub fn init_lib(vm: &mut Bund) -> Result<&Bund, Error> {
    Ok(vm)
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}

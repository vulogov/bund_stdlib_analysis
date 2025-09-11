use easy_error::{Error};

use bundcore::bundcore::Bund;

use crate::bund_interface::*;

pub mod bund_interface;
pub mod anomalies;
pub mod breakout;


pub fn init_lib(vm: &mut Bund) -> Result<&Bund, Error> {
    let _ = vm.vm.register_inline("analysis.anomalies".to_string(), analysis_anomalies);
    Ok(vm)
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}

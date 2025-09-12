use easy_error::{Error};

use bundcore::bundcore::Bund;

use crate::bund_interface::*;

pub mod bund_interface;
pub mod anomalies;
pub mod breakout;


pub fn init_lib(vm: &mut Bund) -> Result<&Bund, Error> {
    let _ = vm.vm.register_inline("analysis.anomalies".to_string(), analysis_anomalies);
    let _ = vm.vm.register_inline("analysis.outliers".to_string(), analysis_outliers);
    let _ = vm.vm.register_alias("analysis.outliers.mad".to_string(), "analysis.outliers".to_string());
    let _ = vm.vm.register_inline("analysis.outliers.dbscan".to_string(), analysis_outliers_dbscan);
    Ok(vm)
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}

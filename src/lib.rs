use easy_error::{Error};

use bundcore::bundcore::Bund;

use crate::bund_interface::*;
use crate::forecast::{markov};

pub mod bund_interface;
pub mod anomalies;
pub mod breakout;
pub mod forecast;
pub mod estimate;


pub fn init_lib(vm: &mut Bund) -> Result<&Bund, Error> {
    let _ = vm.vm.register_inline("analysis.anomalies".to_string(), analysis_anomalies);
    let _ = vm.vm.register_inline("analysis.breakouts".to_string(), analysis_breakouts);
    let _ = vm.vm.register_inline("analysis.outliers".to_string(), analysis_outliers);
    let _ = vm.vm.register_alias("analysis.outliers.mad".to_string(), "analysis.outliers".to_string());
    let _ = vm.vm.register_inline("analysis.outliers.dbscan".to_string(), analysis_outliers_dbscan);
    // FORECAST
    let _ = vm.vm.register_inline("analysis.forecast.markov".to_string(), markov::stdlib_forecast_stack_consume_markov);
    let _ = vm.vm.register_inline("analysis.forecast.markov.".to_string(), markov::stdlib_forecast_wb_consume_markov);
    let _ = vm.vm.register_inline("analysis.forecast.markov,".to_string(), markov::stdlib_forecast_stack_keep_markov);
    let _ = vm.vm.register_inline("analysis.forecast.markov.,".to_string(), markov::stdlib_forecast_wb_keep_markov);
    // ESTIMATE
    let _ = vm.vm.register_inline("analysis.estimate".to_string(), estimate::stdlib_forecast_estimate_stack);
    let _ = vm.vm.register_inline("analysis.estimate.".to_string(), estimate::stdlib_forecast_estimate_wb);
    let _ = vm.vm.register_inline("analysis.estimate.uncertainty".to_string(), estimate::stdlib_forecast_estimate_uncertainty_stack);
    let _ = vm.vm.register_inline("analysis.estimate.uncertainty.".to_string(), estimate::stdlib_forecast_estimate_uncertainty_wb);
    let _ = vm.vm.register_inline("analysis.estimate!".to_string(), estimate::stdlib_forecast_estimate_analysis_stack);
    let _ = vm.vm.register_inline("analysis.estimate!.".to_string(), estimate::stdlib_forecast_estimate_analysis_wb);
    Ok(vm)
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}

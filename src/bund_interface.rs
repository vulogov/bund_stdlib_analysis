extern crate log;
use easy_error::{bail, Error};

use bundcore::{common, common_get_data};
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};

use crate::anomalies;

pub fn analysis_anomalies(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline ANALYSIS.ANOMALIES");
    }

    let period_value = match vm.stack.pull() {
        Some(period_value) => period_value,
        None => bail!("ANOMALIES: error getting period"),
    };

    let period = match period_value.cast_int() {
        Ok(period) => period,
        Err(err) => {
            bail!("analysis.anomalies returned for #1: {}", err);
        }
    };

    let data = match common_get_data::get_data(vm, StackOps::FromStack, common::SourceMode::Consume, "ANALYSIS.ANOMALIES".to_string()) {
        Ok(data) => data,
        Err(err) => {
            bail!("ANOMALIES: error getting data: {}", err);
        }
    };

    if data.len() == 0 {
        bail!("ANOMALIES: NO DATA for analysis");
    }

    let anomalies_data = match anomalies::detect_anomalies(data, period as usize) {
        Ok(anomalies_data) => anomalies_data,
        Err(err) => bail!("{}", err),
    };

    let mut res = Value::list();
    for n in anomalies_data.iter() {
        res = res.push(Value::from_float(*n as f64));
    }
    vm.stack.push(res);
    Ok(vm)
}

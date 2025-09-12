extern crate log;
use easy_error::{bail, Error};

use bundcore::{common, common_get_data};
use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM, StackOps};

use crate::{anomalies, breakout};

#[derive(Debug, Clone)]
pub enum OutliersMode {
    MAD,
    DBSCAN,
}

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

pub fn analysis_outliers_generic(vm: &mut VM, op: OutliersMode) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for inline ANALYSIS.OUTLIERS");
    }

    let sensitivity_value = match vm.stack.pull() {
        Some(period_value) => period_value,
        None => bail!("OUTLIERS: error getting sensitivity"),
    };

    let sensitivity = match sensitivity_value.cast_float() {
        Ok(period) => period,
        Err(err) => {
            bail!("analysis.outliers returned for #1: {}", err);
        }
    };

    let data1 = match common_get_data::get_data(vm, StackOps::FromStack, common::SourceMode::Consume, "ANALYSIS.OUTLIERS".to_string()) {
        Ok(data) => data,
        Err(err) => {
            bail!("OUTLIERS: error getting data #1: {}", err);
        }
    };

    if data1.len() == 0 {
        bail!("OUTLIERS: NO DATA #1 for analysis");
    }

    let data2 = match common_get_data::get_data(vm, StackOps::FromStack, common::SourceMode::Consume, "ANALYSIS.OUTLIERS".to_string()) {
        Ok(data) => data,
        Err(err) => {
            bail!("OUTLIERS: error getting data #2: {}", err);
        }
    };

    if data2.len() == 0 {
        bail!("OUTLIERS: NO DATA #2 for analysis");
    }

    match op {
        OutliersMode::MAD => {
            let outliers_data = match anomalies::detect_outliers(data1, data2, sensitivity) {
                Ok(outliers_data) => outliers_data,
                Err(err) => bail!("{}", err),
            };
            vm.stack.push(outliers_data);
        }
        OutliersMode::DBSCAN => {
            let outliers_data = match anomalies::detect_outliers_dbscan(data1, data2, sensitivity) {
                Ok(outliers_data) => outliers_data,
                Err(err) => bail!("{}", err),
            };
            vm.stack.push(outliers_data);
        }
    }
    Ok(vm)
}

pub fn analysis_outliers(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    analysis_outliers_generic(vm, OutliersMode::MAD)
}

pub fn analysis_outliers_dbscan(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    analysis_outliers_generic(vm, OutliersMode::DBSCAN)
}

pub fn analysis_breakouts(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline ANALYSIS.BREAKOUTS");
    }

    let ms_value = match vm.stack.pull() {
        Some(ms_value) => ms_value,
        None => bail!("ANOMALIES: error getting min-size"),
    };

    let ms = match ms_value.cast_int() {
        Ok(ms) => ms,
        Err(err) => {
            bail!("analysis.breakouts returned for #1: {}", err);
        }
    };

    let data = match common_get_data::get_data(vm, StackOps::FromStack, common::SourceMode::Consume, "ANALYSIS.BREAKOUTS".to_string()) {
        Ok(data) => data,
        Err(err) => {
            bail!("ANOMALIES: error getting data: {}", err);
        }
    };

    if data.len() == 0 {
        bail!("ANOMALIES: NO DATA for analysis");
    }

    let breakouts_data = match breakout::detect_breakouts(data, ms as usize) {
        Ok(breakouts_data) => breakouts_data,
        Err(err) => bail!("{}", err),
    };
    let mut res = Value::list();
    for n in breakouts_data.iter() {
        res = res.push(Value::from_float(*n as f64));
    }
    vm.stack.push(res);
    Ok(vm)
}

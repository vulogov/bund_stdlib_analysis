extern crate log;
use bundcore::{common, common_get_data};
use rust_multistackvm::multistackvm::{VM, StackOps};
use rust_dynamic::value::Value;
use easy_error::{Error, bail};
use decorum::{R64};
use markov_chain::Chain;
use rstats::*;

fn forecast_markov_base(vm: &mut VM, op: StackOps, smode: common::SourceMode, err_prefix: String) -> Result<&mut VM, Error> {
    match common_get_data::get_data(vm, op.clone(), smode, err_prefix.clone()) {
        Ok(source) => {
            let mut dst: Vec<R64> = Vec::new();
            for v in source {
                dst.push(v.into());
            }
            let mut palanteer = Chain::<R64>::new(16);
            palanteer.train(dst);
            let res = palanteer.generate_limit(16);
            if res.len() == 0 {
                bail!("{} forecasting does not returned a prognisis", &err_prefix);
            }
            let res_amean = match res.amean() {
                Ok(res_amean) => res_amean,
                Err(err) => {
                    bail!("{} forecasting can not compute mean: {}", &err_prefix, err);
                }
            };
            let _ = match op {
                StackOps::FromStack => vm.stack.push(Value::from_float(res_amean)),
                StackOps::FromWorkBench => vm.stack.push_to_workbench(Value::from_float(res_amean)),
            };
        }
        Err(err) => {
            bail!("{} returned: {}", &err_prefix, err);
        }
    }
    Ok(vm)
}

#[time_graph::instrument]
pub fn stdlib_forecast_stack_consume_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromStack, common::SourceMode::Consume, "FORECAST.MARKOV".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_wb_consume_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromWorkBench, common::SourceMode::Consume, "FORECAST.MARKOV.".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_stack_keep_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromStack, common::SourceMode::Keep, "FORECAST.MARKOV,".to_string())
}

#[time_graph::instrument]
pub fn stdlib_forecast_wb_keep_markov(vm: &mut VM) -> Result<&mut VM, Error> {
    forecast_markov_base(vm, StackOps::FromWorkBench, common::SourceMode::Keep, "FORECAST.MARKOV.,".to_string())
}

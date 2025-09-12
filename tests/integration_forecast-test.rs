#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_analysis::*;
    use bundcore::bundcore::Bund;

    #[test]
    fn test_integration_forecast_markov1() {
        let script = r#"
        [1.0 2.0 1.0 2.0 1.0 2.0 1.0 2.0 1.0 2.0
         1.0 2.0 1.0 2.0 1.0 2.0 1.0 2.0 1.0 2.0
         1.0 2.0]
            analysis.forecast.markov
        "#;
        let mut bund = Bund::new();
        let _ = init_lib(&mut bund);
        match bund.eval(script) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", &err);
            }
        }
        let res = bund.vm.stack.pull().unwrap();
        println!("{}", &res);
        assert_ne!(res.cast_float().unwrap(), 2.0 as f64);
    }


}

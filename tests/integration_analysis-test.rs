#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_analysis::*;
    use bundcore::bundcore::Bund;

    #[test]
    fn test_integration_analysis_estimate1() {
        let script = r#"
        4.0 // Will set a shape of the sample (between 2.0 and 6.0)
        [5.0 9.0 2.0 9.0 0.0 6.0 3.0 8.0 5.0 18.0
        7.0 8.0 8.0 0.0 2.0 15.0 0.0 5.0 6.0 7.0
        3.0 6.0 1.0 4.0 4.0 4.0 30.0 7.0 5.0 8.0]
            analysis.estimate
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
        assert_ne!(res.cast_float().unwrap(), 8.0 as f64);
    }

    #[test]
    fn test_integration_analysis_estimate2() {
        let script = r#"
        4.0 // Will set a shape of the sample (between 2.0 and 6.0)
        [5.0 9.0 2.0 9.0 0.0 6.0 3.0 8.0 5.0 18.0
        7.0 8.0 8.0 0.0 2.0 15.0 0.0 5.0 6.0 7.0
        3.0 6.0 1.0 4.0 4.0 4.0 30.0 7.0 5.0 8.0]
            analysis.estimate!
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
        //assert_ne!(res.cast_float().unwrap(), 8.0 as f64);
    }

    #[test]
    fn test_integration_analysis_estimate3() {
        let script = r#"
        4.0 // Will set a shape of the sample (between 2.0 and 6.0)
        [1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0
         9.0 10.0 11.0 12.0 13.0 14.0 15.0
         1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0
          9.0 10.0 11.0 12.0 13.0 14.0 15.0
        ]
            analysis.estimate!
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
        //assert_ne!(res.cast_float().unwrap(), 8.0 as f64);
    }

}

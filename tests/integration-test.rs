#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_analysis::*;
    use bundcore::bundcore::Bund;

    #[test]
    fn test_integration_anomalies() {
        let script = r#"
        [5.0 9.0 2.0 9.0 0.0 6.0 3.0 8.0 5.0 18.0
        7.0 8.0 8.0 0.0 2.0 15.0 0.0 5.0 6.0 7.0
        3.0 6.0 1.0 4.0 4.0 4.0 30.0 7.0 5.0 8.0]
            7 analysis.anomalies
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
        assert_eq!(res.len(), 3 as usize);
    }

}

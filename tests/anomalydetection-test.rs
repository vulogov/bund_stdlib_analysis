#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use bund_stdlib_analysis::*;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_anomaly_detection1() {
        let series = vec![
            5.0, 9.0, 2.0, 9.0, 0.0, 6.0, 3.0, 8.0, 5.0, 18.0,
            7.0, 8.0, 8.0, 0.0, 2.0, 15.0, 0.0, 5.0, 6.0, 7.0,
            3.0, 6.0, 1.0, 4.0, 4.0, 4.0, 30.0, 7.0, 5.0, 8.0
        ];
        let res = anomalies::detect_anomalies(series, 7).unwrap();
        println!("{:?}", res);
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_detect_outliers1() {
        let data1 = vec![
            5.0, 9.0, 2.0, 9.0, 0.0, 6.0, 3.0, 8.0, 5.0, 18.0,
            7.0, 8.0, 8.0, 0.0, 2.0, 15.0, 0.0, 5.0, 6.0, 7.0,
            3.0, 6.0, 1.0, 4.0, 4.0, 4.0, 30.0, 7.0, 5.0, 8.0
        ];
        let data2 = vec![
            1.0, 1.0, 10.0, 9.0, 9.0, 6.0, 3.0, 8.0, 5.0, 18.0,
            7.0, 8.0, 8.0, 0.0, 2.0, 15.0, 0.0, 5.0, 6.0, 7.0,
            3.0, 6.0, 1.0, 4.0, 4.0, 4.0, 30.0, 7.0, 5.0, 8.0
        ];

        let res = anomalies::detect_outliers(data1, data2, 0.99).unwrap();
        println!("{}", res);
    }

    #[test]
    fn test_detect_breakouts1() {
        let data = vec![
            3.0, 1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0, 2.0, 3.0,
            6.0, 4.0, 4.0, 5.0, 6.0, 4.0, 4.0, 4.0, 6.0, 5.0,
            9.0, 8.0, 7.0, 9.0, 8.0, 9.0, 9.0, 9.0, 7.0, 9.0
]       ;

        let res = breakout::detect_breakouts(data, 5).unwrap();
        println!("{:?}", res);
        assert_eq!(res.len(), 1);
    }
}

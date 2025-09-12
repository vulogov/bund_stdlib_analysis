# Module for BUND standard library: Analytical tools for telemetry and not only for telemetry

This module includes BUND functions developed for analysis of the numeric and textual data and data samples.

## Installation

This module required _make_ and _Rust_ framework to be installed first. After that:

```bash
cargo add bund_stdlib_analysis
```

## Quick start

Get started with a simple example to analyze anomalies in the sample of numeric data

```rust
[
  5.0 9.0 2.0 9.0 0.0 6.0 3.0 8.0 5.0 18.0
  7.0 8.0 8.0 0.0 2.0 15.0 0.0 5.0 6.0 7.0
  3.0 6.0 1.0 4.0 4.0 4.0 30.0 7.0 5.0 8.0
]
  7
    analysis.anomalies
```

The following call will return a LIST value:

```json
[ 18.0 ::  15.0 ::  30.0 :: ]
```
## BUND functions exposed in this module

| Name | Stack IN | Stack OUT | Description |
|------|----------|-----------|-------------|
| analysis.anomalies | `Data`<br/>`period` | `List of anomalies`<br/> | Search for anomalies in the data sample |
| analysis.breakouts | `Data`<br/>`minimum size` | `List of breakout points`<br/> | Search for breakouts in the data sample |
| analysis.outliers.mad | `Data`<br/>`Data`<br/>`sensitivity` | `Dict with information about outliers`<br/> | Search for outliers in two samples using MAD algorithm |
| analysis.outliers.dbscan | `Data`<br/>`Data`<br/>`sensitivity` | `Dict with information about outliers`<br/> | Search for outliers in two samples using DBSCAN algorithm |

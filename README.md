# VictoriaMetrics2OpenMetrics

This is a small CLI filter to convert a `VictoraMetrics` JSON-lines export into
`OpenMetrics` exposition format.

# status

[JSON-lines](https://jsonlines.org) exported metrics from
[VictoriaMetrics](https://victoriametrics.com/) can be converted into
[OpenMetrics](https://openmetrics.io/) expositions:

```(shell)
cargo run --release < metrics.json > metrics.open
```

Then [backfilling via promtool](https://medium.com/tlvince/prometheus-backfilling-a92573eb712c) can
take place.

Each TYPE emitted will be added to a HashSet, so very complicated exports may consume large amounts of RAM, but for most use cases everything will stream and peak memory usage should be trivial in nature.

Only Counter exports have been worked with so far, so the Counter TYPE is hardcoded.

## Code of conduct

Please note that this project is released with a Contributor Code of Conduct. By
participating in this project you agree to abide by its terms.

## Contributing

PR's on Github as normal please. Cargo check, rustfmt code before submitting.
There is no test suite yet, so please make sure you can successfully backfill
metrics into Prometheus after whatever change you've made.

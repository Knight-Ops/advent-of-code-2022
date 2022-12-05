# Raspberry Pi 4 (4 GB) Benchmarks

## Note : mutability dilutes the benchmark by including parsing in all tests, parsing is benchmarked by itself and the difference can be taken to find each solution's individual benchmark

```
day5 input parser       time:   [218.86 µs 218.89 µs 218.91 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

day5 part 1 + parse     time:   [474.03 µs 474.06 µs 474.10 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

day5 part 2 + parse     time:   [459.82 µs 459.90 µs 460.00 µs]
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe
```
# Raspberry Pi 4 (4 GB) Benchmarks

## Due to mutability today, all benchmarks include parsing in the measurement, but parsing is also done by itself for a baseline, the difference between the two is the time for each solution

```
day11 input parser      time:   [18.422 µs 18.428 µs 18.434 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild

day11 part 1 + parse    time:   [68.621 µs 68.648 µs 68.675 µs]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  5 (5.00%) high mild

day11 part 2 + parse    time:   [24.360 ms 24.379 ms 24.399 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
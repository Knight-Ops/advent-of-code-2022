# Raspberry Pi 4 (4 GB) Benchmarks

## Due to mutability today, all benchmarks include parsing in the measurement, but parsing is also done by itself for a baseline, the difference between the two is the time for each solution

```
day11 input parser      time:   [18.522 µs 18.527 µs 18.534 µs]
                        change: [+0.5640% +0.6175% +0.6700%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

day11 part 1 + parse    time:   [32.324 µs 32.342 µs 32.362 µs]
                        change: [-52.827% -52.760% -52.662%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

day11 part 2 + parse    time:   [5.6882 ms 5.6929 ms 5.6976 ms]
                        change: [-76.676% -76.649% -76.622%] (p = 0.00 < 0.05)
                        Performance has improved.
```
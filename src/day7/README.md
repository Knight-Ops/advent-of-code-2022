# Raspberry Pi 4 (4 GB) Benchmarks

Input parser probably needs some work here as it's doing *most* of the required work.

```
day7 input parser       time:   [1.2033 ms 1.2035 ms 1.2037 ms]
                        change: [+1.7095% +1.8355% +2.0785%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

day7 part 1             time:   [1.1311 µs 1.1324 µs 1.1337 µs]
                        change: [-4.3992% -4.2397% -4.0775%] (p = 0.00 < 0.05)
                        Performance has improved.

day7 part 2             time:   [1.2637 µs 1.2654 µs 1.2671 µs]
                        change: [-0.0404% +0.1137% +0.2701%] (p = 0.16 > 0.05)
```
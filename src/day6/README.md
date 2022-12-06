# Raspberry Pi 4 (4 GB) Benchmarks

```
day6 input parser       time:   [15.507 µs 15.509 µs 15.511 µs]
                        change: [-0.3407% -0.2812% -0.2030%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

day6 part 1             time:   [117.11 µs 117.19 µs 117.28 µs]
                        change: [+3.3259% +3.4563% +3.5791%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild

Benchmarking day6 part 2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.7s, enable flat sampling, or reduce sample count to 60.
day6 part 2             time:   [1.3293 ms 1.3296 ms 1.3298 ms]
                        change: [+0.9167% +1.0422% +1.1377%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe

day6 part1_skipping     time:   [71.233 µs 71.416 µs 71.619 µs]
                        change: [+3.6222% +3.9050% +4.2159%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

day6 part1_hashset      time:   [164.56 µs 164.57 µs 164.59 µs]
                        change: [+0.4627% +0.4937% +0.5251%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

day6 part1_skipmap      time:   [109.72 µs 109.84 µs 109.98 µs]
                        change: [-0.3983% -0.2067% -0.0040%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

day6 part1_compare      time:   [10.231 µs 10.256 µs 10.282 µs]
                        change: [-0.5355% -0.2392% +0.0560%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

day6 part1_lib          time:   [26.188 µs 26.205 µs 26.224 µs]
                        change: [-2.9293% -2.8549% -2.7909%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

day6 part2_skipping     time:   [291.07 µs 291.12 µs 291.16 µs]
                        change: [+0.9821% +1.1390% +1.2260%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

day6 part2_hashset      time:   [593.20 µs 593.26 µs 593.31 µs]
                        change: [+0.3901% +0.4844% +0.5384%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

day6 part2_skipmap      time:   [271.13 µs 271.27 µs 271.42 µs]
                        change: [+0.7361% +0.8347% +0.9363%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

day6 part2_compare      time:   [55.299 µs 55.316 µs 55.332 µs]
                        change: [-0.1887% -0.1438% -0.0967%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

day6 part2_compare_iter time:   [51.568 µs 51.589 µs 51.607 µs]
                        change: [-0.2109% -0.1746% -0.1326%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

day6 part2_lib          time:   [97.809 µs 97.825 µs 97.846 µs]
                        change: [+0.0495% +0.0720% +0.0997%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe
```
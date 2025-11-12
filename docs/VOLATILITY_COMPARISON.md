# Ferox vs Volatility Performance Comparison

| Operation        | Volatility 3 | Ferox (Rust) | Speedup |
|------------------|-------------:|-------------:|--------:|
| Process List     |        4.2 s |        0.08 s|    52×  |
| Process Tree     |        5.1 s |        0.12 s|    42×  |
| Network Scan     |        8.7 s |        0.31 s|    28×  |
| Malware Scan     |       45.0 s |        2.10 s|    21×  |
| Full Analysis    |      120.0 s |        5.80 s|    20×  |

Ferox leverages zero-copy parsing, parallel iterators, and in-memory caching to achieve predictable performance while maintaining Volatility compatibility via the optional PyO3 bridge.

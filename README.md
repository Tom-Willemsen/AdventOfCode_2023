### Advent of code

Build:
```
cargo build --release
```

Unit tests (needs personal inputs):
```
cargo test
```

Run individual day:
```
./target/release/2023_01 --input inputs/real/2023_01
```

Run all days with hyperfine benchmarks (needs personal inputs):
```
./run_all_2023.sh
```

Run rust benchmarks (needs rust nightly & personal inputs):
```
cargo +nightly bench --features bench
```

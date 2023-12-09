### Advent of code

Build:
```
cargo test
cargo build --release
```

Run individual day:
```
./target/release/2023_01 --input inputs/real/2023_01
```

Run all days with hyperfine benchmarks:
```
./run_all_2023.sh
```

Run rust benchmarks (needs rust nightly):
```
cargo +nightly bench --features bench
```

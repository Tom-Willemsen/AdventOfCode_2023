set -e

HYPERFINE_RUN_ARGS="--warmup=5 --runs 25"

mkdir -p profiling-data

for i in $(seq -w 1 25) 
do 
    if test -f "./target/release/2023_$i"; then
        CMD="./target/release/2023_$i --input inputs/real/2023_$i"
        perf record -g -F max $CMD
        perf script -F +pid > profiling-data/2023_$i.perf
        rm perf.data

        valgrind --tool=cachegrind --cache-sim=yes --branch-sim=yes --cachegrind-out-file=profiling-data/2023_$i.cachegrind --log-file=profiling-data/2023_$i.cachegrind_log -- $CMD
        cg_annotate --annotate --auto=yes --no-show-percs profiling-data/2023_$i.cachegrind > profiling-data/2023_$i.cachegrind_formatted
    fi
done;

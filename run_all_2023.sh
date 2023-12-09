set -e

HYPERFINE_RUN_ARGS="--warmup=10 --runs 50"

for i in $(seq -w 1 25) 
do 
    if test -f "./target/release/2023_$i"; then
        CMD="./target/release/2023_$i --input inputs/real/2023_$i"
        echo ""
        echo "2023 Day $i"
        $CMD
        echo ""
        # Main benchmarking
        hyperfine $HYPERFINE_RUN_ARGS -u millisecond --style basic "$CMD" 2>/dev/null
        
        # CPU energy usage benchmark
        CPU_JOULES_ITER=20
        perf stat -r $CPU_JOULES_ITER -e power/energy-pkg/ -- $CMD 2>&1 >/dev/null | grep -F "Joules"
    fi
done;

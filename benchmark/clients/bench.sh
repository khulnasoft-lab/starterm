#!/bin/bash

# API Client for benchmarking available jobs.

if [[ $# -lt 1 ]]; then
    echo "Usage: bench.sh <JOBS_URL>"
    exit 1
fi

if [ $EUID -ne 0 ]; then
    echo "Error: Must be run as root"
    exit 11
fi

user="starbench"
bench_dir="/tmp/benchdir"
gnuplot_script="/home/$user/termbench/gnuplot/summary.sh"
benchmark_dir="/home/$user/benchmark"
bench_script="$benchmark_dir/bench.sh"
jobs_url="$1"

function build_notification {
    repository="$1"
    sha="$2"
    id="$3"

    echo "Running $repository/$sha benchmark…"

    # Build the commit in question.
    rm -rf "$bench_dir"
    sudo -u $user git clone "https://github.com/$repository" "$bench_dir"
    cd "$bench_dir"
    sudo -u $user git fetch origin $sha
    sudo -u $user git checkout $sha
    sudo -u $user cargo build --release

    # Run benchmark and generate output SVG.
    iso_date=$(date +"%Y-%m-%dT%H:%M:%SZ")
    output_file="results/starterm/starbench/$iso_date-$sha.svg"
    "$bench_script" "./target/release/starterm" "./"
    master=$(ls $benchmark_dir/results/starterm/master/*.dat | tail -n 1)
    "$gnuplot_script" "$master" *.dat "$benchmark_dir/$output_file"

    # Push SVG to benchmark for long-time storage.
    cd "$benchmark_dir"
    sudo -u $user git add "$output_file"
    sudo -u $user git commit -m "Results for $repository#$sha"
    sudo -u $user git push origin master
    image_url="https://raw.githubusercontent.com/khulnasoft-lab/benchmark/master/$output_file"

    # Upload the results.
    curl -X POST --data "{\"result\": \"![results]($image_url)\"}" "$jobs_url/$id"
}

while true; do
    # Get the next available job.
    job=$(curl "$jobs_url" 2> /dev/null | jq -c '.[]' | head -n 1)

    if [ -n "$job" ]; then
        repository=$(echo "$job" | jq -r '.repository')
        sha=$(echo "$job" | jq -r '.hash')
        id=$(echo "$job" | jq -r '.id')

        # Mark job as in-progress.
        curl -X PATCH "$jobs_url/$id"

        if [ "$sha" == "null" ]; then
            echo "Starting Starterm master benchmark…"

            cd "$benchmark_dir"
            ./starterm_master.sh

            curl -X POST "$jobs_url/$id"
        else
            build_notification "$repository" "$sha" "$id"
        fi
    fi

    sleep 60
done
#!bin/bash

perf record -g ./target/debug/chess
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
google-chrome flame.svg

#/bin/sh
for i in $(seq 1 10);
do
    cargo run --bin tarascope random -o $PWD/output
done
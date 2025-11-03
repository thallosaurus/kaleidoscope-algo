#/bin/sh
for i in $(seq 1 10);
do
    cargo run --bin kaleido random -o $PWD/output
done
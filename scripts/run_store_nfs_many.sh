#!/bin/sh
for i in $(seq 1 10);
do
    cargo run --release --bin trigger -- --output-dir /mnt/blender_nfs/tarascope
done
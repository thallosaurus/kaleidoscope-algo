#!/bin/sh
for i in $(seq 1 10);
do
    cargo run --release --bin tarascope random -o /mnt/blender_nfs/tarascope
done
#!/bin/sh
for i in $(seq 1 10);
do
    docker run --rm --runtime=nvidia -e NVIDIA_VISIBLE_DEVICES=nvidia.com/gpu=all -v /mnt/blender_nfs/tarascope:/media -u $(id -u):$(id -g) tarascope tarascope-publisher --output-dir /media
done
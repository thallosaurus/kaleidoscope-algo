#!/bin/sh
for i in $(seq 1 10);
do
    docker run --rm --runtime=nvidia -e NVIDIA_VISIBLE_DEVICES=nvidia.com/gpu=all tarascope
done
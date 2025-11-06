#!/bin/bash

set -a && source .env && set +a

for i in $(seq 1 10);
do
    docker run --rm --runtime=nvidia \
    -e NVIDIA_VISIBLE_DEVICES=nvidia.com/gpu=all \
    -e PG_HOST=${PG_HOST} \
    -e PG_USER=${PG_USER} \
    -e PG_PASS=${PG_PASS} \
    -e PG_DB=${PG_DB} \
    -v /mnt/blender_nfs/tarascope:/media \
    -u $(id -u):$(id -g) \
    tarascope-publisher --output-dir /media \
done
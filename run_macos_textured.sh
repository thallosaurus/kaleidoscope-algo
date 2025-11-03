#!/bin/sh
BLENDER=/Applications/Blender.app/Contents/MacOS/Blender cargo run --bin kaleido custom -o $PWD/output 3 5 0.0 3.0 0.0 0.0 0.0 0.0 1 0 textured $PWD/red.png
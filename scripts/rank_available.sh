#!/bin/sh
#source rank/.venv/bin/activate
find $1 -name frame_00000.png -exec python3 ./rank -- "{}" \;
#/bin/sh
for i in $(seq 1 10);
do
    BLENDER=/Applications/Blender.app/Contents/MacOS/Blender cargo run --bin kaleido random -o $PWD/output
done
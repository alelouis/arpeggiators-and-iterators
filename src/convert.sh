#!/bin/bash

# Convert a tinynotation string from rust to .svg sheet music

cargo run --bin $1 | xargs python tiny_to_ly.py $1      # run rust bin and pass stdout as arg to lily converter
lilypond -dbackend=pdf -o ../out/ ../out/$1             # render to pdf with lilypond
pdf2svg ../out/$1.pdf ../out/$1.svg                     # convert to svg
sed -i '' 's/rgb(0%,0%,0%)/#cac5d3/' ../out/$1.svg        # change color
cd ../out && shopt -s extglob                           # remove temp files
rm !(*.svg)                                        
#!/bin/bash
OUTCOME_ID="$1"
INPUT_FILE="./generated/${OUTCOME_ID}.tsv"
OUTPUT_FILE="./generated/${OUTCOME_ID}.png"
gnuplot -e "input_file='$INPUT_FILE'" -e "output_file='$OUTPUT_FILE'" ./scripts/plot.gp

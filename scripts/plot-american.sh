#!/bin/bash
EVENT_ID="$1"
OUTCOME_ID="$2"
INPUT_FILE="./generated/${EVENT_ID}-${OUTCOME_ID}.tsv"
OUTPUT_FILE="./generated/${EVENT_ID}-${OUTCOME_ID}.png"
gnuplot -e "input_file='$INPUT_FILE'" -e "output_file='$OUTPUT_FILE'" ./scripts/plot-american.gp

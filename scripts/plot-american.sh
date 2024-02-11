#!/bin/bash
EVENT_ID="$1"
OUTCOME_ID="$2"
SHOW_Y2="$3"
INPUT_FILE="./generated/${EVENT_ID}-${OUTCOME_ID}.tsv"
OUTPUT_FILE="./generated/${EVENT_ID}-${OUTCOME_ID}.png"
gnuplot \
  -e "input_file='$INPUT_FILE'" \
  -e "output_file='$OUTPUT_FILE'" \
  -e "show_y2='$SHOW_Y2'" \
  ./scripts/plot-american.gp

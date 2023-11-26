#!/bin/bash

OUTCOME_ID_1="$1"
OUTCOME_ID_2="$2"

while true
do
    ./scripts/extract.py "$OUTCOME_ID_1" && ./scripts/plot.sh "$OUTCOME_ID_1"
    ./scripts/extract.py "$OUTCOME_ID_2" && ./scripts/plot.sh "$OUTCOME_ID_2"
    sleep 5
done

#!/bin/bash

EVENT_ID="$1"
OUTCOME_ID_1="$2"
OUTCOME_ID_2="$3"

while true
do
    ./scripts/extract.py "$EVENT_ID" "$OUTCOME_ID_1" && ./scripts/plot.sh "$EVENT_ID" "$OUTCOME_ID_1"
    ./scripts/extract.py "$EVENT_ID" "$OUTCOME_ID_2" && ./scripts/plot.sh "$EVENT_ID" "$OUTCOME_ID_2"
    sleep 5
done

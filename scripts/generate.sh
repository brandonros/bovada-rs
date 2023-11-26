#!/bin/bash

while true
do
    ./scripts/extract.py "1484002205" && ./scripts/plot.sh "1484002205" # Ohio
    ./scripts/extract.py "1484002204" && ./scripts/plot.sh "1484002204" # Michigan
    sleep 5
done

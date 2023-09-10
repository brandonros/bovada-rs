#!/bin/bash

while true
do
    ./scripts/extract.sh "1422096928" && ./scripts/plot.sh "1422096928" # Daniil Medvedev
    ./scripts/extract.sh "1422096929" && ./scripts/plot.sh "1422096929" # Novak Djokovic
    sleep 5
done

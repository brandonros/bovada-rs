#!/bin/bash

echo "timestamp,implied_probability" > /tmp/extracted.csv
cat output.tsv | awk -F'\t' '/PriceEvent/ && /1422096929/ {print $2, $4}' | while read -r timestamp event; do
    decimal_price=$(echo "$event" | jq -r '.price.decimal')
    # If both timestamp and decimal_price are not empty, calculate the implied probability
    if [ -n "$timestamp" ] && [ -n "$decimal_price" ]; then
        # Convert decimal odds to implied probability
        implied_probability=$(awk -v odds="$decimal_price" 'BEGIN { printf "%.2f", (1/odds) }')
        echo "$timestamp,$implied_probability" >> /tmp/extracted.csv
    fi
done

#!/bin/bash

SLUG=$1

mkdir -p ./snapshots/

while true
do
    TIMESTAMP=$(date +%s)
    echo "$TIMESTAMP"
    curl -s "https://www.bovada.lv/services/sports/event/coupon/events/A/description/${SLUG}?lang=en" -o "./snapshots/${TIMESTAMP}.json"
    sleep 30
done

#!/bin/bash

while true
do
    TIMESTAMP=$(date +%s)
    echo "$TIMESTAMP"
    curl -s "https://www.bovada.lv/services/sports/event/coupon/events/A/description/tennis/us-open/women-s-singles/cori-gauff-aryna-sabalenka-202309091615?lang=en" -o "${TIMESTAMP}.json"
    sleep 30
done

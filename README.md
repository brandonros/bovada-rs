# bovada-rs

Real time Bovada WebSocket event streaming client

## How to use

```shell
# scrape event
EVENT_SLUG="football/college-football/floridastate-florida-202311251900"
cargo run -- "$EVENT_SLUG"
# plot loop
EVENT_ID="11967642"
OUTCOME_ID_1="1484241161"
OUTCOME_ID_2="1484241162"
./scripts/generate.sh "$EVENT_ID" "$OUTCOME_ID_1" "$OUTCOME_ID_2"
# serve html + img
cd frontend && npm run server # open http://127.0.0.1:8080/?outcomeId1=1484241161&outcomeId2=1484241162
```

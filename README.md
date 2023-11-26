# bovada-rs

Real time Bovada WebSocket event streaming client

## How to use

```shell
# scrape event
EVENT_SLUG="football/college-football/floridastate-florida-202311251900"
OUTCOME_ID_1="1484241161"
OUTCOME_ID_2="1484241162"
cargo run -- "$EVENT_SLUG"
# plot loop
./scripts/generate.sh "$OUTCOME_ID_1" "$OUTCOME_ID_2"
# serve html + img
npx http-server -c-1 # open http://127.0.0.1:8080/assets/index.html
```

# bovada-rs

Real time Bovada WebSocket event streaming client

## How to use

```shell
# scrape event
EVENT_SLUG="football/college-football/ohio-state-2-michigan-3-202311251200"
cargo run -- "$EVENT_SLUG"
# plot loop
./scripts/generate.sh
# serve html + img
npx http-server -c-1 # open http://127.0.0.1:8080/assets/index.html
```

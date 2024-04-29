# bovada-rs

Real time Bovada WebSocket event streaming client

## How to use

```shell
# scrape event
export EVENT_SLUG="basketball/nba/minnesota-timberwolves-phoenix-suns-202404282140"
cargo run
# serve html + img
cd frontend
npm install
npm run server
# open http://127.0.0.1:8080/events/:eventId/implied or http://127.0.0.1:8080/events/:eventId/american
```

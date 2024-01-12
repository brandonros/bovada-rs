# bovada-rs

Real time Bovada WebSocket event streaming client

## How to use

```shell
# scrape event
EVENT_SLUG="basketball/nba/phoenix-suns-los-angeles-lakers-202401112200" bash -c 'cargo run -- "$EVENT_SLUG"'
# serve html + img
cd frontend
EVENT_SLUG="basketball/nba/phoenix-suns-los-angeles-lakers-202401112200" npm run server
# open http://127.0.0.1:8080/events/:eventId/implied or http://127.0.0.1:8080/events/:eventId/american
```

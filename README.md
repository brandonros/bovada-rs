# bovada-rs

Real time Bovada WebSocket event streaming client

## How to use

```shell
# scrape event
EVENT_SLUG="football/nfl/buffalo-bills-philadelphia-eagles-202311261625" bash -c 'cargo run -- "$EVENT_SLUG"'
# serve html + img
cd frontend && npm run server 
# open http://127.0.0.1:8080/events/:eventId/implied or http://127.0.0.1:8080/events/:eventId/american
```

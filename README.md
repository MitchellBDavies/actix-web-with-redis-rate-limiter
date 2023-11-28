# actix-web-with-redis-rate-limiter
A small repo implementing a Rust rate limiter on an Actix Web Server.

Note that this system does not yet work fully, and I'd recommend against using this in a prod system. I've had trouble finding easy documentation to use for this project at the moment, and at the time of writing this readME, actix-redis is listed as "maybe insecure" on their GitHub repo: https://github.com/actix/actix-extras.

# How to run:
The project can be ran with the following command:

```
docker-compose up --build --force-recreate -d
```
# actix-web-with-redis-rate-limiter
A small repo implementing a Rust rate limiter on an Actix Web Server utilizing Redis.

The Rate Limiter will begin rate limiting requests after a user makes more than 10 requests per 30 seconds. 

Note that this system does not yet work fully, and I would highly recommend against using this in a prod system. I've had trouble finding easy documentation to use for this project at the moment, and at the time of writing this readME, actix-redis is listed as "maybe insecure" on their GitHub repo: https://github.com/actix/actix-extras.

# How to run:
This code and it's libraries have not been fully scanned for vulnerabilities or bugs, so please **run at your own risk.**

**I am currently experiencing an issue where repeatedly running these containers will cause my WSL to hog a significant amount of memory that is not freed when containers are exited or docker enginer is powered off. I specifically must force the WSL to shutdown to release this memory. I am unsure if this is an issue with my project specifically.**

If you are interested in running the code, the project can be ran with the following command:

```
docker-compose up --build --force-recreate -d
```

Then, begin making requests to the server. I find that I can reach the server at 127.0.0.1:8080 when running these as container on docker on my local machine. '/' is not rate limited, but '/ratelimit' is. If you make more than 10 requests in a 30 second window, you should receive an HTTP 429 status code. 
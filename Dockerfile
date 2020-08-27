FROM alpine:3.12 as os

# install Rust, Cargo, Bash, Redis
RUN apk update && \
    apk upgrade && \
    apk add rust cargo redis bash

# Copy themes
COPY ./themes /themes

# Copy Redis config
COPY ./redis /etc/redis

WORKDIR /gs
COPY ./src .
RUN cargo install --path .

# Define mountable directories.
VOLUME ["/data"]

# Define working directory.
WORKDIR /data
# CMD ["gs"]
# Define default command.
# CMD ["redis-server", "/etc/redis/redis.conf"]

# Expose ports.
EXPOSE 6379 3000
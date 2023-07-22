FROM rust:1.71 as builder

WORKDIR /app/src
RUN USER=root cargo new --bin replex
COPY Cargo.toml Cargo.lock ./replex/

WORKDIR /app/src/replex
RUN cargo build --release

COPY ./ ./
RUN cargo build --release

# alpine needs musl, too much work
FROM debian:stable-slim as standalone
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY --from=builder /app/src/replex/target/release/replex /app/
EXPOSE 3001
CMD ["/app/replex"]

# FROM nginx:stable-alpine as nginx
# RUN apk update && apk add --no-cache bash openssl
FROM nginx as nginx
COPY --from=builder /app/src/replex/target/release/replex /app/
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY nginx.conf.template /etc/nginx/templates/
RUN echo "daemon off;" >> /etc/nginx/nginx.conf
RUN rm /etc/nginx/conf.d/default.conf
COPY start.sh start.sh
STOPSIGNAL SIGQUIT
CMD ./start.sh

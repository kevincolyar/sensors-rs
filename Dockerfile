FROM rust:1.62.1 as builder
MAINTAINER Kevin Colyar <kevin@colyar.net>

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

ENV LANG C.UTF-8

# Set timezone
ENV TZ=America/Los_Angeles
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

COPY --from=builder /usr/local/cargo/bin/sensors /usr/local/bin/sensors

CMD ["sensors"]

FROM rust:slim-bullseye AS build

WORKDIR /root/
COPY src /root/src
COPY Cargo.lock /root/Cargo.lock
COPY Cargo.toml /root/Cargo.toml
RUN apt-get update && apt-get install -y \
   pkg-config \
   libssl-dev \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build --chown=root:root /root/target/release/ddnsd /app/ddnsd

RUN apt-get update && apt-get install -y \
   ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN groupadd -g 10001 ddnsd && useradd -u 10000 -g ddnsd ddnsd && chmod -R 755 /app

WORKDIR /app
ENV RUST_LOG=info
USER ddnsd
CMD ["/app/ddnsd"]
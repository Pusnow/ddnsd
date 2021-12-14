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
COPY --from=build /root/target/release/ddnsd /app/ddnsd

RUN apt-get update && apt-get install -y \
   ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /app
ENV RUST_LOG=info
CMD ["/app/ddnsd"]
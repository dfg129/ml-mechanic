FROM rust:1.56.0 as builder
WORKDIR  /usr/src/spanner/
COPY . .
RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update  
WORKDIR /var/task/
COPY --from=builder /usr/src/spanner/target/release/bootstrap /usr/local/bin/bootstrap
CMD [ "bootstrap" ]


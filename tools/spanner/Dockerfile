FROM rust:1.56.0 as builder
WORKDIR  /usr/src/spanner
COPY . .
RUN cargo install --path .

# FROM debian:buster-slim
# RUN apt-get update  
# COPY --from=builder
RUN cp ./target/aarch64-unknown-linux-gnu/release/bootstrap .
CMD [ "bootstrap" ]


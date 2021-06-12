FROM rust:1.52.1-buster as build-env
WORKDIR /app
ADD . /app
RUN RUSTFLAGS="-C link-arg=-s" cargo build --release

FROM gcr.io/distroless/cc-debian10:debug
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/flusher-cli /app
CMD ["./flusher-cli", "--help"]
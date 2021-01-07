FROM rust:latest
WORKDIR /app
ADD target/x86_64-unknown-linux-musl/release/todo-actix .
CMD ["/app/todo-actix"]

#FROM rust:latest as builder

#WORKDIR /app
#COPY src/ src/
#COPY Cargo.toml .
#RUN cargo build --release

#FROM debian:buster-slim
#COPY --from=builder /app/target/release/todo-actix ./
#CMD ./todo-actix

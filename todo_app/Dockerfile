FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache libgcc

WORKDIR /app

COPY --from=build /app/target/release/todo_console_app /app/todo_console_app

ENTRYPOINT [ "./todo_console_app" ]


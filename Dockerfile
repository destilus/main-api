# FROM rust:1.57.0

# # ENV ROCKET_ADDRESS=0.0.0.0
# # ENV ROCKET_PORT=6666

# WORKDIR /main-api
# COPY . .

# RUN cargo build

# CMD ["cargo", "run"]


FROM rust:1.57.0 as builder

RUN USER=root cargo new --bin main-api
WORKDIR /main-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
# COPY ./migrations ./migrations
# COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/main_api*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /main-api/target/debug/main-api /app/

ENTRYPOINT [ "/app/main-api" ]




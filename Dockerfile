FROM rust:alpine

RUN mkdir -p /usr/src/app
COPY . /usr/src/app

WORKDIR /usr/src/app

RUN useradd -u 1000 -d /home/user user

RUN cargo b --release
ENTRYPOINT ["./target/release/undefined_bot"]

FROM rust:slim

RUN apt-get update -y && \
    apt-get install -y libssl-dev pkg-config &&\
    apt-get clean

RUN mkdir -p /usr/src/app
COPY . /usr/src/app

WORKDIR /usr/src/app

RUN useradd -u 1000 -d /home/user user

RUN cargo b --release
ENTRYPOINT ["./target/release/undefined_bot"]

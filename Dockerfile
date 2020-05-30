FROM rust:slim-stretch

RUN apt update && apt install libssl-dev pkg-config build-essential libpq-dev -y

WORKDIR /usr/src/judge-agent
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .

CMD ["/usr/local/cargo/bin/judge-agent"]
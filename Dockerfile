FROM rust:latest

RUN cargo build --release

CMD ["judge-agent/target/release/"]

RUN ./judge-agent
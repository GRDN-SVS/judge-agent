FROM rust:latest 

WORKDIR /usr/src/judge-agent
COPY . .

RUN cargo build --release

CMD ["./target/release/judge-agent"]
FROM rust as builder
WORKDIR app
COPY . .
RUN apt update && apt install -y lsb-release wget software-properties-common && \
    wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && ./llvm.sh 10
RUN cargo build --release --bin spcy

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/spcy /usr/local/bin
ENTRYPOINT ["/usr/local/bin/spcy"]

FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /src/kube-node-memory-flusher

COPY . .

RUN cargo build --release

#FROM debian:buster-slim
FROM scratch

LABEL maintainer="currycan <ansandy@foxmail.com>"

# Copy our build
COPY --from=builder /src/kube-node-memory-flusher/target/x86_64-unknown-linux-musl/release/kube-node-memory-flusher /usr/bin/

CMD ["kube-node-memory-flusher"]
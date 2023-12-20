FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /src

COPY . .

RUN cargo install --path=. --bins --root=.

#FROM debian:buster-slim
FROM scratch

LABEL maintainer="currycan <ansandy@foxmail.com>"

# Copy our build
COPY --from=builder /src/bin/ /usr/bin/

CMD ["kube-node-memory-flusher"]
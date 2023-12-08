FROM rust:latest AS builder

WORKDIR /usr/src/kube-node-memory-flusher
COPY . .
RUN set -ex; \
  rustup default nightly && rustup update; \
  cargo install --path .

FROM scratch

LABEL maintainer="currycan <ansandy@foxmail.com>"

COPY --from=builder /usr/local/bin/kube-node-memory-flusher /usr/bin/kube-node-memory-flusher
CMD ["kube-node-memory-flusher"]

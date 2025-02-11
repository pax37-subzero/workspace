# Stage 1: Build Go binary
FROM golang:1.22-bullseye AS go-builder
WORKDIR /go/src/app

# Initialize Go module
RUN go mod init workspace

# Clone and build substreams-sink-sql
RUN git clone https://github.com/streamingfast/substreams-sink-sql.git && \
    cd substreams-sink-sql/cmd/substreams-sink-sql && \
    go build && \
    go install

# Stage 2: Build Rust WASM
FROM rust:1.71 as rust-builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown

COPY . .
RUN cargo build --target wasm32-unknown-unknown --release

# Stage 3: Final image
FROM ubuntu:22.04

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    apt-get -y install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

# Create required directory structure
RUN mkdir -p target/wasm32-unknown-unknown/release proto

# Copy Go binary
COPY --from=go-builder /go/bin/substreams-sink-sql /usr/local/bin/

# Copy files maintaining directory structure
COPY --from=rust-builder /app/target/wasm32-unknown-unknown/release/substreams.wasm ./target/wasm32-unknown-unknown/release/
COPY --from=rust-builder /app/substreams.yaml ./
COPY --from=rust-builder /app/schema.sql ./
COPY --from=rust-builder /app/proto/mydata.proto ./proto/

# Create start script with setup
RUN echo '#!/bin/bash\n\
case "$MODE" in\n\
 "historical")\n\
   echo "Running historical data import..."\n\
   substreams-sink-sql setup "$DSN" ./substreams.yaml\n\
   echo "Starting historical sink with optimized settings..."\n\
   substreams-sink-sql run "$DSN" ./substreams.yaml \\\n\
     --undo-buffer-size 128 \\\n\
     --batch-block-flush-interval 5000 \\\n\
     --batch-row-flush-interval 200000\n\
   ;;\n\
 "realtime")\n\
   echo "Running setup..."\n\
   substreams-sink-sql setup "$DSN" ./substreams.yaml\n\
   echo "Starting realtime sink..."\n\
   substreams-sink-sql run "$DSN" ./substreams.yaml --undo-buffer-size 64\n\
   ;;\n\
 *)\n\
   echo "Please specify MODE=historical or MODE=realtime"\n\
   exit 1\n\
   ;;\n\
esac' > ./start.sh && chmod +x ./start.sh

CMD ["./start.sh"]
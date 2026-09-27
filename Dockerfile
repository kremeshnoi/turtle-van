# Build stage
FROM ubuntu:24.04 AS builder

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    cmake \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    unzip \
    python3 \
    python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Setup deno (JS runtime required by yt-dlp for YouTube JS challenge solving)
RUN curl -fsSL https://deno.land/install.sh | sh
ENV PATH="/root/.deno/bin:${PATH}"

# Setup yt-dlp (via pip for latest version + yt-dlp-ejs)
RUN pip3 install --break-system-packages \
    "yt-dlp[default] @ https://github.com/yt-dlp/yt-dlp/archive/master.tar.gz"

WORKDIR /app
COPY --from=builder /app/target/release/turtle-van ./turtle-van

CMD ["./turtle-van"]

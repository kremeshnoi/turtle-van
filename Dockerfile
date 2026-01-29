# Setup environment
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    gcc \
    git \
    python3 \
    python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Setup Rust environment
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN /root/.cargo/bin/rustup default stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Setup deno (JS runtime required by yt-dlp for YouTube JS challenge solving)
RUN curl -fsSL https://deno.land/install.sh | sh
ENV PATH="/root/.deno/bin:${PATH}"

# Setup yt-dlp (via pip for latest version + yt-dlp-ejs)
RUN pip3 install "yt-dlp[default] @ https://github.com/yt-dlp/yt-dlp/archive/master.tar.gz"

# Setup App
WORKDIR /app
COPY . .

# Build App
RUN cargo build --release

# Run App
CMD ["./target/release/turtle-van"]

FROM tnk4on/yt-dlp:latest as yt-dlp

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    gcc \
    git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN /root/.cargo/bin/rustup default nightly

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version

COPY --from=yt-dlp /usr/local/bin/yt-dlp /usr/local/bin/yt-dlp

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/turtle-van"]
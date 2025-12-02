FROM rust:1.82-slim

RUN apt-get update && apt-get install -y \
    nodejs \
    npm \
    pkg-config \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

WORKDIR /app/pixelify_web
RUN npm install

WORKDIR /app
CMD ["bash"]

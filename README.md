# Pixelify  

**In Development**

This README is both a traditional README and a road map of planned or features in progress/

Convert normal images into crisp pixel-art sprites using Rust + WASM + React.

Pixelify takes any uploaded image, processes it through a Rust backend compiled to WebAssembly, and outputs a pixelated sprite with customizable resolution, palette, and dithering options.
Designed to be fast, portable, and game-dev-friendly.

### Format Support Note

When creating a simple grayscale feature I ran into upstream bugs with decoding a jpg image into bytes then into a grayscale format and writing those back.
The crate `Image` (while really powerful and amazing) has some limitations with decoding from one type then re-encoding to another type.

So for now I will offer full support for PNG image types and then a working "best effort" for other types. 

Later I hope to revisit this issue and or side step it by just converting non png types into png types on the backend and go from there.

## Features

- Convert any image into pixel art
- Adjustable pixel resolution (8×8, 16×16, 32×32…)
- Palette reduction (NES, Game Boy, custom)
- Optional dithering
- Built-in Rust core library (`pixelify_core`)
- WASM processing in the browser (`pixelify_wasm`)
- React + TypeScript frontend (`pixelify_web`)
- Full Docker-based development environment

## Project Structure

    pixelify/

        pixelify_core/ # Pure Rust image/pixel logic

        pixelify_cli/ # Rust CLI for local usage

        pixelify_wasm/ # WASM bindings for the browser

        pixelify_web/ # React + TypeScript frontend (Vite)

        Dockerfile # Dev container

        docker-compose.yml

        Cargo.toml # Workspace config

## Running in Docker (I Recommend this way since there are a lot of moving parts)

Prerequisites:
- Docker Desktop installed  
- This repo cloned locally  

Start the dev environment:

docker compose up --build

Now open the frontend:

http://localhost:5173

To enter the container shell:

docker compose exec dev bash

---

## Local Dev + WASM + CLI

## Local Development (non-Docker)

### Install frontend deps

cd pixelify_web
npm install
npm run dev

cargo build --workspace

docker compose exec dev bash
cd pixelify_wasm
cargo build --target wasm32-unknown-unknown

## CLI Usage

cargo run -p pixelify_cli -- input.png output.png --width 32 --height 32

---

## Tech Stack + Roadmap

## Tech Stack

**Backend / Core**
- Rust
- `image` crate
- color quantization algorithms
- WASM (`wasm32-unknown-unknown`)

**Frontend**
- React
- TypeScript
- Vite

**Dev Environment**
- Docker / docker-compose
- Node + Rust inside container

---

## Roadmap

- [ ] Add NES, GB, Pico-8 palettes
- [ ] Add dithering options
- [ ] Add sprite sheet generator
- [ ] Add tilemap mode
- [ ] Add downloadable metadata
- [ ] Add animation previewer
- [ ] Deploy to Vercel / Netlify

This is in the early stages of development, so as I proceed through this project, items listed above are subject to modification, addition, or removal based on how I see fit.
If you have any cool ideas or features you'd like to see accessible/ available, then let me know!

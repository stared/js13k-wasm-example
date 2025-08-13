# JS13K Space Invaders

A tiny Space Invaders game written in Rust and compiled to WASM for the js13k competition.

## Prerequisites

- Rust with wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`
- (Optional) wasm-opt for further optimization

## Build

```bash
./build.sh
```

## Check Size

```bash
./check-size.sh
```

## Play

```bash
python3 -m http.server 8000
```

Then open http://localhost:8000 in your browser.

## Controls

- **Arrow Left**: Move left
- **Arrow Right**: Move right  
- **Space**: Shoot

## Game Features

- Classic Space Invaders gameplay
- Minimal size optimized for js13k (< 13KB zipped)
- Written in Rust with no_std for minimal WASM size
- Uses wee_alloc for small memory allocator
- All game logic and rendering in Rust/WASM
- Minimal JavaScript (< 500 bytes inline)
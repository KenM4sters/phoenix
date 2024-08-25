## Overview
Phoenix is a simple 2D "bullet-hell shmup" written in Rust using wgpu for graphics rendering.

This currently in early stages of development and is frequently being worked on.


## Design

```rust

let model = ModelBuilder::new()
    .uri("")
    .shader("")
    .position((0, 0, 0).into())
    .scale((10, 10, 10).into());


```
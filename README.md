![scene](./renders/scene.png)
![checkered floor](./renders/checkered_floor.png)
![mirror ball](./renders/mirror_ball.png)
![Cornell Box](./renders/cornell_box.png)
![red ball](./renders/red_ball.png)

# this pathtracer has the following interesting features:
- [x] importance sampling

# usage
```bash
cargo run --release -- [--multiplier=N] [scene_file.toml] or --all
```

for example
```bash
cargo run --release -- --multiplier=2 scene.toml
```

or 
```bash
cargo run --release -- --all
```

for example scenes check the scenes folder
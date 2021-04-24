# kdisk
A tiny utility for displaying disk usage, written in Rust.

![screenshot](./assets/screenshot.png)

## Installing

### From source
```bash
git clone https://github.com/keesvv/kdisk
cd kdisk
cargo build --release
sudo cp ./target/release/kdisk /usr/local/bin
```

## Todo
- [x] Display filesystem labels
- [X] Sorting
- [ ] Change bar color depending on usage percentage

## License
[MIT](./LICENSE)

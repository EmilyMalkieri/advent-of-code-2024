# Advent of Code '24

AoC doesn't like you publishing their input files. Put them in `inputs/**/input.txt` and we'll auto-ignore them.

Use [`watchexec`](https://github.com/watchexec/watchexec) to continually run unit tests:

```bash
brew install watchexec
# or
cargo install --locked watchexec-cli
```

```bash
watchexec -r cargo test
```

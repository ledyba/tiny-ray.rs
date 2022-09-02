# Tiny ray-tracing engine

![Lighted Spheres](./samples/lighted-spheres.png)

![Many Spheres](./samples/many-spheres.png)

# How to run

```shell
cargo run --release -- spheres
cargo run --release -- many-spheres
cargo run --release -- lighted-spheres
cargo run --release -- --width 1600 --height 1600 --num-rays 8192 --output "cornell.png" cornell
```

# References

- [週末レイトレーシング (翻訳)](https://inzkyk.xyz/ray_tracing_in_one_weekend/)
- [週末レイトレーシング - 達人出版会](https://tatsu-zine.com/books/ray-tracing-part1)
- [dannyfritz/awesome-ray-tracing: Curated list of ray tracing resources](https://github.com/dannyfritz/awesome-ray-tracing)

# Tiny ray-tracing engine

![Lighted Spheres](sample-images/lighted-spheres.png)

![Cornell Box](sample-images/cornell-box.png)

![Many Spheres](sample-images/many-spheres.png)

# How to run

```shell
cargo run --release -- spheres
cargo run --release -- many-spheres
cargo run --release -- many-boxes
cargo run --release -- lighted-spheres
cargo run --release -- cornell-box
# with full options
cargo run --release -- \
  --width 1600 \
  --height 1600 \
  --num-rays 8192 \
  --num-reflections 256 \
  --output "cornell.png" \
  cornell-box
```

# References

- [週末レイトレーシング (翻訳)](https://inzkyk.xyz/ray_tracing_in_one_weekend/)
- [週末レイトレーシング - 達人出版会](https://tatsu-zine.com/books/ray-tracing-part1)
- [awesome-ray-tracing: Curated list of ray tracing resources](https://github.com/dannyfritz/awesome-ray-tracing)
- [Computer Graphics - memoRANDOM](https://rayspace.xyz/CG/)

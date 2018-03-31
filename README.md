This piece of software implements a pretty simple idea of fitting two images into one. Without any gamma tricking, thumbnail tricking, rarjpegging and kinda that. It simply makes one image the carrier wave for another one.

Build
=====
```
cargo build --release
cargo install
```

Usage
=====
```
~/.cargo/hide_my_g --mode encode base.png mixed.png > result.png
```
or
```
~/.cargo/hide_my_g --mode decode result.png > decoded.png
```

Notes
=====
* The resolution of input images must be the same.
* For input, it accepts any image formats that the [`images` crate support](https://crates.io/crates/image). The output format is always PNG now.

Roadmap
=======
* Automatic resolution fitting
* Optional dithering
* More flexible output
* Tuning the ratio of the signal levels of the base and mixed images

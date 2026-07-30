# CAD Kernel (name pending)

A greenfield B-rep solid-modeling kernel in Rust, built API-first: the kernel
and its programmatic modeling API are the product; any GUI is a thin client
added later. The design contract — decisions, layering, and open questions —
lives in [`docs/DESIGN.md`](docs/DESIGN.md). The project name is still pending
(see Q9); the workspace is deliberately nameless until it lands.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

This covers **every** build configuration, the optional `interval` cargo
feature included: that feature's transcendentals come from the in-repo
`interval-transcendentals` crate — pure Rust over the same `libm` the
kernel already uses — so no kernel build has a copyleft dependency or a
C build step. The one LGPL-3.0+ dependency anywhere in the repo is
`inari/gmp` as that crate's optional differential-certification
*dev*-dependency, in its own excluded workspace; dev-dependencies of a
path dependency never enter the dependent's graph, so no kernel build
pulls it in.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

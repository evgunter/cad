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

**Exception — the optional `interval` cargo feature**: enabling it links
LGPL-3.0+ code (`gmp-mpfr-sys`/`rug`, pulled in via `inari/gmp` for
interval transcendentals), so builds with that feature carry the
corresponding LGPL compliance obligations. Default builds have no LGPL
dependencies and no C build step (see issue #4; a post-M7 in-house
replacement that drops the LGPL dependency is on the roadmap in
`docs/DESIGN.md`).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

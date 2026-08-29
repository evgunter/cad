# CAD Kernel (name pending)

A greenfield B-rep solid-modeling kernel in Rust, built API-first: the kernel
and its programmatic modeling API are the product, and the GUI is a thin
client over them — every operation it performs is itself API, testable with
no renderer present. The design contract — decisions, layering, and open questions —
lives in [`docs/DESIGN.md`](docs/DESIGN.md). The project name is still pending
(see Q9); the workspace is deliberately nameless until it lands, and `pncad`
appears throughout as a greppable placeholder.

## Start here

**[`docs/GUIDE.md`](docs/GUIDE.md)** is the guide: a quickstart for Rust and
Python, then the canonical journey — author, validate, measure, tessellate,
cross-check, export — worked in both languages. Every code block in it is
executed, as a doctest or by the Python test runner.

- [`docs/guide/examples.md`](docs/guide/examples.md) — the corpus as the
  example set: every demo-tour stop and document-corpus entry, mapped to
  what each demonstrates.
- [`docs/guide/fail-loud.md`](docs/guide/fail-loud.md) — this kernel refuses
  rather than guessing. What the refusals look like, and how to read one.
- [`docs/guide/selecting.md`](docs/guide/selecting.md) — naming and
  selecting entities: materializers, the pattern language, the geometric
  filters, and the detect/declare protocol.
- [`docs/guide/north-star-audit.md`](docs/guide/north-star-audit.md) — what
  the Python bindings can author today, and the named gaps.

```console
$ cargo build                                  # the kernel and the pncad façade
$ cargo test --doc -p pncad                    # runs every Rust block in the guide
$ cd demos/tour && cargo run --release -- ../out   # render the example corpus
$ ./crates/pncad-py/run-python-tests.sh        # build and exercise the bindings
$ cargo run -p viewer --features app -- [document.pncad]   # the v1 GUI
```

Depend on the façade crate `pncad` and nothing else — it re-exports every
kernel crate as a module and offers a curated prelude. The Python package is
the same kernel through PyO3; see
[`crates/pncad-py/README.md`](crates/pncad-py/README.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

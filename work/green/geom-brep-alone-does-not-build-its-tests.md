---
id: geom-brep-alone-does-not-build-its-tests
kind: issue
title: cargo nextest run -p geom-brep alone does not build its integration tests: Reach::fixture is behind geom-brep's own test-support, which only a workspace build turns on
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by `onecopy/foot-certificate` running the touched crates' full suites. The implementer discipline asks every lane to run `cargo nextest run -p <crate>`.

`cargo nextest run -p geom-brep` fails to compile the `all` test binary:

> error[E0599]: no variant, associated function, or constant named `fixture` found for enum `Reach<T>`

It fails at `tests/shared/surf.rs`, `tests/pair_section.rs`, `tests/plane_cone_arc.rs` and `tests/section_bounds.rs`. `Reach::fixture` (`crates/geom-brep/src/extent.rs`) is `#[cfg(any(test, feature = "test-support"))]`. `cfg(test)` does not reach an integration test, and geom-brep's `[dev-dependencies]` turn on `test-support` for `geom` and `geom-core` but not for geom-brep itself. A workspace build gets the feature by unification with a crate that turns it on (`topo`'s `test-support` forwards it), so CI is green and only the per-crate run breaks. The fixture doors came in with `72f10792d` (ONEHOME).

Workaround: `cargo nextest run -p geom-brep --features test-support`.

Fix: geom-brep's dev-dependencies name the crate itself with `features = ["test-support"]`, the usual self-dev-dependency pattern. Alternatively, the discipline's per-crate command passes the feature.

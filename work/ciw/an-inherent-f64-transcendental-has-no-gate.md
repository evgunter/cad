---
id: an-inherent-f64-transcendental-has-no-gate
kind: issue
title: Nothing stops kernel production code calling an inherent f64 transcendental (the platform libm D9 rules out); sweep_places was one
status: open
opened: 2026-10-16
priority: P3
cost: M
refs: [sweep-places-reads-the-platform-libm]
---


## Finding

D9 (`docs/DESIGN.md`; `geom_core::Real for f64`'s doc in
`crates/geom-core/src/real.rs`) routes every transcendental through
the pure-Rust `libm` crate, because the platform libm's `sin`, `cos`
and `atan2` differ across platforms in the last ulp, which is enough to
flip a marginal predicate. An inherent call on a bare `f64`
(`x.atan2(y)`, `t.sin_cos()`) compiles silently and goes to the
platform libm.

`work/inlane/sweep-places-reads-the-platform-libm.md` found one:
`sweep::sweep_places`' turn angle, now `<f64 as Real>::atan2`. Its
audit read sweep's production call sites one by one. The others are on
a generic `T`, so they are already `Real`'s. Nothing holds that line in
place for sweep or any other crate.

## Repair shape

Two options:
- A gate in `scripts/gates/` that greps kernel `src/` outside
  `#[cfg(test)]` modules for `.sin(`, `.cos(`, `.sin_cos(`, `.tan(`,
  `.atan(`, `.atan2(`, `.asin(`, `.acos(`, `.sinh(`, `.cosh(`,
  `.tanh(`, `.exp(`, `.exp2(`, `.exp_m1(`, `.ln(`, `.ln_1p(`,
  `.log*(`, `.powf(`, `.cbrt(` and `.hypot(` on an `f64` receiver. A grep cannot see the receiver's
  type, so it would need an allowlist of generic sites, or the
  receiver's type read from a parser.
- `clippy::disallowed_methods` on `f64::{sin, cos, ...}`, scoped to
  non-test code. Clippy applies it per crate and cannot exempt
  `#[cfg(test)]`, so test modules would need an `allow` each.

Either way, the first run is the census of today's sites. One known
population: `crates/sweep/src/test_support.rs` (the `test-support`
feature) makes about forty such calls to build fixture inputs. A gate
decides whether feature-shipped fixtures are in scope.

---
id: raw-f64-min-max-store-a-codegen-chosen-zero-sign
kind: issue
title: Raw f64::min/max across the kernel store a zero sign the codegen chooses, which differs between debug and release
status: open
opened: 2026-10-01
priority: P2
cost: M
---


(LINALG, `linalg-zeros`) Found as the blind spot of the sweep that put
the interval backend's endpoint choices on a rule
(`work/linalg/interval-backend-signed-zero-conventions.md`, the SCALAR
section).

`f64::min`/`f64::max` are IEEE 754-2008 `minNum`/`maxNum`, which leave
the sign unspecified when the operands are zeros of opposite sign. What
rustc 1.97 on x86-64 answers depends on the optimisation level and on
whether the operands are constants. A standalone probe (both operand
orders):

| build | `min(-0,+0)` | `min(+0,-0)` | `max(-0,+0)` | `max(+0,-0)` |
|---|---|---|---|---|
| debug | `+0` | `-0` | `+0` | `-0` |
| release, opaque operands | `+0` | `+0` | `+0` | `+0` |
| release, constant operands | `-0` | `-0` | `+0` | `+0` |

So a stored value taken from one of these at a zero tie is a function
of the build as well as of the source and inputs. D9's "same build"
wording survives it, but anything that compares bits across builds does
not: a golden recorded under one profile and checked under another, a
debug run against a release run of the same history, and any doc that
claims cross-platform bit identity.

The interval backend and `Certification::clamped_to` now decide the tie
by rule (`interval-transcendentals/src/lib.rs`, "Signed zeros"). The
rest of the tree was not swept: a grep for `.min(`/`.max(`/`f64::min`/
`f64::max` under `crates/*/src` returns ~850 lines across ~160 files,
most of them on integers, decorations or `Real` (whose `f64` lane,
`crates/geom-core/src/real.rs`'s `impl Real for f64` `min`/`max`, is
comparison-based and deterministic). Two that are plainly `f64` folds
whose result is stored:

- `crates/editor-core/src/mc.rs`'s summary statistics:
  `values.iter().copied().fold(f64::INFINITY, f64::min)` (and the `max`
  twin), whose report `mc.rs`'s module docs claim is bit-identical
  across rayon schedules;
- `crates/editor-core/src/resolve/pick.rs`:
  `spans.iter().map(|s| s.t_hi).fold(f64::INFINITY, f64::min)`.

## What a taker owes

Triage the hits to the ones on `f64` whose result can be a zero and is
stored or emitted, and give each a rule (or route it through `Real::min`
/ `Real::max`, which decide by comparison). A mechanical guard is
probably cheap: clippy's `disallowed-methods` can name `f64::min` and
`f64::max` workspace-wide, with the backend's two rule helpers and any
audited site allowed by name.

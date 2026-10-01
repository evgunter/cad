---
id: interval-backend-signed-zero-conventions
kind: issue
title: The interval backend's signed-zero conventions: a stale inari comment at interval.rs, abs(−0.0) = −0.0, and * and / dropping the bit
status: closed
opened: 2026-09-05
priority: P1
cost: M
closed: 2026-10-01
---


(PROPS orchestrator) Filed from the ONB-measure lane (PR #1939, table 1).

- `crates/geom-core/src/interval.rs:904-909` says inari canonicalises a
  point zero's endpoint representation and weakens an assertion to
  value equality. Stale: `interval.rs` wraps the in-repo
  `interval-transcendentals` (since M5 PR 1), whose `DInterval::point`
  stores both endpoints verbatim and keeps the sign bit. A doc and
  assertion fix.
- `interval-transcendentals/src/ops.rs:19-26`: `DInterval::abs([−0.0, −0.0])`
  returns `[−0.0, −0.0]` (the `lo >= 0.0` arm admits `−0.0`) where
  `f64::abs(−0.0)` is `+0.0`. Set-equal, so not a soundness defect, but
  `Real::copysign`'s hull is built from `abs()`.
- `*` and `/` drop the sign bit (`(−1)·[+0,+0]` → `[+0,+0]`; `normalize`
  likewise) while `+`, `−` and unary `−` keep it: the backend has no
  stated convention either way. Whether one is wanted is decided by the
  sign-hull ruling (this item's neighbour): under (c′) nothing depends
  on the bit and the convention is "none, and say so"; under (c) it
  would have to be "preserved everywhere", a backend invariant.

(SCALAR, `scalar/cert-diff`) A fourth instance, from the certification
door differential (`crates/geom-core/tests/certification_door_differential.rs`):
where `DInterval::hull` (`interval-transcendentals/src/ops.rs`) and
`Certification::clamped_to` (`crates/geom-core/src/interval/certification.rs`)
choose an endpoint between zeros of opposite sign, the sign comes from
`f64::min`/`f64::max` (IEEE 754-2008 `minNum`/`maxNum`), which leave it
unspecified for zeros of opposite sign. This build's codegen (x86-64,
1.97) picks the second operand at the door, so `hull` is not
bit-commutative there: `hull([-0, 1], [0, 1])` is `[0, 1]` and
`hull([0, 1], [-0, 1])` is `[-0, 1]`; a clamp of `[-0, 1]` to `[0, 1]`
gives `[0, 1]` and of `[0, 1]` to `[-0, 1]` gives `[-0, 1]`. That is a
codegen fact, not a language one. A standalone probe at 1.97
(`min`/`max` over `0` and `-0`, both orders) answers three ways:
`rustc -O` with opaque operands gives `+0` all four times; `rustc -O`
on constants folds `min` to `-0` and `max` to `+0`; unoptimised, the
first operand wins. So it is also a determinism hazard (D9): the
stored bits of a hull or clamp endpoint at a signed zero can change
with the compiler, the target or the optimisation level, with no
change to the source. The differential
compares those zeros by value and says why. Whatever convention the
sign-hull ruling settles on would have to be spelled past
`f64::min`/`max` to be one.

## Closed (2026-10-01)

Closed by #3687, and settled by #2468's option 1: nothing in the frame
construction depends on a zero's sign.

- **The stale inari comment is gone**, and `from_f64`'s assertion is
  bitwise again.
- **`abs` returns `+0`** at zero endpoints.
- **The backend's convention is stated at its home**
  (`interval-transcendentals/src/lib.rs`, "Signed zeros"). No
  enclosure semantics depend on a zero's sign; the identity channels
  read it.
- **Endpoint choices at an opposite-zero tie follow one rule**: lower
  takes −0, upper takes +0, in either operand order. The rule covers
  `hull`, `intersection`, `min_i`/`max_i`, the `×`/`÷` folds and
  `Certification::clamped_to`. It replaces `minNum`/`maxNum`'s
  unspecified pick, which made the old `×` fold store a different bit
  in debug and in release. The door differential now compares bits.
- **The rest of the tree's raw `f64::min`/`max`** is on
  `work/issues/raw-f64-min-max-store-a-codegen-chosen-zero-sign.md`.

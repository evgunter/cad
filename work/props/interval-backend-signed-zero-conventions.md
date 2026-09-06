---
id: interval-backend-signed-zero-conventions
kind: issue
title: The interval backend's signed-zero conventions: a stale inari comment at interval.rs, abs(−0.0) = −0.0, and * and / dropping the bit
status: open
opened: 2026-09-05
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

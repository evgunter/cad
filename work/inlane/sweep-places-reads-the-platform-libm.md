---
id: sweep-places-reads-the-platform-libm
kind: issue
title: sweep_places turns its stations by std's atan2, the platform libm D9 rules out; sweep's other f64 trig sites are unaudited
status: open
opened: 2026-10-11
priority: P2
refs: [the-skin-computes-its-values-in-t]
---


D9 (`docs/DESIGN.md`; `geom_core::Real for f64`'s doc,
`crates/geom-core/src/real.rs`) puts every transcendental through the
pure-Rust `libm` crate: system libm `sin`/`cos` differ across platforms
in the last ulp, enough to flip a marginal predicate. An inherent `f64`
call (`x.atan2(y)`, `x.sin_cos()` on a bare `f64`) goes to the platform
libm instead.

`the-skin-computes-its-values-in-t` (#4726) moved the skin's arc
conversion off the inherent calls, because generic code calls `Real`'s.
It left `sweep_places` (`crates/sweep/src/skin.rs`, the turn's
`sin.atan2(cos)` on `f64`) on std's `atan2`. Its pair review raised
this, and it was deferred for scope: `sweep_places` is the placements
path, and this item owns it. Output stability is not the reason; the
fix re-baselines whatever swept bodies move, and says what moved.

Owed:
- `sweep_places`' turn angle through `Real::atan2`;
- an audit of sweep's other trig call sites on bare `f64`, read one by
  one, since many of them are on a generic `T` and already `Real`'s.
  The candidates at #4726's head, outside tests and fixtures:
  - `crates/sweep/src/revolve/axis.rs:463`, `(dr.abs() / dz.abs()).atan()`;
  - `revolve/tube.rs:411`, `t0.sin_cos()`;
  - `blend/arms.rs:809`, `half_angle.sin_cos()`;
  - `blend/surgery.rs:2489` and `:5148`, `atan2`;
  - `blend/reach.rs:463`, `:530`, `:531`, `:626`, `:627` and `:1654`;
  - `swept.rs:1798`–`:1800`;
  - the `blend/open/end_face.rs` meter.

  A site whose receiver is `f64` moves to `Real`'s call, or says why
  it is exempt.

---
id: sweep-places-reads-the-platform-libm
kind: issue
title: sweep_places turns its stations by std's atan2, the platform libm D9 rules out; sweep's other f64 trig sites are unaudited
status: closed
opened: 2026-10-11
priority: P2
refs: [the-skin-computes-its-values-in-t]
cost: E
closed: 2026-10-16
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

## Closed

`sweep_places`' turn angle is `<f64 as Real>::atan2` (`crates/sweep/src/skin.rs`), the pure-Rust `libm` D9 prescribes.

The audit read sweep's production trig call sites one by one: every
`.sin`, `.cos`, `.sin_cos`, `.tan`, `.atan`, `.atan2`, `.asin`,
`.acos`, `.exp`, `.ln`, `.powf` and `.hypot` call outside a
`#[cfg(test)]` module. Every site but `sweep_places` has a receiver of
the function's generic `T`, so it is `Real`'s already:
- `revolve/axis.rs` `classify_segment`'s `atan`;
- `revolve/tube.rs` `build`'s window `sin_cos`;
- `blend/arms.rs` `Meridian::trace`'s cone `sin_cos`;
- `blend/surgery.rs` `CircleFrame::past`'s and `attach_contact`'s `atan2`;
- `blend/reach.rs` `surface_distance`, `sheet_fn_own`, `sheet_fn` and
  `sheet_clip`;
- `swept.rs`'s `atan`;
- `skin.rs` `segment_curve`.

The rest sit in test modules: the `end_face` meter and its fuzz,
`reach.rs`' and `surgery.rs`' fixtures, `axis.rs`'s rows, and the
item's `swept.rs` candidates, now inside `swept.rs`'s `mod tests`.

`crates/sweep/src/test_support.rs` (under `test-support`, a dev
feature) makes about forty bare-`f64` platform-libm calls (`tan`, `acos`,
`asin`, `atan2`, `sin`, `cos`, `hypot`). They build fixture inputs, so
they shape goldens per platform, but no kernel output. They are exempt
here and named in the gate item's census.

A second pass looked for the free-function spellings (`f64::sin(..)`,
`core::f64::..`) and found none.

One swept body moved: the swept elbow. Of its nine stations, station
6's turn angle is one ulp apart between glibc and `libm`
(1.19383730525368592 against 1.19383730525368614), and the rest agree.
`mesh`'s `d9_mesh_goldens` digests of the elbow at both δ moved, at all
three ε rows, and were re-baselined. The old digests come back with the
old call. sweep's and profile's own goldens do not see the elbow's
mesh. Nothing holds the line in place for future
code: `work/ciw/an-inherent-f64-transcendental-has-no-gate.md`.

---
id: curve-recognition-roots-its-certified-sups-to-nearest
kind: issue
title: recognize_curve roots a certified sup and a lower bound to nearest, so each can land an ulp on the unsafe side
status: closed
opened: 2026-10-01
priority: P3
cost: E
closed: 2026-11-20
branch: exch/arc-map-obligation
---


## The finding

Found by `linalg/certification-gains-a-sqrt-door`'s sweep for roots
taken on bound endpoints. In `crates/step-import/src/recognize_curve.rs`:

- The line arm (INV-C3): `let delta_line = sup.abs().sqrt();`. Here
  `sup` is the certified `dist(P, line)²` composite sup in m², and its
  root to nearest can land half an ulp below the true distance bound.
  That bound is then compared against `eps_in`.
- The circle arm (INV-C2):
  `inner = (radius − δ_s)² − δ_p²; lower = inner.sqrt()`. This is a
  LOWER bound on a distance, and it is computed to nearest throughout,
  so it can land above the true value. It enters
  `m = δ_s.max(radius − lower)`, an upper bound, from the unsafe
  direction. `residual = m.hypot(δ_p)` also rounds to nearest.

Both errors are ulp-relative against a tolerance comparison, so they
matter only at the threshold. They are recorded because the rest of
the arm is certified: the sups come out of interval arithmetic, and
the conversions to metres are the last step that rounds to nearest.
The interval spelling would keep them outward. Build the composite's
sup as an `Interval`, take `.sqrt()` through
`geom_core::interval::certification::Certification`, and read `.mag()`
or `.lo()`. The file would then join `certification-doors.sh`'s
importer list, which holds it to that gate's rules.

## Closed

EXCH unit B (`exch/arc-map-obligation`). Both arms compute their metre
conversions of the composites' sups at `Interval` and read them
outward: the line roots its cylinder sup through `Certification::sqrt`
and reads `.mag()`; the circle's INV-C1 `δ_s = S/r`, INV-C2's lower
bound (the root's `.lo()`, taken only while `r − δ_s` is certified
positive), `m`, the hypot and an open arc's overrun term are interval
operations read on the safe side. The distances the line's coverage
and the straight map's Greville hull compare are formed at `Interval`
too, but the quantities they compare — the line's direction and chord
length, the Greville abscissae `ξᵢ` and positions `ℓ(ξᵢ)`, the
coverage's azimuths, `total` and overrun angle — are `f64` closed forms
treated as exact (`docs/DESIGN.md` `[simple.conditioned]`), stated at
each claim site (`recognize_curve` module docs, INV-C2; `straight_map`;
`arc_residual`). `recognize_curve.rs` joined `certification-doors.sh`'s
importer list.

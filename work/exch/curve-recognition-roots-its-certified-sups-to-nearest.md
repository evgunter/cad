---
id: curve-recognition-roots-its-certified-sups-to-nearest
kind: issue
title: recognize_curve roots a certified sup and a lower bound to nearest, so each can land an ulp on the unsafe side
status: open
opened: 2026-10-01
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

---
id: nurbs-cert-certificate-is-rounded-to-nearest-end-to-end
kind: issue
title: nurbs_cert's cert() and the split selection's steps are evaluated to nearest end to end, so the certificate can sit ulps on the unsafe side
status: open
opened: 2026-10-01
---


## The finding

Found by `linalg/certification-gains-a-sqrt-door`'s root sweep (PR
#3727), and filed at review. In `crates/mesh/src/nurbs_cert.rs`, two
things are computed from certified sups in `f64` rounded to nearest,
with no step outward:

- `NurbsFaceBound::cert`: the chord-error certificate
  `0.25·(muu·au² + 2·muv·au·av + mvv·av²)`.
- `split_steps` and `step_v_at`: the steps that saturate that ellipse
  at `δ_s`. These are `t_star = √(mvv/muu)`, `hv = √(δ_s/q)` and
  `(√(b² + mvv·rem) − b)/mvv`.

`muu`, `muv`, `mvv`, `mu1` and `mv1` leave `cell_component` as outward
bounds. Every operation after that can round either way. So a cell
sized to exactly `δ_s` can carry a true chord error a few ulps above
the certificate, and `cert` can read a few ulps below the true bound.

Whether this matters depends on what `δ_s` is compared against, and
whether that has slack. If the budget always keeps a margin of many
ulps, this is a documentation note. If anything decides on
`cert ≤ tol` at equality, it is a soundness question. Either way the
root is not the special step: the whole formula is to nearest. An
outward spelling would put the ellipse through interval arithmetic and
read `.mag()` for `cert`, and step the selected `h` inward by one ulp.

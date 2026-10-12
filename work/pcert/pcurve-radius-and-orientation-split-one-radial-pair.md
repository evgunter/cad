---
id: pcurve-radius-and-orientation-split-one-radial-pair
kind: issue
title: pcurve envelope: the moving and meridian classes read one radial pair as Radius plus Orientation, a lever up to 3
status: open
opened: 2026-10-12
priority: P2
cost: E
---

(PCERT implementer, from review 4842's first reviewer, who executed
it; filed by the fix pass of
`pcurve-envelope-terms-sum-the-cos-and-sin-coefficients-of-a-deviation`.)

## What

`incidence` in `crates/geom-brep/src/pcurve_cache.rs` reads a moving
or meridian class's radial pair `(a_r, b_r)`, the radial part of one
`a·cos t + b·sin t`, in two slots and sums them:

- **Radius** `|ρ − ‖a_r‖|` (`norm_gap`), the `cos t` coefficient's
  length against the class circle's radius `ρ`;
- **Orientation** `‖b_r − β·(e × a_r)‖`, the `sin t` coefficient against
  the quarter turn of the `cos t` one.

Sites: `Derivation::CylinderMoving { beta: Positive | Negative }`, the
cone rim's and the sphere parallel's moving arms, and the torus
parallel's (`Radius` + `Orientation` side by side in each), and very
likely `SphereMeridian { pole: false }` (`norm_gap(R, R², a)` +
`‖σ·(e × a) − b‖`), whose pair has the same shape.

Both read the same pair's deviation from the class's circle
`ρ·(â·cos t + β·(e × â)·sin t)`, and the second re-reads the first's
error: an ellipse about a cylinder's axis with semi-axes `ρ ± δ`
stands at most `δ` off the wall (radially), but reads Radius `δ` plus
Orientation `‖(ρ − δ)·ŷ − (ρ + δ)·ŷ‖ = 2δ`, a lever of **3**. The
reviewer's probe refused that ellipse from `δ = 0.4·zero` (sampled
residual `0.4·zero`, envelope `1.2·zero`); `0.3·zero` certifies. That
is a worse lever than the `√2`/`2` of the issue the PR closed.

## Direction

Read the pair's deviation from the class circle as one harmonic pair,
`harmonic_sup(a_r − ρ·â, b_r − β·ρ·(e × â))` (`crate::harmonic`),
in place of the two slots, and restate the lemma text on
`EnvelopeStatement::MapResidualClosedForm` for it. Where `â` is not
well defined (`‖a_r‖` in the band) the class is a winding-Zero one and
reads its drift instead. A row: the `ρ ± δ` ellipse at
`δ ∈ (zero/3, zero)`, certifying at `f64` and `Interval`.

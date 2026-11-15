---
id: blend-contact-carriers-re-derive-the-directions-their-plan-held
kind: issue
title: A blend's contact carriers re-derive from their placed feet the directions the plan already held: a chamfer's support trim, a corner or seam arc's u_ref, a band's rim phase
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by BOXFINISH's sweep for its unit `boxfinish/sweep-frames` (the sweep doors keep the frames they are handed; `work/boxfinish/the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box.md`). A survey of the shape "a stored direction re-derived from points the construction placed", not measured over a box.

**Sites** (as of main `2c843a2e`):

- `crates/sweep/src/blend/surgery.rs` `attach_contact`, `ContactCarrier::Chord`: `Curve3::line(p0, (p1 - p0) / len)` is the stored carrier of a chamfer's support trim (`open/planar.rs` `blank_phase` picks `Chord` for `BlendKind::Chamfer`). The plan held the strip's trimline (`arms.rs` `chamfer_strip`, `geom::Line { origin: foot_a, dir: tau }`); a fillet in the same place passes `ContactCarrier::TrimLine`.
- `attach_contact`'s arc arm: `u_ref = flat(p0)`, `(p0 − center)` normalized, for `CornerArc` (the feet are `c + r·n_i`; the support normals `n_i` were held, `corner_ball`), `SeamArc` and `Section`; and with no plane, the axis is `(u × w).normalize()`.
- `surgery.rs` `rim_phase`: `radial = (fp − ca) / sa`, stored as the torus `u_ref`, `fp` a point of the scaled trim circle whose frame radial at `t0` was held; `rim_phase_annulus` the same off the host circle.
- Scaffolds a restate replaces, same shape: `junction.rs` `MixedTrim::spec` (`line_between` over a held line), `open/ruled.rs` `ruled_phase`, `open/planar.rs` `blank_phase` (`None => line_between`), `junction.rs` `strut_foot`, `open/planar.rs` `overrun_cut` (unconfirmed).

**Why it matters.** Over a parameter box the quotient carries the feet's width into a direction the plan held exactly, and tier 3 then reads that width as tilt or length (the class BOXFINISH measured on the extrude: a cap normal read `±0.2` of tilt it does not have).

**Repair shape.** Pass the held direction to the carrier (the trimline, the support normal, the frame radial) instead of re-deriving it from the feet, as `TrimLine` already does for fillets.

- 2026-11-15 — **Sites this row omitted** (the review of PR 6972, reviewer r1; read at `boxfinish/sweep-frames` after merging main at `dbb21b4a`, line numbers as of then). Each mints an edge by `EdgeCurveSpec::line_between` over two points the blend placed, so its direction is the points' difference over its length:
  - `crates/sweep/src/blend/surgery.rs:4915` (`line_between(pf, pv)`), `:4536` (`line_between(fp_i, fp_j)`), `:4596` (`line_between(p1, p2)`), the last two a `MefSite::Chords` split;
  - `crates/sweep/src/blend/junction.rs:720` (`line_between(merged.start, merged.end)`), a merged line restated from its two ends;
  - `crates/sweep/src/blend/open/end_face.rs:1262` and `open/planar.rs:1187`, `:1343`, `:1763`, each a `MefSite::Chords` chord.

  Most are scaffold chords that cut a face and are killed or restated before rest, as `junction.rs`'s `MixedTrim::spec` above; whether each survives to rest, and so whether its direction is read at rest, is the first thing to settle per site.


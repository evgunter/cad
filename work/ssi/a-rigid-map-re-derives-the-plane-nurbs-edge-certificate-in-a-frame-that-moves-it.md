---
id: a-rigid-map-re-derives-the-plane-nurbs-edge-certificate-in-a-frame-that-moves-it
kind: issue
title: transform_rigid re-derives the plane x NURBS edge certificate in the new frame, and its componentwise box norms move under rotation, so an edge certified near its band can refuse after a rigid map
status: open
opened: 2026-09-28
priority: P3
cost: M
pr: 3668
branch: ssi/probes
design: true
---

## Found (ENCL rigid-map headroom lane, branch `encl/rigid-map-approx-headroom`, 2026-09-28)

The sweep for siblings of
`work/encl/a-rotation-can-refuse-an-approx-face-that-certifies-near-eps.md`
— a rigid map that re-derives a hull-assembled bound in the new frame
and refuses on it — met this one. **Inferred from the construction,
not reproduced.**

`topo::transform_rigid` re-certifies every mapped edge carrier
through `EdgeCurve::certify_via` (`crates/topo/src/transform.rs`). An `Intersection` edge between a plane and a
described NURBS wall certifies through the injected lane,
`geom_brep::plane_nurbs_limbs` (`crates/geom-brep/src/edge_nurbs.rs`),
which delegates to the rung-3 SSI certificate. Parts of that
certificate read a vector's size off its componentwise enclosure box —
`EncloseBox::speed_sup` (`crates/geom-brep/src/ssi/enclose.rs`), the
chart-floor rate, the tube pad, the transverse stretch — and a box norm
of the same vector set can differ by up to √3 between two frames. The
edge's `hull_sup` limb is a Bernstein composite over `S(P(t)) − C(t)`,
which is at least as frame-sensitive in its enclosure width (the
`Approx` sibling's trace shows the width, not the norm, moving).

The transform module's own docs claim the opposite: "a rigid map
preserves every distance-valued residual up to rounding, so
re-certification of a valid body succeeds". That holds for sampled
residuals and implicit-form scalars, not for a bound assembled from
ambient-frame hulls.

## What is open

1. Reproduce: a plane × NURBS edge certified within a few percent of
   its band, moved by an oblique rotation through
   `transform_rigid` (which reads the lane off `f64`'s policy). The `Approx` row's
   scale-to-land-near-ε construction
   (`crates/topo/tests/rigid_map_near_eps_approx.rs`) is the template.
2. If it reproduces, the remedy is this lane's to choose. The `Approx`
   face could be re-fitted because its fit is derived from a
   description; an edge's declared carrier is not re-derivable the same
   way, so re-minting is not obviously available here.

## Measured (ssi-probe lane, branch `ssi/probes`, 2026-10-01)

**Reproduced.** `crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`
pins it, `#[ignore]`d until a remedy is chosen:
`cargo test -p topo --test all rigid_map_near_eps_plane_nurbs -- --ignored --nocapture`.

The subject: the `z = 1/2` plane against the rational quarter-cylinder
wall, carrier the exact rational arc of radius `1 + δ` (a residual field
of exactly `δ`, radial), seated at ±45° about the x axis, on a two-face
lamina whose two edges both run the arc, minted through
`set_edge_curve_nurbs_lane`. `δ = 0.783 ε` puts limb 2 at `0.9705 ε`
(default ε). Through `transform_rigid` at `f64`, the
32 rotations of the `Approx` row's sweep: **24 refuse**, all on limb 2
(`ssi_hull_sup_chart` in-band at `1.0006 … 1.1479 ε`, surfaced as
`CertifyError::Escalated { check: PlaneNurbsHull }`); the 8
rotations about x, which keep the arc's chord on an axis, move it.

**Which term moves.** At a loose band, every image certifies and its
limbs read:

| term | seated | across the 32 maps |
|---|---|---|
| limb 1, `on_locus_max` | 0.78281 ε | unchanged (7 digits) |
| tube radius, transversality, boxes | 0.125, 0.8824661, 32 | unchanged (7 digits) |
| `min_sin_theta` | 1.000000 | unchanged |
| limb 2, `hull_sup` | 0.97053 ε | up to 1.14790 ε (**×1.183**) |

Limb 2 decomposes into the fold and the enclosure width. The bound is
`tensor::SurfaceResidual::sup_bound`: the three whole-domain
per-coordinate sups, folded Euclidean. For this field that fold is
`√(1 + ½)·δ = 1.2247 δ` seated and up to `√2·δ = 1.4140 δ` under a
quarter-turn-ish map (**×1.155**). The bound over the fold of the true
field, which is the enclosure-width overshoot, goes from 1.0123 to
1.0371 (**×1.024**). So the fold accounts for most of the drift and the
enclosure width for the rest. `EncloseBox::speed_sup` and the tube pad
do not move anything measurable here.

The same composite's per-span bounds (`span_bounds()`), each folded
Euclidean and then maxed over the spans, read 0.81468 ε seated and
0.81301 … 0.82159 ε across the maps: **a drift of ×1.0085, and 16 %
tighter seated**. That reading was taken with the unlocalized wall,
whose whole-domain number matches the lane's to 5 digits.

**Not a near-ε corner.** An edge certified anywhere above `ε/1.183 ≈
0.845 ε` refuses some rotation of this fixture. The fold also reaches up
to `√3·|r|` in the worst frame, so a field certified at
`0.58 ε` of true sup norm can already refuse at its mint, in an
unlucky frame.

**Options (not implemented; the remedy is this row's to choose):**

1. *Fold per span, at the call site.* `nurbs_limbs` reads
   `span_bounds()` and takes the max over spans of the per-span
   Euclidean fold instead of `sup_bound()`. It is still a certified
   bound and never looser. On this fixture the drift falls from ×1.183
   to ×1.0085, so it shrinks the problem and does not remove it. SSI
   ground, cost E, and it moves every plane × NURBS `hull_sup` reading
   down, so re-baseline what pins one. `sup_bound` has this one
   production caller, so the same change could be made in
   `geom_core::spline::compose::tensor` (NURBS/PROPS ground) instead.
2. *Certify in a canonical frame.* The lane maps its three operands
   into the plane's own frame (origin, `u_ref`, `n × u_ref`, `n`) before
   the composite, plus a pad for the rounding of that map. The
   certificate is then rigid-invariant up to that pad, which fixes both
   the fold and the enclosure term, at the cost of a rigorous pad
   argument. It still refuses within an ulp-scale band of ε, which is
   the honest residue.
3. *Carry the certificate across the map.* Accept the operand-frame
   certificate for an exact image. This contradicts the transform's
   "re-derive, never copy" posture (D4 ¶2, the PR #83 ruling), so it is
   Ev's.
4. *Re-mint.* Not available for a declared carrier. A marched carrier
   could be re-fitted, as the `Approx` face is, but this class's
   carriers come from a file.

(1) and (2) compose: (1) is cheap and shrinks the class now, and (2) is
what closes it. The transform module's doc claimed "a rigid map
preserves every distance-valued residual up to rounding, so
re-certification of a valid body succeeds". It now says which bounds
are frame-sensitive and points here. The same docs' two-class contract
("what can still refuse is authored VERDICT marginality") is
ratified text (the PR #83 ruling) and is not re-worded here. This class
is a third refusal it does not name.

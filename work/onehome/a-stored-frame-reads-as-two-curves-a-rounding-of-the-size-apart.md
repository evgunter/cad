---
id: a-stored-frame-reads-as-two-curves-a-rounding-of-the-size-apart
kind: issue
title: A conic's stored frame reads as two curves, the stored entries' and the polar factor's, a rounding of its size apart: a definite root right for one is past eps of the other
status: open
opened: 2026-11-21
priority: P2
cost: M
---



Found by `oneround/anchored-conic`'s probes for PR 7258 (2026-11-21), on
`oneround/anchored-conic` at `c3fb679d1`, stacked on
`onehome/carrier-frames` at `ec8749552`. Local debug runs with temporary
probes; the tables, the patch and the raw lines are on
`analysis/oneround/conic-vertex-probes` (never merged), under
`analysis/oneround/conic-vertex/exact-vertex-chord/`.

## Finding

A conic whose frame is not axis-aligned has two readings of "the curve
it is", and on a large conic they are far enough apart that a definite
answer right for one is wrong for the other by more than ε.

- **The stored entries**: the frame's `u_ref` and `axis` `f64` entries
  taken as reals. This is what the `f64` evaluator sums
  (`crates/geom/src/curves/conic.rs`) and what the exact oracle takes
  (`crates/geom-brep/src/curve_face/conic_oracle/exact.rs`, `truth`).
- **The polar factor**: the nearest rotation to the stored
  `(u_ref, v_ref, axis)`, which is how the frame is read when it is
  lifted to an `Exact` scalar (`RigidMotion::map_scalar`,
  `enclose_rotation`, `crates/geom-core/src/linalg/rigid_motion.rs`). It
  is about an ulp an entry from the stored entries, so `u·a` on a conic
  of size `a`.

Measured: the exact roots of a conic against a unit wall, near-tangent
(depths 0.5 to 2 µm), taken once with each reading of the directions
(the polar factor to first order in `MᵀM − I`), on the 200 poses a size
of the near-gap families
(`in_band_sweep::the_near_gap_families_certify_by_their_stored_form`'s,
at each size). The two sets of roots lie apart along the arc by at most:

| size (m) | apart at depths 0.5 to 2 µm | apart at depths 0.5 to 2 nm |
|---|---|---|
| 10 | 5.8e-11 | 5.9e-10 |
| 100 | 3.9e-11 | 1.3e-9 |
| 500 | 1.8e-10 | 5.8e-9 |
| 1000 | 3.7e-10 | 1.2e-8 |
| 1500 | 5.5e-10 | 1.7e-8 |
| 1e4 | 3.6e-9 | 1.1e-7 |

So at ε = 1e-9 the two part by more than ε from about 3 km at these
depths, and at ε = 1e-12 at every size.

A reading of the conic's point that is exact for the stored entries (a
chord from the seam vertex `centre + u_ref·a` held as an error-free
pair) certifies roots that are all within ε of the stored entries'
roots, and against the polar factor's roots are wrong on 50 of 200
poses at 1e4 m (up to 3.6e-9 m off), and on 4, 12 and 9 of the poses on
door-built circle edges at 3e3 and 1e4 m (a sketch arc's prism and its
plane section, whose seams are generic in the plane; none on a revolve's
axis-aligned edges). The centre sum as `main` reads it refuses those
poses under either reading and gives no wrong answer.

Both reviews of PR 7203 saw the same thing from the other side:
`Interval` enclosures of a lifted conic's point that exclude the exact
value of the stored formula on the stored `f64` vectors (48 of 5,184 in
one; 1 on the head and 33 on the base in the other), by up to 40% of the
box's width, at quarter and half turns.

## What it leaves open

Which of the two is the conic, at `f64` and at `Interval`: the one the
`f64` lane draws, or the one the certified lane encloses. Today a
verdict certified at `Interval` is about the polar factor's curve and a
point evaluated at `f64` is on the stored entries', and nothing states
that they are one curve only to `u·a`.

## Not established

- Whether any decision in the suite is answered differently by the two
  lanes for this reason. No row was found that is.
- The gap past first order in `MᵀM − I` (of order `u²`), and the
  `Interval` lane's own enclosure of the frame, which was not read.
- Surfaces: only conics were measured.

---
id: a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded
kind: issue
title: A wedge's vertical edge tilted 3e-8 off a cube's face leaves its far vertex 9.6e-9 from the face and a 3.7e-8 edge: the boolean neither records nor refuses the in-band vertex-face pair
status: open
opened: 2026-10-08
priority: P0
cost: M
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by this program's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

On the 345° wedge, with the cube's face plane tilted 3e-8 off the
wedge's vertical edge (`e2`), the result holds a vertex 9.55e-9 from a
face. The vertex projects inside the face. The result also holds an edge
3.7e-8 long. Neither is recorded as a contact, and the boolean did not
refuse.

- **ε = 1e-9:** `w345 nt e2 a6 d-3e-8` and `w345 nt e2 a14 d3e-8`. All
  six ops ship BAD, failing tier 3′ only. On ∪ and ∖ the census escalates
  at the same entities on five predicates:
  - `pm_census_vf_residual` 9.55e-9 (vertex `24v1`, face `14v1`);
  - `pm_census_ve_line_gap` 9.55e-9;
  - `pm_census_ef_residual` at three edges' ends on that vertex;
  - `pm_census_ee_parallel` on the 3.7e-8 edge (sin θ 0.26);
  - `pm_census_ee_gap`.

  Each is exact at 60 digits, and each is a real in-band distance.
- **ε = 1e-12:** `w60 nt e2 a12 d1e-11` and `w60 nt e2 a4 d-1e-11`, the
  same shape (51 escalations).
- **Not at the neighbouring tilts.** At ±1e-8 every op on these poses
  refuses `Escalated` at the boolean. At ±1e-7 they build clean.

The body is at the oracle volume, with t2, certificate and operand
passing. The pair is a vertex-on-face coincidence the boolean neither
glued nor refused.

**The stage.** The sweep's vertex-against-face reading (the split) left
the vertex off the face, while its distance is inside the band.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row.

## Owed

Find the reading that put the vertex off the face: which margin it read
and at what lever. Then either record the coincidence or refuse it
typed. This is not REST or declared contact (D10 hold): it is an
undeclared in-band vertex the boolean made itself.

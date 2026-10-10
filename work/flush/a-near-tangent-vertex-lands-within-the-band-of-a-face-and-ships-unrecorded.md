---
id: a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded
kind: issue
title: A wedge's vertical edge tilted 3e-8 off a cube's face leaves its far vertex 9.6e-9 from the face and a 3.7e-8 edge: the boolean neither records nor refuses the in-band vertex-face pair
status: parked
opened: 2026-10-08
priority: P0
cost: M
refs: [4335]
blocked_on: [boolean-door-runs-the-census-over-its-result]
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

## Measured (JOIN lane, branch `join/near-tangent-vertex-in-band`)

Main `681f0385`, release. Every decision the boolean took was logged
through the `classify` funnel (`geom_core::k_stats`), with name, sign and
margin, by a temporary local patch that was not committed. The poses are
the four named above at their own ε.

**The pair.** `w345 nt e2 a6 d-3e-8 pc U`:
- vertex `24v1` = (3.565e-8, −9.552e-9, 0). It is where the cube's face
  pierces the wedge's 345° bottom edge, 3.6907e-8 from the corner.
- face `14v1` is the wedge's own 0° side face (y = 0). It touches that
  edge only at the corner.
- So the vertex and the face belong to **one operand**. Their gap is
  r·sin 15° = 3.6907e-8 × 0.2588 = 9.552e-9, the census's margin to the
  last digit.
- The 3.7e-8 edge is the corner-to-`24v1` piece of the same edge.
- At ε = 1e-12 the w60 poses have the same shape: the pierce is
  1.005e-11 along one bottom edge, so the gap is 1.005e-11 × sin 60° =
  8.70e-12 to the other side face.

**What the split read, all definite** (band 1e-9 / 1e-8; the smallest
non-noise margins in each run):

| reading | margin | what it is |
|---|---|---|
| `bool_vertex_face_side` | 3e-8 (3 Kε) | the corner against the cube's plane |
| `split_edge_param_interior` | 3.691e-8, 4.729e-8 | the two pierces' distance from the corner, along their edges |
| `bool_join_chord` / `bool_strut_order` / `bool_contact_vertex` | 1.506e-8 | the two pierce points' separation |
| `dihedral_arm` / `dihedral_wedge` | 3.691e-8 | the new short edge's length, at a 90° dihedral |

- No reading in the split took 9.55e-9. No reading compares `24v1`
  with face `14v1`: the edge-face sweep reads one operand's edges
  against the other operand's faces only.
- The only escalations inside the four builds are retrying ladders, and
  none of them is the split. They are the shell-role ray cast
  (`point_in_solid` → `point_in_loop_boundary`, including 1.224e-8 =
  4.729e-8·sin 15°, the mirror pair, which reads definite) and the
  validator's winding fallback (`bool_ring_run_winding`).
- At ε = 1e-12, `bool_vertex_face_side` reads the corner at exactly
  1e-11 = K·ε, which is definite at the band's edge. The pierces read
  1.005e-11 and 2.423e-11.

**The neighbours.**
- At d = ±1e-8 the boolean refuses on the corner's own reading:
  `Coincidence(VertexOnFace)`, margin 9.9999e-9, which is the corner
  against the cube's plane.
- At ±1e-7 the pierce lands about 1.2e-7 along the edge, so the composed
  gap is about 3e-8, which is definite.

So the tilts refuse or build by the corner's distance, and the in-band
pair at ±3e-8 is the product of a definite pierce distance and the
operand's own 15° (60°) corner angle. Neither factor is in band.

**Class (b), a composed quantity.** Every reading the split took was
definite. The in-band vertex-face pair arises only from their
composition with the operand's fixed geometry. This is designer A's "a
new vertex near a far face" in PR 4335 (here the face is the same
operand's, adjacent to the edge at the corner). It is not the premise of
that PR's "JOIN: the w345 reading … fix it at that decision". There is
no in-band reading to fix. Detecting the pair is the result-side
question PR 4335 puts to Ev.

Repro: patch `classify_in` to `eprintln!` name, sign and margin, then
`NT_ONLY=w345 NT_POSE="e2 a6 " NT_D=-3e-8 cargo run -p sweep --release --example near_tangent_census_probe`
(and `CAD_TOLERANCE_EPS=1e-12 NT_ONLY=w60 NT_POSE="e2 a12 " NT_D=1e-11`).

## More evidence (door-typing unit, branch `join/door-types-in-band-results`)

The census clause's one number reads this pose in band at the edge pair
too. `near_tangent_census_classify.py`'s far-end gap gives six
`ee_parallel` pairs at `w345 nt e2 a6 d−3e-8` and `a14 d3e-8` (ε = 1e-9).
Each is two edges sharing the pierce vertex at 15°, the shorter 3.69e-8
long, with far-end gap 9.552e-9, in band. It is the only shared-point
pair in the near-tangent probe whose gap is not definite.

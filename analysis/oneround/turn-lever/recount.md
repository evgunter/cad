# The unfolded recount: the covers against the points they must hold

G2's line-site door (`g2-measure.patch`, `crates/topo/src/wedge_reach.rs`)
folds the bound from below into the bound from above:

    covers.iter().fold(far(own).max(lo), |m, c| m.max(far(*c)))

So two of G2's results hold by construction and say nothing about the
covers: "above < below: 0" and "no wrong-direction pair". The middle-site
door on the branch folds in the same way (`wedge_reach.rs:164-169`).

This recount reads, at every levered read of the line site on the f64 lane:
- **raw**: the covers' own far distance from the line, nothing folded in;
- **below**: the vertices plus (ii)'s edge points;
- **vertices**: the vertices alone;
- **truth**, where one exists: the boundary maximum, on plane faces and on
  cylinder and cone walls with the apex guard.

It is taken at 1e-9 on core (topo, step-import) and sweep, with config A's
semantics. `recount.patch` is the measurement tree as it ran.

## By class

Each pair is (past 1e-12 relative / past ε).

| class | run | reads | raw < below | raw < truth | raw < vertices |
|---|---|---|---|---|---|
| plane, straight edges | core | 2,422,531 | 160 / 0 | 160 / 0 | 0 / 0 |
| | sweep | 1,768,879 | 0 / 0 | 0 / 0 | 0 / 0 |
| plane, straight, inexact | core | 484,564 | 6 / 0 | – | 0 / 0 |
| | sweep | 202,978 | 0 / 0 | – | 0 / 0 |
| plane, arcs | core | 222,780 | 10 / 0 | 10 / 0 | 0 / 0 |
| | sweep | 554,854 | 0 / 0 | 0 / 0 | 0 / 0 |
| plane, arcs, inexact | core | 1,237 | 0 / 0 | – | 0 / 0 |
| | sweep | 53,102 | 15 / 0 | – | 0 / 0 |
| cylinder/cone wall | core | 10,737 | 108 / 108 | 108 / 108 | 0 / 0 |
| | sweep | 539,124 | 5,476 / 5,246 | 5,476 / 5,246 | 526 / 342 |
| wall, inexact | core | 4,925 | 21 / 21 | – | 0 / 0 |
| | sweep | 85,060 | 117 / 82 | – | 4 / 4 |
| other | core | 402,314 (395,415 poison) | 0 / 0 | – | 0 / 0 |
| | sweep | 257,976 (250,748 poison) | 69 / 67 | – | 1 / 1 |

On plane faces bounded by straight edges the truth is the farthest vertex,
so there only the cover against the vertices is evidence: 0 in both runs.
The 160 relative hits on core are against carrier ends that are not
vertices, and none is past ε.

## By door, past ε

Details are sampled, at most 5 per door and class per test. Every test with
a read past ε is represented in them (179 on sweep).

| door | what it reads | samples (core + sweep) |
|---|---|---|
| `euler.rs:3988` `certify_rebased_run` | a rebased run before its move lands | 10 + 494 |
| `euler.rs:3733` `certify_edge_spec` | an edge certified onto a face still carrying its seed surface (import's adopt; blend surgery through Euler ops) | 77 + 98 |
| `attach.rs:365` `set_face_surfaces_certifying` | offset and shell: the new surface is set while the boundary sits the offset distance (0.02, 0.05) off it | 0 + 83 |
| `boolean/combine.rs:968` the graft | the boolean's working source body | 0 + 20 |

**In every sampled read past ε, a face's boundary is off its own surface**,
by 0.02 to 2. For walls this is measured as the boundary's distance from the
cylinder, or from the double cone. Of 725 wall reads past ε, 0 have every
wall face's boundary on its surface. The cover reads the surface the face
carries. What has not yet happened is the move that brings the boundary
onto it.

**The graft, settled per face.** In `a_box_along_a_fillets_line_arc_junction`
the vertex at 0.539 from the line belongs to face 12, whose generation
climbs from v3 to v7 during the run. Its boundary is 0.16 to 0.56 off its
r = 0.2 cylinder, and its cover reads that cylinder at 0.40. The partner
face is on its surface, and its vertices and cover agree at 0.283. Face 12
is a working face of the boolean. The finished result passes the test's
tier 3.

**At-rest doors are relative-only.** These are tier 3's `validate.rs:7900`,
`boolean/ops.rs:3282` and `attach.rs:1479`. The largest shortfall there is
5e-10, which equals that boundary's own distance off its surface: a
boundary within tolerance of its surface.

## Verdicts at inverted reads

An inverted read is one whose unfolded cover is short of the bound from
below by more than ε.

- On core, none.
- On sweep: `tangent_second_order`, 560 reads at the graft's door, all
  served Positive; `classify_dihedral`, 4 at `certify_rebased_run`, served
  Transverse.
- A Zero stood in none of them. Each is a definite sign from below, which
  stands under the design's rule.

The first core pass keyed inverted reads by the value of the bound from
below. It collided on common values and produced 54 spurious plane-plane
`Crease` entries. The sweep pass clears the mark when a later read of the
same value is not inverted.

## The design's rule, run

`TL_DESIGN=1`: an inversion past ε is no bound from above. Definite signs
stand, and Zeros are refused.

On core at 1e-9:
- **test outcomes changed against the recount run: 0 of 4,799**;
- levered verdicts refused: 90 in both runs;
- straddles: 7 in both, identical by decision;
- moves against the middle: 34 in both.

The design run made 66 fewer verdicts of 1.8 M. Every difference is a
served `classify_dihedral` in `census::link_fuzz` and
`census::pierce_fuzz::tilted`, which draw their bodies from a fresh seed on
each run.

## The `Dual` lane's NaN derivative

`gram_top` (`crates/geom/src/cover.rs:482-485`) takes `sqrt(0)` on every
unit orthogonal frame. `Dual::sqrt` gives that a derivative of
`0 / (0 + 0)`. So every curved piece's far distance carries a NaN
derivative with a finite value; the probe is `tl_dual_cover_probe.rs`.

`Decide for Dual` reads the value channel alone (`dual.rs:887-893`, E9), so
no decision reads the NaN. Across G2 config A there are 120 k Dual verdicts:
0 refused, 0 straddles, 0 moved.

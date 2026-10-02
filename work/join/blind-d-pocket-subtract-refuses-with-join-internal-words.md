---
id: blind-d-pocket-subtract-refuses-with-join-internal-words
kind: issue
title: boolean: a blind D-profile pocket (block minus a D rod) refuses JoinDesync from the top face — an internal-desync payload for an ordinary pocket
status: open
opened: 2026-09-25
priority: P1
cost: H
---


## Measured

`Tol::witness()`, public doors. The block `[−1, 1]² × [0, 1]` (one
extruded square) minus a D-profile rod extruded for `1.0` from
`z = z0` — the D is `sweep::test_support::rod_chord_at(0.3)`'s: chord
`x = 0.3`, major arc of radius 0.5 about the origin (the D-rod's own
profile, `review_fillet_h7_r1_probes::d_rod`).

| tool | `z0` | `topo::subtract` |
|---|---|---|
| D rod | `0.5` (pocket from the TOP face) | `JoinDesync { what: "ring-run winding is degenerate (zero enclosed area)" }` |
| D rod | `−0.5` (pocket from the BOTTOM face) | `Join(SectionInvariant { what: "the all-planar join lane reached a conic run edge (the operand gate promises every carrier planar)" })` |
| disc r = 0.5 | either | builds |
| square 0.6 | either | builds |

So a blind round pocket and a blind square pocket build, and a blind D
pocket — the two combined — refuses from both sides, each time in an
internal payload's words rather than a frontier's. The top-face payload
is `crates/topo/src/boolean/join.rs`'s ring-run winding decision
(`bool_ring_run_winding` deciding `Zero`); the bottom-face payload is the
same site `work/contact/axis-coincident-lap-trips-the-planar-join-invariant`
carries for a different pose (evidence added there).

## Diagnosed (2026-10-02, JOIN, branch `join/ring-run-winding`)

Re-measured on that branch: the BOTTOM pose (`z0 = −0.5`) now builds,
at `3.6632128205514776` — exactly `4 − 0.5·A_D`
(`A_D = πr² − (r²·acos(0.6) − 0.3·0.4)`). The TOP pose still refuses
with the same payload. Its cause is not the ring-run winding sum; that
sum is right about the run it is handed, and the run is the wrong
question.

**The state at the refusal.** Solid A (the block), top face `z = 1`.
The D's section polygon there is a 2-gon: the flat side (germ: top ×
the rod's flat face, a line) and the arc side (germ: top × the rod's
cylinder, a circle), meeting at the two pierce sites `(0.3, ±0.4, 1)`.
The flat side has already joined, minting its two straight copies, so
the top face carries the ring
`[line (0.3, 0.4)→(0.3, −0.4), null, line (0.3, −0.4)→(0.3, 0.4), null]`
and the flat side's sliver face carries the mates. The arc side's match
then hands `choose_roles` the ring's two null halves; both candidate
runs are `[null, one straight copy, null]`, closed by `ring_run_ccw`'s
STRAIGHT chord back along the same line — zero area in either order.

**Two defects, both in how the join treats a straight edge on a curved
germ's lane (`JoinLane::BoolPlanar`):**

1. `chord_join.rs`, `between_edge_in_plane`: a `Line` between edge
   answers "in plane, it IS the section segment" with no evaluation.
   On the plane×plane lane that is right (both ends on the partner
   plane ⇒ the segment is in it). On `BoolPlanar` the section segment
   is an arc of the partner wall, and a chord of that circle is not
   it — so both adjacency guards would SKIP the arc chord, and the arc
   side would never be minted on A. Measured: forcing either role
   order past the winding refusal gives
   `ZipCorrespondence { "seam cycles differ in length" }` (A lacks the
   arc B has).
2. `boolean/join.rs`, `ring_run_ccw`: the run is closed by the straight
   chord `end(h2) → p₀`, but the chord the joiner's first `mef` mints
   on `BoolPlanar` is a CONIC (`chord_spec`'s wall arc), so the region
   the mef walls off is bounded by the arc. With the arc as the
   closure, `[null, line up x = 0.3, null]` + the major arc back round
   the left is the D, counterclockwise about `+z`; the other order is
   the D clockwise — exactly one order is CCW again.

Measured with both patched by hand (probe only, not landed): `Line`
under `BoolPlanar` answering "not in plane", plus the role order
`(ea, ra)` — the one the arc closure picks — builds the top pose at
`3.6632128205514776`, exactly; the other order refuses
`JoinDesync { "minted-edge description failed certification" }`.

**Why it was not fixed in that PR.** (1) needs a decided statement of
when a straight edge is a curved wall's section segment (a plane
parallel to a cylinder's axis meets it in rulings, so "never" is not
the rule; a wall-membership test on the line's midpoint, with its own
predicate name, is the natural one); (2) needs the joiner's chord
(`chord_spec` over the match's lane) available where `choose_roles`
decides, and the run winding to take a conic closing chord's bulge —
`Body::planar_run_winding_decided`'s `Closing::Chord` is the place.
Both sit in shared join ground (`chord_join.rs` is REACH/TANG's too).

## Why it matters

A D-shaped blind pocket is an ordinary feature, and the ruled fillet
carves its concave creases as of `band/ruled-d-hole-ring-crease` (the
top end of each crease in the top cap's ring, the bottom end in the
pocket floor's outer cycle) — but nothing can build the body to fillet:
the extrude door makes only the THROUGH-hole, and the boolean refuses
the blind one.

## Found by

`band/ruled-d-hole-ring-crease`'s class sweep (a mixed crease, one end
in a ring and one in an outer cycle), probing for a fixture.

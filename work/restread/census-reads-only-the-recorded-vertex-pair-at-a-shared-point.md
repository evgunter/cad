---
id: census-reads-only-the-recorded-vertex-pair-at-a-shared-point
kind: issue
title: At a point several vertices share, the census reads only the vertex pair a record names, so a rest there can clear an unrecorded cross-solid pair on the same key that crosses
status: closed
opened: 2026-10-18
priority: P1
cost: M
rides_with: a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap
closed: 2026-10-19
---


Found by the two designers weighing
`a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap`
(2026-10-18); both confirmed it against the code.

`census::pair_vertex_vertex` (`crates/topo/src/census.rs`) emits a
finding only when the two vertices are *not* on one point key
(`!geo.same_point(ka, kb)`) and the pair is unrecorded. The pair-level
clear (`blocks`) reads standing findings and the pairs records name
(`recorded` → `declared.vv`, `declared.vf`). So a cross-solid vertex
pair that shares a point key and carries no record is never read by the
touch analysis.

Shape: one solid with two cones at `v` (two cubes united corner to
corner), and a third solid's corner recorded against one cone. The rest
read at the recorded pair clears it, while the unrecorded pair, which
may cross the other cone, is never read. The boolean does not build that
body, but tier 3′ reads the body alone, so an import or a surgery can
present it (likely; not built). Unchecked: whether solids meeting *only*
at an unrecorded shared-key pair leave the point unread altogether
(trace `meets_found` and the material probe).

A false clear at tier 3′, hence P1. The fix is the point-level unit the
pinch fork is deciding (each solid's whole link at a point, every cycle
read, recorded or not). Whichever answer is taken there must close this;
until then it stands on its own.

## Closed

2026-10-19, with the carrier row (branch `restread/link-touch-reading`,
after its review's fix pass).
- A recorded pair now reads its point's unit: both solids' whole links
  there, every vertex of each on the point's keys. A face held at two
  vertices on the point (a pinch, one vertex per cone) gives a sector per
  pass, each ending where the loop next returns to the point.
- `blocks` reads every key two solids share whether or not a finding or
  record names it, strictly: an undecided reading at a shared key
  refuses, and only a declared-only pair's own records are still taken
  on their word (CONTACTHOLD's D10 ground, not extended).
- Witnesses:
  - `census::tests::a_rest_recorded_at_one_cone_of_a_pinch_reads_the_other`:
    one solid with two cones at the origin on one key, a parallelepiped's
    corner on that key recorded against the first cone and reaching into
    the second's octant. Main reads that touch Rest (measured at the
    verdict level); the branch reads Crossing, and the same corner in
    the empty octants rests. The body is read at the touch: built as two
    shells of one solid, tier 3 refuses it first, so the end-to-end
    reading of the shape is the boolean pinches of
    `join_pierce_runs_sweep::four_pairs_battery`.
  - `census::tests::a_shared_key_only_pair_reads_strictly`: two solids
    meeting only at an unrecorded shared key, read in band, refuse.
  - `census::link_fuzz::a_pinch_link_agrees_with_the_prisms_own_inequalities`
    and the falsifier's bowtie family: two-cone links against the exact
    oracle.

---
id: a-spheroid-face-tessellates-within-its-own-sizing-theorem
kind: unit
title: A spheroid face tessellates on the sphere's chart and pole fans, sized by its own pinned worst-triangle theorem
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: an-ovoid-cannot-be-authored
blocked_on: [the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis]
---



Unit 6 of `an-ovoid-cannot-be-authored` (Ev, PR 7289). The designers'
bar: "tessellates within its theorem"; their door list: "the sphere's
chart and pole fans, with the lane's own sizing theorem under
`[mesh.sizing]`". The rule is `crates/mesh/README.md` `[mesh.sizing]`
(ratified #5708): each lane meshes as coarse as its own pinned
worst-triangle theorem allows, with no safety factor.

## What is true when it is done

- `mesh`'s chart table (`crates/mesh/src/chart_table.rs`, `Analytic`)
  has a spheroid arm on the sphere's chart and pole fans; `walk.rs`,
  `trimmed.rs`, `memo.rs` and `chords.rs` (the spindle's other mesh
  sites) handle it.
- A spheroid lane's worst-triangle theorem is stated in
  `crates/mesh/README.md` beside the sphere's and pinned by a test that
  breaks if the bound is loosened or tightened past its proof: the
  deviation of a chord-sized triangle from the surface, in terms of
  the principal curvatures, whose extremes on a spheroid are at the
  poles and the equator (`c/a²` and `a/c²` for the meridian,
  `1/a` and `c/a²` across it, prolate and oblate swapped).
- Every body unit 5 builds tessellates watertight (`mesh::validate::check_mesh`),
  with signed volume inside its certified bound of `4/3·π·a²·c` for the
  full spheroids, and inside the theorem's deviation everywhere.

## Check first

That the sphere's pole fan is sized by a reading that stays true when
the meridian's curvature at the pole is not the equator's. On a prolate
spheroid the pole is the most curved point; if the sphere lane's fan
assumes one curvature everywhere, the spheroid's fan needs its own
sizing, which is this unit's to write. If the theorem cannot be pinned
without a constant the README does not class, **stop and report**.

## Rows and ground

- Changes no other row.
- Crosses TESSLANE (`chart_table.rs`) and CHORD. Announce on TESSLANE's
  log.
- Runs beside units 7, 8 and 9.
- Review: **single (FULL)**: the theorem's proof is the review.
  Difficulty **M**.

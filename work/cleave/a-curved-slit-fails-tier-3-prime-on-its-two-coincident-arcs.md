---
id: a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs
kind: issue
title: A curved keel's slit or ring fails tier 3′: its two coincident arcs sit on two curve entries and no record backs the overlap
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
---


Found by `cleave/keel-slit-ring`, which builds the split's zero-area
ring (`work/cleave/a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section.md`).

## What

A split along a curved keel in the plane gives the side round it two
coincident arcs, floor | wall a and floor | wall b: a slit of the
section's outline where the keel is open at one end (PR 4622), a
zero-area ring where both ends are wrapped. The two arcs are on two
curve entries. Tier 3′ with no records refuses that side with one
`UnrecordedContact { EdgeEdgeOverlap }` naming the two arcs: census pass
5′ (`crates/topo/src/census.rs`, `sweep_curved_edge_edge`) owes a curved
overlap its edge-edge record, and the split returns none. Measured at
the default ε:

- the ring: `crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
  `a_curved_keel_closed_at_both_ends_splits_with_its_keel_a_ring`,
  pins the refusal, both normals, the V's tip at 0 and `±0.6 ε`;
- the one-end slit on main: every row of
  `a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`
  refuses the same way on the walls' side.

Straight slits and rings pass: two collinear edges whose bounds are one
point key each are one line by the census's reconstruction rule
(`census.rs` module docs, the D3 rule). Two arcs between two shared
points need not be one arc. The split's door gates at tier 3, so it
returns these sides.

## Owed

One of:
- **One curve entry.** The two arcs share one curve entry (tier 1
  bars sharing for null-scaffold curves only, pass 13), and the census
  reads a curved pair on one entry bounded by shared point keys as
  structural. That extends `crates/topo/README.md`'s "Structural sharing
  (same surface or point key) needs no record" to curve keys, which is
  Ev's.
- **A record.** The split returns its own contact records for the
  pinches it builds.

Neighbour: `work/contacthold/two-coincident-curved-edges-touching-record-no-edge-edge-row.md`
(the boolean's curved edge-edge rows).

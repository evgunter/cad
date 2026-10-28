---
id: a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs
kind: issue
title: A curved keel's slit or ring fails tier 3′: its two coincident arcs sit on two curve entries and no record backs the overlap
status: open
opened: 2026-10-19
priority: P1
cost: M
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

- the ring, on PR 5350's first head: both normals, the V's tip at 0
  and `±0.6 ε`. The split now refuses that trace typed instead
  (`SplitJoinError::CurvedSlitTrace`, pinned by
  `crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
  `a_curved_keel_closed_at_both_ends_refuses_as_a_construction_gap`), so
  the ring form builds only once this row is fixed;
- the one-end slit on main: every row of
  `a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`
  refuses the same way on the walls' side.

- the Boolean's twin, on `cleave/boolean-builds-the-slit-form`'s first
  head: a 270° annular slab about `y` less a 150° revolved V whose tip
  rests on its floor, turned 60° (a ring) and −20° (a slit of the
  floor's outline), built both forms at the closed-form volume, tiers
  1, 2 and 3 passing, and 3′ refused one `UnrecordedContact {
  EdgeEdgeOverlap }` naming the two arcs. The Boolean now refuses
  `EdgeInFace { gap: Curved }` before parting instead, with the split's
  words (pinned by
  `crates/sweep/tests/a_curved_keel_on_the_floor_refuses_its_slit_form.rs`).

Straight slits and rings pass: two collinear edges whose bounds are one
point key each are one line by the census's reconstruction rule
(`census.rs` module docs, the D3 rule). Two arcs between two shared
points need not be one arc. The split's door gates at tier 3, so it
returns the one-end slit's sides
(`the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`).

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

Once this lands, lift `CurvedSlitTrace` from `Sweep::section_kind`, and
`EdgeInFaceGap::Curved` from `topo::boolean::slit::part_riders`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: straight slits pass tier 3′ by the reconstruction rule and curved ones have no record: a special case that should be handled uniformly, P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).

## Decided (2026-10-28, Ev on PR 5967)

The slit's two coincident arcs are two cells, each with its own face pair's description, so one curve entry is not an option ([edges], tier 3 check 4). The split records them as an edge–edge pair, citing a new `SplitOn` row (the operand edge ON the plane). The census confirms the record as a curve touch. `CurvedSlitTrace` and `EdgeInFaceGap::Curved` come out once the record lands.

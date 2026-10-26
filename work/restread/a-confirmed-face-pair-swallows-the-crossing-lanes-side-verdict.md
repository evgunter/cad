---
id: a-confirmed-face-pair-swallows-the-crossing-lanes-side-verdict
kind: issue
title: A confirmed face pair answers for a side verdict the census reads under Declared::confine, not only for the touch's record
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [tier-3-prime-reads-no-side-between-coincident-edges-on-one-line, tier-3-prime-reads-no-side-between-coincident-curved-edges]
---


Found by PR 5529's verifier (branch `restread/coincident-wedges`).
The fix there covers pass 5's collinear lane only.

## What

`Declared::confine` (`census.rs:1179`, PR 5605) runs a lane and, where
a confirmed face pair holds the touch point in its overlap's closure,
drops everything the lane found. The ratified clause (`docs/DESIGN.md`,
PR 5605) is about the record: such a touch "is part of that pair …
and leaves the same contact". A confirmed pair confirms the two faces'
area contact. It never reads the materials' sides where that area ends
on a line or a point, so it cannot answer for a side verdict.

PR 5529's verifier showed the hole on pass 5's collinear lane: a
pinwheel of two prisms doubles the unit square's material while every
corner line lies in a confirmed pair's closure, and `confine` dropped
the four decided `CoincidentWedgesOverlap`. That lane now reads its
sides outside `confine`
(`coincident_line_wedges::a_pinwheel_resting_on_confirmed_face_pairs_refuses_its_doubled_core`).

The same wrapper still holds a side verdict elsewhere:

- **Pass 5's crossing lane** (`census.rs:4079`, `pair_edge_edge`'s
  `Meeting::Cross` arm, around `ee_crossing_lane` at `census.rs:4777`).
  Its `CrossingBacking::Refused(verdict)` arm (`census.rs:4804`) pushes
  `EdgeEdgeCross { side: Some(SameSide) }`: two edges crossing with
  their materials on one side, interpenetration evidence. Under a
  confirmed pair holding the crossing point, it is dropped.
- **The vertex lanes** (`census.rs:2459`, `:2531`, `:2647`, `:2868`)
  push only records. Arm 2's touch analysis reads a cross-solid site's
  sides from those findings (`Named::of` → `Said::Touch`), so a dropped
  one is a site arm 2 never reads. Its records route (`recorded`) reads
  no face pair. Whether arm 2's containment probe catches every such
  interpenetration is unread.
- **Pass 5′'s curved lane** (`pair_curved_edge_edge`, `census.rs:4224`)
  runs under `confine` too. It pushes only records today, but the
  side reading filed as
  `tier-3-prime-reads-no-side-between-coincident-curved-edges` will
  meet the same trap when it lands.

Not a hit: pass 4's `edge_in_own_face` (`census.rs:3655`, from
`ef_overlap_lane` at `:3633`) does not run under `confine`. A
confirmed pair only adds backing to a cell's bounds there
(`ef_bound_backed`, `census.rs:3060`), which reaches the side read
rather than skipping it.

Unprobed: PR 5529's second verifier tried two same-side crossings
inside a confirmed pair's closure (flush and stepped); both made
transversal face crossings that tier 3 catches. No body has yet been built that drops a same-side crossing
under a confirmed pair. The shape to try is the pinwheel turned so two
of its edges cross at a point of a flush wall's boundary rather than
run along a corner line.

## Owed

Read each lane's side verdict outside `confine`, as pass 5's collinear
lane now does: `confine` returns whether a pair answered, and the side
reading runs when the touch is certified either way. Or have `confine`
keep a finding it cannot answer for, by kind. Either way, make the
question one each caller has to face: `confine`'s answered flag is
dropped by every caller but `pair_edge_edge`, carries no `#[must_use]`,
and its doc still says a confirmed pair "answers for" what the lane
finds, "an in-band reading included", where that now holds of records
only.

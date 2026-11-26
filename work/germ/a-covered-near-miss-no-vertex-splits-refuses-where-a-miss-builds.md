---
id: a-covered-near-miss-no-vertex-splits-refuses-where-a-miss-builds
kind: issue
title: a covered arc passing a carrier in the band's gap, which no vertex splits, refuses where a miss would build
status: open
opened: 2026-11-26
priority: P3
cost: M
refs: []
---

Found by GERM U4b's review (PR 7809), the covered posture's gap rule.

## What it is

A covered arc's posture is read off `cross`'s answer
(`reduce::covered_posture`, `crates/topo/src/boolean/reduce.rs`). A
method's sign read in the band's gap is the zero posture there: the
cover certifies the arc's residual one-signed, so the reading is a touch
or a miss, never a crossing. The zero posture's covered rung records no
incidence a miss would not, so it never builds a different body.

It is not free. The rung reads each end by its own side and defers an
interior it cannot see (`CurvedEvent::Deferred`) to the fragments both
member orders leave (`reduce::settle_deferred`). A pair that no vertex
splits is read again exactly as it was deferred, and refuses typed
(`CurvedPierceUnsupported`). A near miss in the gap, with its ends off
the carrier and nothing splitting it, therefore refuses where the same
arc moved past `Kε` builds.

Evidence: `curved_posture_rows::a_covered_near_miss_in_the_gap_records_nothing_a_miss_would_not`.
A disc's circle passes a wall sheet 3ε outside it, its parent covered
`Off(Positive)`. The sweep defers it against both faces of the sheet and
the settle refuses it typed; moved `100·Kε` clear, it defers nothing and
settles clean.

## What fixes it

The settle reading a deferred pair nothing split by a reading that can
tell a near miss from a touch: the gap reading's window localized against
the carrier at a finer band, or the arc's least distance from the carrier
certified past zero, so a miss the cover already bounds one-signed reads
as no event.


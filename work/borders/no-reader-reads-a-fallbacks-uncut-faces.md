---
id: no-reader-reads-a-fallbacks-uncut-faces
kind: issue
title: No naming reader reads BooleanNaming::uncut: a name that depends on a dropped face the fallback could not divide is neither refused nor marked
status: open
opened: 2026-10-25
priority: P2
cost: M
design: true
refs: [5706]
---

## What

FLUSH's PR 5706 has the containment fallback divide each dropped
coincident face where the region its kept face holds ends
(`crates/topo/src/boolean/ops.rs`, `fallback_fates`). Where that face's
cut meets one of `fate::cut`'s placement limits
(`crates/topo/src/boolean/fate.rs`, `Limit`), the face is
recorded in `BooleanNaming::uncut` (`topo::UncutFace`, with the cut's
refusal) and in no other row: neither `substituted` nor `discards`
names any part of it.

No reader reads `uncut`: `editor-core` `names/borders.rs`
(`Obstacles::record`) and `names/emit_union.rs` (`Links::judged`) read
only `substituted` and `discards`. A name whose `Borders` or
`Merged` set would read the uncut face's held part therefore reads an
absence and says nothing of it.

The open question is whether such a name should refuse, typed, citing
the uncut row, or be named and carry a mark.

## Repro

- `sweep` `a_closed_conic_in_the_partners_face::the_fallback_cuts_a_closed_rim_or_records_why_not`:
  the boss `r = 0.3` less the holed plate, and less the holed plate
  with a lid. The plate's top is uncut ("a fate cut's end lies in no
  corner of the dropped face"); the lid's bottom is held in part.
- The PR 5706 review probe `boss ∖ (holed plate ∪ lid)`
  (`sweep` `fallback_rows_review_head::fallback_rows_review_uncut`,
  ignored).

The cut's limit itself is FLUSH's
`work/flush/a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch.md`.
Once that closes, `uncut` is empty on every known pose. A reader's
answer still owes the rows a meaning for the next limit.


## The first repro no longer refuses (OFFSET, 2026-10-27)

PR 5875 levers the fate cut's corner at each bound's extent from above,
and the boss over the holed plate (with and without the lid) now cuts
its top: the row, renamed
`a_closed_conic_in_the_partners_face::the_fallback_cuts_a_closed_rim_beside_a_ring`,
asserts that nothing is uncut. FLUSH's
`a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch` keeps the
plug-in-a-bore and pinch poses, which that PR did not measure, so a
repro for this item is to be read from those.

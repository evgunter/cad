---
id: a-seam-region-holding-a-minted-wrap-tip-leaves-the-cover-census-unread
kind: issue
title: check 6's cover census leaves a sphere's seam region unread where a face holds a wrap tip import minted: an over-cover there is never named
status: open
opened: 2026-11-25
priority: P1
---


Found by the dual review of `lathe/import-wrap-tip` (PR 7774, R1's M2).
Read there.

## What

Import reaches a sphere's pole that lies inside a face by one wrap
edge, walked out and back (DESIGN `[topology]`'s seam clause). A sphere
covered by several faces, each holding a pole, is then a seam region
whose faces each carry a wrap tip. `crates/topo/src/validate.rs`,
`seam_region`, leaves such a region unread. Its boundary cycles pass a
vertex twice (`pinched`), or its loops do not flatten. So check 6's
cover census (`SeamRegionOvercovered`, `SeamRegionCoverUnresolved`)
cannot fire there, under any sense bits.

Witnesses: `crates/step-import/tests/fixtures/seam-region/side_cut_equator_split.step`
and `side_cut_meridian_split.step`. With every sphere face flipped,
main's two-half form read the region `SeamRegionOvercovered { times: 4 }`.
On the one-band stack's import the census names nothing.

No wrong body ships. Each flipped face is still named
`CurvedSenseInverted`, and by the pole its complement holds
(`ChartSingularityNotAVertex`). Each flipped wrap-tip face also reports
`Pcurve { LoopNotClosed }`. So the census is the reader that goes blind,
not the gate.

The row that goes red the day this closes is
`crates/step-import/tests/poles_inside_a_seam_region.rs`,
`the_cover_census_reads_a_seam_region_holding_a_wrap_tip`, which is
pinned red (`test_utils::pinned_red`) at every ε.

Kin: `a-pinched-seam-region-has-no-witness.md`, whose premise ("no body
reaches that branch") these files now break if the region reads as
pinched.

## What would close it

`seam_region` reads a region whose boundary walks a wrap edge out and
back. Read as tier 3's surface boundary does: the wrap edges dropped,
each tip kept as the datum of the face's side. Then the cover census
counts it, and the pinned row passes and has its pin removed.

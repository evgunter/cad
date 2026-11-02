---
id: a-line-edges-span-bounded-by-a-conic-crossing-is-not-recorded
kind: issue
title: A line edge's span in a face bounded by a crossing of the face's conic boundary edge is not recorded
status: open
opened: 2026-10-11
priority: P4
cost: M
refs: [an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap]
---


Unmeasured, read. The blind spot of `an-apex-edge-…`'s sweep.

## What

`boolean::ops::record_spans` takes a bound of a line edge from an
edge-edge record only where the other edge is a line too: it computes
the crossing as the nearest point of two lines. Where a line edge lying
in a plane face leaves it across one of the face's arc boundary edges
(a shelf edge across a half-disc cap's rim), that crossing bounds no
span, so the edge's record against the face is not minted, whatever
records the op made at the crossing.

## Why it is P4

The census refuses every such configuration already: pass 4 cuts the
overlap at the conic crossing (`CutAt::CurvedCrossing`, named `ConicCrossing` before stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227) gave the cut every carrier kind) and backs that
bound with nothing, and CONTACTHOLD measured the same seat refusing
(`contacthold/census-edge-pass-reads-no-line-conic-crossing`, closed;
CHART's `a-planar-face-with-a-conic-trim-has-no-region-door` is the
census side). So the missing record changes no verdict today.

## Owed

First measure whether the op records the crossing at all (an
edge-edge record of the line and the arc). If it does, the bound is
the line's point on the arc's carrier, read where the op decided it;
then the span is minted as the line-line case is.

## Note (2026-10-15, fuse/curve-pair-reader-c3)

The census side moved. Tier 3′'s pass 5′ (`census::sweep_curved_edge_edge`) owes the crossing of the line and the arc its edge-edge record, and pass 4 now backs a bound at a curved crossing (`CutAt::CurvedCrossing`, which names the arc) by that record. So a seat whose op records the crossing no longer refuses on the bound, and "the missing record changes no verdict today" is to be measured again.

## Measured on a curved partner, and 3′ does not always refuse (CONTACTHOLD, 2026-10-19)

The PR 5389 review (`crates/sweep/tests/review_5389_probes.rs` on
branch `contacthold/two-tubes-zero-band-review`, `probe_rod_on_puck` and
`probe_unrecorded_contact_detail`): a rod `r = 0.25` lying along `x`
on a unit puck's top face, `0.5·ε` above it, its seam ruling on the
contact line and the rod ending at `x = ρ`, on the puck's rim. The seam
lies in the puck's top face between the rim crossing and the rod's
end. Both operand orders build ∪ and tier 3′ reports
`UnrecordedContact { EdgeFaceOverlap }` on the seam and the top face:
the op mints no span record for it. With the puck spun 1 rad about its
axis the record is still missing and 3′ reports nothing, so the "the
census refuses every such configuration" premise above does not hold
for this pose; the missing record then ships silently. Not measured
further.

## Measured recorded on the spun pose (PARTING, 2026-11-01)

`parting/sweep-records-edge-in-face` moves the record to the reduction
sweep (`boolean::reduce::settle_in_plane`), which no longer needs a
bound at the crossing: it records each piece of a line edge the sweep
read lying in a plane face whose end the sweep recorded inside the face
or whose midpoint the face holds. On
`two_tubes_in_the_zero_band::a_rod_on_a_puck_with_its_seam_on_the_ruling`
(the puck spun 1 rad) ∪ now carries the seam / top-face record beside
the rim's two edge-edge touches, in both operand orders (records 2 → 3),
with tier 3′'s findings unchanged. The seam is not split at the rim
crossings there, so the record names the whole seam edge, whose
interior meets the face's between them. The unspun pose (the rod ending
on the rim, `review_5389_probes`) was not measured.

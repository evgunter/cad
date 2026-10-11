---
id: an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap
kind: issue
title: A triangle prism's apex edge in the partner's face plane builds and fails tier 3' on one EdgeFaceOverlap the boolean neither refuses nor records
status: dispatched
opened: 2026-10-11
priority: P0
cost: M
refs: [a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts, 4650]
branch: flush/apex-edge-in-face-plane
pr: 4737
---


Split out of `a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts`
(its "Also in `join1_delta_arc_battery`" section), whose corner-pair
runs PR 4650 fixes and these it does not.

## What

`join1_delta_probes.rs` `join1_delta_arc_battery`, the poses
`ARC tri {half, quarter, shallow} d=(0.0, 0.5) zb=(0.5, 1.5) decl={false, true} U`:
six unions build at the volume with tier 2, the certificate and a legal
operand, and fail tier 3′. The row they came from read one
`UndeclaredContact { EdgeFaceOverlap }` at `(0.0, 0.5, 1.0)`: the
triangle's apex edge `x = 0, y = 0.5` lies in the partner's face plane
`y = 0.5`. (That row named `tri tri` where this measurement fails
`tri quarter`; `tri tri` is `SOUND` here.)

## Measured (2026-10-11)

The same six lines fail tier 3′ on main (merge base `8c8f3b4c`), on
PR 4650 (`flush/corner-ruling-tier3`, which lands vertex-on-face records
on the piece of a divided face that holds the vertex), and on PR 4604's
head `29f98d7a` (CONTACTHOLD, which records every non-incident cell
pair). None of the three moves them, so this is not a record carried
to the wrong piece. Read, not measured: one `EdgeFaceOverlap` and no
`VertexOnFace` suggests the apex edge's ends are not in the face's
interior, so no rest is minted at all, and nothing records the edge
lying in the face.

## Owed

Read one pose's reduction contacts: whether any decision places the
apex edge on the face, and where the overlap's bounds lie. Then decide
whether the op owes an edge-face record (4604's currency) or a
vertex-on-edge record at the bounds.

## Measured through the plain booleans (2026-10-11)

The declared spelling is gone, so the six lines are three: `ARC tri
{half, quarter, shallow} d=(0.0, 0.5) zb=(0.5, 1.5) U`. On main
`bef9a8f1` (with this branch's build fix, below) and on `b65bc54d`
(PR 4650 merged, before PR 4613) alike, each builds at its closed form
and fails tier 3′, but **not on an `EdgeFaceOverlap`**. Its findings are
all `CensusUndecidable { CurvedWithinReach }`: the census's
cross-solid backstop (`census.rs`, arm 1) refuses the triangle's
slanted faces against the partner's arc-bounded caps and cylinder wall.
The two solids touch along the apex segment, so no separation clears
those pairs. That is RESTREAD's
`census-cross-solid-curved-pairs-undecidable-on-shell-results`, where
this measurement is now recorded. The `EdgeFaceOverlap` was the parent
row's earlier read and does not reproduce here. This row's own
measurement only read `t3p=false`.

The reduction makes two vertex-vertex decisions, the apex edge's
crossings with the partner's two cap edges at `z = 0.5` and `1.5`. The
join makes both split pairs whole again, so the result carries two
edge-edge records (apex edge × cap edge, `half`, `shallow`) or two
vertex-on-edge records (the partner's corners on the apex edge,
`quarter`). Pass 4's overlap lane backs both bounds from them, and no
`EdgeFaceOverlap` is raised. But nothing records the span itself: D1
owes "every pair whose interiors meet … an edge lying in a face or
along an edge", so the op owed both the bound records (present) and the
apex edge against the face `y = 0.5`, or against the partner's corner
edge it runs along (`quarter`), which was absent. The same holds on the
all-planar `ARC tri tri`, which passes 3′ only because the census
reconstructs the span from its bounds.

## Fix

`boolean::ops::record_spans`, run by `carry` after every record is in
result keys: for each line edge a record reaches, the records naming a
point of it (a vertex on it, a line edge crossing it, one of its ends
touching a cell) are its bounds, and so are its own ends by
structure. Between consecutive bounds the span is recorded against
every cell both bounds touch whose interior holds its midpoint: a plane
face, placed by `piece_holds` (the census's own containment read), or a
line edge, by `census::on_segment_interior`. Each record cites the two
bounds' records. A curved face is skipped (the FUSE twin's case, now
pointed at this function).

The batteries do not move (3675 arc lines and 16380 corner lines, base
against head). Three record-count pins each gain one edge-edge record
D1 owed, a span along a collinear edge between bounds that were already
recorded: `records_cite_their_decision`'s block beside the kiss,
`a_ruling_lying_on_a_wall`'s void and notch on a seam ruling, and
`ring_carry_through_by_piece`'s pocket edge down the bore's seam ruling.
The three lines stay red on RESTREAD's arm 1.

A body now carries a coincident edge pair's edge-edge record into its
next boolean, and `ee_lineage` places that record through the splits.
Three defects in that lineage were reached for the first time, and
are fixed here:
- `pieces` read a piece's ends off the clone after the insertion, where
  a null edge had moved them onto its copy, so a decision named a vertex
  no input holds. It now reads each end back to the vertex the copy was
  minted at.
- A minted vertex was read against every piece's extended line, however
  far past its ends.
- Two pieces were read along the first one's line, however short.

Both levered readings put a pair within the zero into the band
(`three_solids_on_one_line`, the 0.1ε row). The census's own
collinear test has the same asymmetry:
`restread/census-collinear-overlap-reads-along-the-first-edges-line`.
With the lineage reaching its pieces, WIRE's
`a-boolean-clipping-a-carried-contact-at-a-pinch-records-no-contact-for-the-pierce`
witnesses pass 3′, and `union_pinch_member_order`'s four pins assert
`Ok`.

Residue filed: `a-line-edges-span-bounded-by-a-conic-crossing-is-not-recorded`
(FLUSH, P4).

---
id: an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap
kind: issue
title: A triangle prism's apex edge in the partner's face plane builds and fails tier 3' on one EdgeFaceOverlap the boolean neither refuses nor records
status: closed
opened: 2026-10-11
priority: P0
cost: M
refs: [a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts, 4650]
branch: flush/apex-edge-in-face-plane
pr: 4737
closed: 2026-10-12
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

The op owed two records, and they now come from two producers.

- **Along an edge** (`quarter`'s apex edge on the partner's corner
  edge, and the collinear pins): CONTACTHOLD's PR 4740 (merged
  2026-10-12) has the reduction write the touch's edge-edge record
  (`boolean::touch_rows`), and the census asks for it. Its carried
  lineage (`ops::uncopied`, `census::past_segment` /
  `segments_apart`) also fixes what this branch had first fixed in
  `ee_lineage` (null-edge copies, the levered segment reads). PR 4737
  merged main and took 4740's version of all of it.
- **In a face** (`half`, `shallow`, `tri`):
  `boolean::ops::record_spans`, run by `carry` once every record is in
  result keys.
  - For each line edge a record reaches, the bounds are the records
    naming a point of it (a vertex on it, a line edge crossing it, one
    of its ends touching a cell) and its own ends, by structure.
  - Between consecutive bounds, the span is recorded against each plane
    face both bounds touch whose interior holds its midpoint
    (`piece_holds`, the census's own containment read), citing the
    readings that put each bound on that face.
  - It reads the census's parallel test and crossing point
    (`census::segments_parallel`, `segment_cross_point`; `rim_wedge`
    calls the latter too).
  - A curved face is skipped (the FUSE twin's case, now pointed at this
    function).

The arc, corner and pinch batteries do not move against main, nor the
review's span battery against the pre-fix head. The three lines stay
red on RESTREAD's arm 1. A carried edge-face span is still dropped by
`ContactRecords::carried` (WIRE's
`a-boolean-drops-its-operands-curve-and-patch-records`, evidence there).

Residue filed: `a-line-edges-span-bounded-by-a-conic-crossing-is-not-recorded`
(FLUSH, P4).

---
id: tier-3-prime-reads-no-side-between-coincident-edges-on-one-line
kind: issue
title: Tier 3′ licenses coincident edges on one line by their shared ends and never reads their wedges
status: closed
opened: 2026-10-19
priority: P2
cost: E
closed: 2026-10-21
branch: restread/coincident-wedges
---


Found by `census-passes-a-face-inside-an-edges-material-wedge`'s lane
(branch `restread/face-in-edge-wedge`), by reading.

## What

Pass 5's collinear lane (`census.rs` `ee_collinear_lane`,
`ee_bound_backed`) certifies two coincident line edges on one line by
their bounds alone (shared vertices or one point key), and reads no
side. D1 (`crates/topo/README.md` tier 3) has the material sectors on
one line disjoint, and tier 3′ "reads every touch for sides, the
coincident edges on one line included".

The Boolean result gate runs `census::line_wedges` over every line
`census::collinear_overlaps` finds (CLEAVE,
`boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none`),
so no Boolean result reaches rest with overlapping wedges. Tier 3′
does not: STEP adoption admits any body tier 3/3′ passes, and
`insert_voids(box, [s, s], cavity)` grafts the unequal-pinch touch as
two 270° edges whose wedges overlap by 180°, which tier 3′ refuses
only for want of records
(`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).

## Fix

Run `census::line_wedges` in the census, on the lines where pass 5
certifies a collinear overlap, and refuse an overlap typed beside
`FaceThroughEdgeWedge`. Witnesses: a reverted slit body handed to
`validate_pseudomanifold` directly, and the `insert_voids` graft
above with its records.

## Evidence: the tangent pinch

Ev's decision on `[ev]` PR 5395 makes a material-crescent pinch along a
tangent line two coincident cusp edges (D1 tier 3, C7). A complement
that fails to re-pair leaves two coincident edges each claiming a 2π
wedge, which tier 3 reads one edge at a time and passes; this row's
check, that coincident edges' sectors are disjoint, covers it (sectors
0 and 2π, read at second order). Builder: CONTACTHOLD's
`a-material-crescent-tangent-line-builds-two-coincident-cusp-edges`.

## Evidence: two readers of wedge overlap on a line

PR 5431 added a second reader of whether two material wedges about one
line overlap. `census::line_wedges` (CLEAVE) reads two coincident
edges' wedges through tier 3's pinch-corner reading
(`validate::wedge_holds_named`) at the middle of their shared span;
`census::edge_in_own_face` reads an edge's wedge against a face of its
own solid lying through the line through the touch analysis
(`Link::on_edge`, `Link::in_face`, `rest::judge`). With tier 3's own
pinch-corner check and the cross-solid touch analysis these are four
spellings of one question. Running `line_wedges` in the census (the fix
above) is the place to choose one: either `rest::judge` over two
`Link::on_edge` links, or `line_wedges` extended to a face's two sheets.

## Closed

2026-10-21, branch `restread/coincident-wedges`. Pass 5's collinear
lane reads two line edges' material wedges wherever it certifies their
overlap (`census.rs` `ee_collinear_lane`, `coincident_wedges`), through
CLEAVE's `census::line_wedges` at the middle of the shared span: no
new reader. A decided overlap refuses `CoincidentWedgesOverlap`,
whatever backs the pair. A reading not taken refuses
`CoincidentWedgesOverlapUndecided`, naming why with `TouchUnread`: in
band (`InBand`), a face with no sheet at the line (`Unreadable`), or a
cusp or knife-slit edge (`Degenerate`). Arm 2 reads the first as a
crossing at the line and the second as unexamined.

The census now asks the line question through `line_wedges` for two
edges, and through `rest::judge` for an edge in a face
(`edge_in_own_face`), which `line_wedges` does not reach. Folding the
two into one is filed as
`coincident-edge-and-edge-in-face-wedges-are-read-by-two-readers`.

- Pinned refusals
  (`crates/topo/tests/a_doubled_line_does_not_revert.rs`), at every
  pose:
  - the slit body grafted into a box as a cavity, reversed with no
    re-pairing (`test_support::grafted_reverted_unrefused`);
  - `insert_voids(box, [s, s], pinch)` of the unequal bricks, with its
    two records;
  - two wedges `4 ε` apart on one line, refused undecided in band.
- No shipped body newly refuses. Topo and sweep, slow set included,
  hold only main's known reds at the default ε and at 1e-12. The slit
  form (`a_doubled_line_does_not_revert`,
  `a_keel_wrapped_at_both_ends_splits_as_a_ring`) and the Boolean
  gate's rows (`gate_reads_coincident_wedges`) pass.
- Not read: two coincident curved edges (pass 5′), filed as
  `tier-3-prime-reads-no-side-between-coincident-curved-edges`. A
  material crescent's two coincident cusp edges refuse `Degenerate`
  until a second-order reading lands, noted on CONTACTHOLD's builder
  row (`a-material-crescent-tangent-line-builds-two-coincident-cusp-edges`).

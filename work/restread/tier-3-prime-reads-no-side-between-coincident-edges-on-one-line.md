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

2026-10-21, branch `restread/coincident-wedges` (PR 5529). Pass 5's
collinear lane reads two line edges' material wedges wherever their
overlap is certified (`census.rs` `pair_edge_edge`, `ee_overlap_sides`),
through CLEAVE's `census::line_wedges` at the middle of the shared span:
no new reader. Certified means backed and recorded
(`ee_collinear_lane`), or answered for by a confirmed face pair holding
the span's middle (`Declared::confine`). The pair answers for the
touch's record, never for its sides, so the reading runs outside
`confine`.

- **Decided.** An overlap refuses `CoincidentWedgesOverlap`, whatever
  backs the pair.
- **Not taken.** A reading not taken refuses
  `CoincidentWedgesOverlapUndecided`. It names the edge whose wedge has
  no reading (`unread`) and why (`TouchUnread`):
  - in band (`InBand`, no edge named);
  - a face with no sheet at the line (`Unreadable`);
  - a cusp or knife-slit edge (`Degenerate`);
  - an edge too long or short to scale its sheets by (`ChordScale`).
- **Arm 2.** Between two solids, arm 2 reads a decided overlap as one
  passing into the other (`MixedTouch`), and an unread one as that
  reason's touch refusal (e.g. `TouchInBand`). Inside one solid, a
  decided overlap is a crossing and an unread one is unexamined; either
  way, no other solid near it is cleared.
- **The Boolean's gate.** It carries the same reasons
  (`LineWedgeFault::Unread { edge, why }`).

The census now asks the line question through `line_wedges` for two
edges, and through `rest::judge` for an edge in a face
(`edge_in_own_face`), which `line_wedges` does not reach. Folding the
two into one is filed as
`coincident-edge-and-edge-in-face-wedges-are-read-by-two-readers`.

- **Pinned refusals**
  (`crates/topo/tests/a_doubled_line_does_not_revert.rs`,
  `crates/topo/tests/coincident_line_wedges.rs`,
  `crates/sweep/tests/two_wedge_ends_on_one_line.rs`):
  - the slit body grafted into a box as a cavity, reversed with no
    re-pairing (`test_support::grafted_reverted_unrefused`), at every
    pose;
  - `insert_voids(box, [s, s], pinch)` of the unequal bricks, with its
    two records, at every pose;
  - two wedges a hair apart, a prism wall leaning `40 ε` across a unit
    width over an edge `0.1` long, refused exactly undecided in band,
    with arm 2's in-band touch;
  - a pinwheel of two prisms doubling the unit square's material while
    resting on four confirmed face pairs, at every pose;
  - two lune cavities through the void door whose cusps kiss, refused
    `Degenerate`;
  - review 1's cross-solid rows: bricks sharing a quadrant, and a prism
    leaning into a brick;
  - a second solid floating in a host whose own wedges overlap, or went
    unread.
- **Run coverage.**
  - No shipped body newly refuses.
  - At the final head (`fa60a5f8`, measured by the second verifier):
    topo 3418/3418 at both the default ε and 1e-12, slow set included;
    sweep 3115/3116 at the default ε and 3108/3116 at 1e-12, every red
    one of main's known reds, and no line-wedge finding in either log.
- **Not read.**
  - Two coincident curved edges (pass 5′), filed as
    `tier-3-prime-reads-no-side-between-coincident-curved-edges`.
  - A material crescent's two coincident cusp edges refuse
    `Degenerate` until a second-order reading lands, noted on
    CONTACTHOLD's builder row
    (`a-material-crescent-tangent-line-builds-two-coincident-cusp-edges`).
  - The one-point reading on a face whose tangent plane turns along the
    line is filed as
    `coincident-line-wedges-are-read-at-one-point-of-a-turning-face`.
  - The same `confine` hole on pass 5's crossing lane's side verdict is
    filed as
    `a-confirmed-face-pair-swallows-the-crossing-lanes-side-verdict`.

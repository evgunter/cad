---
id: zero-to-distinct-cells-on-one-point-doors-audit
kind: issue
title: Audit the doors other than the boolean that turn a Zero into distinct cells on one point (e.g. a revolve whose profile meets its axis twice): does each record the coincidence it decides?
status: closed
opened: 2026-10-10
priority: P2
cost: M
branch: contacthold/zero-to-distinct-cells-audit
pr: 4765
closed: 2026-10-12
---


Split off `boolean-vertex-contact-records-are-inferred-from-values`
(closed 2026-10-10), whose "Owed" item 3 this is. That row is answered
for the boolean: every `ContactRecords` row cites the `Coincidence`
that decided it (`boolean/reduce.rs::push_vv` / `push_vf` →
`boolean/ops.rs::cite_rows`, `DecisionSite::VertexFusion`), and the
`unproven-coincidence` lint walks those coincidences.

## The question

Which other doors decide a Zero and leave two distinct cells standing
on one point, rather than fusing them? For each, does the door record
that decision as a `Coincidence` (D10: a Zero coincidence is recorded
at the one door and linted), or does a value coincidence survive into
the result with nothing citing it?

Candidate named by the parent row: a revolve whose profile meets its
axis twice (two poles on one axis; a profile vertex on the axis whose
image is one point for two profile cells). Other candidates to check:
the sweep doors' pole and seam handling, and any construction that
keeps copies (the boolean's `copies_of` is the boolean's own case).

## Owed

1. List the doors, by name, that can produce distinct cells at one
   point from a Zero verdict.
2. For each: recorded and cited, fused, or uncited. File each uncited
   one on its owner's slate.

## Audited (branch `contacthold/zero-to-distinct-cells-audit`)

The census is the net under every door: two cells on one point that
no record backs are an `UnrecordedContact` at rest, unless the two
share one point key, which is structure (D1) and clears unread. Only
an op's copies share a point key, and only `topo` mints them
(`Body::mev_null`, `Body::share_point`; outside tests, the split's
`splitting/insert.rs` and the boolean's `insert.rs`, `vtxfac.rs` and
`zip.rs`). So a door outside those two that leaves distinct cells on
one point is refused at rest; the silent case is a copy no row
records.

Each door that turns a Zero into cells on one point:

- **Split** (`splitting/mod.rs::reduce`): a pinch keeps one vertex per
  piece on the vertex's point key, and records `SplitOn` per ON vertex
  with two or more runs. A grazed curved edge never pinches: convex
  lands whole; concave refuses, `KnifeEdge` where the plane is tangent
  to the bore's wall along its ruling, and `SectionGermsUnordered
  { Coincident }` where it is tilted `+z` through the rim (a bored
  block, the plane through the rim point with normal `(1, 0, 0.3)`);
  tilted `−z`, it lands with no row. Recorded.
- **Boolean pinch** (`zip::split_cones`): a vertex decided on the other
  operand's cell whose result keeps one vertex per cone. Recorded
  NOTHING: `carry` emitted a decision only where a surviving record
  cites it, and copies on one point key need no record. Measured on the
  L-prism and tilted cube (`join_pierce_strut_facing.rs`'s poses): 30
  pinched results, 0 rows. Fixed here: `carry` emits the row of each
  decided vertex at a pinch (`pinch_rows`). A document union or
  intersect of three or more members still drops it
  (`work/wire/a-union-fold-publishes-no-pinch-row.md`).
- **Census at rest** (`record_at_rest`: placed copies, instances,
  STEP imports): records `CensusAtRest` and a record citing it between
  groups; a touch inside one group is a finding. Recorded.
- **Revolve, full**: an on-axis vertex is one pole; an isolated contact
  refuses (`NonManifoldAxisContact`), and two or more runs build one
  cavity per gap, whose poles are profile points distinct from the
  outer shell's, so nothing touches; a
  sphere arc's apex on the axis refuses (`ArcCrossesAxis`); a horn
  torus refuses (`UnsupportedToroid`). **Partial**: an on-axis vertex
  is one vertex both caps share. Fused or refused.
  `tube_along_arc` runs the same machinery.
- **Extrude, revolve and loft profiles**: profile validation refuses a
  touch (`NonSimple { Touch }`, `TangentialContact`). Refused.
- **Loft**: adjacent coincident sections refuse (`DegenerateStacking`).
  `sweep_body` along a closed path decides nothing and builds coincident
  caps, refused at rest only (evidence added to
  `work/carve/self-overlapping-spines-build-and-validate.md`).
- **Shell**: a Zero clearance refuses (`WallClearance`, `OffsetsCross`
  away from a joint); its rim's null-edge copy is collapsed onto ring
  vertices. Refused or fused.
- **Blends**: decide through the battery's recorded sites; the sweep
  crate cannot mint a shared point key, so a coincident vertex it left
  would be a census finding.

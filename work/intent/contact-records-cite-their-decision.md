---
id: contact-records-cite-their-decision
kind: issue
title: D10 stage 4 B2: every ContactRecords row cites the Coincidence that backs it, so a touch without a decision cannot be built
status: review
opened: 2026-10-08
priority: P0
cost: H
branch: intent/s4-b2-records-cite
---


## What

INTENT stage 4, unit B2. Ratified in #4322 (fork log row 92,
FORK-S4P; D1 (ii), D10 Coincidence): a contact record a result carries
cites the coincidence that backs it, so a touch without a decision
cannot be built. B (`coincidences-are-recorded-at-one-door`) lands the
decision record (`topo::Coincidence`), the door and the lint, and leaves
`ContactRecords` as it was (orchestrator ruling on S4-B, 2026-10-08,
option (b)); this unit makes every record cite.

## The three sub-questions, as ruled

- **A carried record** (an operand that is itself a result, carried in
  through `BooleanDeclarations`' carried contacts and `split_lineage`)
  cites the read of the operand result and the index into that
  result's list, never a bare index.
- **The instantiate seam** (`wire_instantiate_part`'s
  `OpOut::contacts` and `CarriedDeclarations`) cites the part's
  decision through the instance's read.
- **At-rest and census records** cite theirs.

No record says "carried from <read>" without an index (rejected: a
second way for a record to say where it came from).

## What it folds in

The reduction's vertex identities (`ContactAcc` `vv`/`vf`, `reduce.rs`
`push_vv` and the `contacts.vf` sinks): each becomes a `Coincidence`
with its margin threaded out, which is
`value-decided-vertex-fusions-are-recorded-with-the-undeclared-glue`'s
half for the undeclared glue; the cited record is this unit's.

## Added from E's review (orchestrator, 2026-10-10)

A face-pair row is kept only where its faces meet: E's glue door
records a row for every box-overlapping pair the ladder decides one
carrier, so its rows followed the frame (two blocks apart: 0 rows
axis-aligned, 2 turned 45°). B2 keeps a face-pair row, declared or
not, only where the reduction placed a cell of one face on a cell of
the other (`boolean::glue::touched`): the faces meet at a point, an
edge or an area. Pinned in `topo/tests/records_cite_their_decision.rs`.
"Where the glue took effect" is stricter than is buildable (spec test
15's Rest row has no merged face and no surviving record); the
narrower reading is filed as
`a-face-pair-row-only-where-its-decision-shaped-the-result`.

## The sites that move

A grep of `VvContact|VfContact|VeContact|EeContact|CurveContact|PatchContact|ContactRecords`
at B's merge base: 99 files, 713 lines. Production (the citation's
producers and consumers):

- `topo/src/boolean/ops.rs` (76), `census.rs` (65), `boolean/mod.rs` (39),
  `boolean/insert.rs` (17), `boolean/reduce.rs` (16), `boolean/vtxfac.rs`
  (15), `props.rs` (10), `validate.rs` (9), `boolean/sectors.rs` (6),
  `lib.rs` (6), `tier3_tests.rs` (6), and the rows in
  `coplanar_conic_rows.rs`, `planar_lane_carrier_rows.rs`,
  `shell_witness.rs`, `torn_hop_rows.rs`, `cert_m3r1_probes.rs`;
- `editor-core/src/assembly.rs` (11), `product.rs` (9), `eval/{mod,wire,parts}.rs`,
  `verbs/boolean.rs`, `checks.rs`;
- `step-import/src/lib.rs` (9); `pncad/src/prelude.rs`;
  `pncad-py/src/py/value.rs`; `verbs/src/run.rs`.

Tests: the rest of the 713, dominated by `topo/tests`
(`vertex_on_edge_records.rs` 32, `bool4_material_containment.rs` 20,
`contact5_gate_and_beam.rs` 19, `bool4r2_base_probe.rs` 18, …) and
`sweep/tests` (`m5_pr9_boss_union.rs` 10, …). Every census golden that
prints a record moves (the record gains its citation).

Docs: CONTACT-DESIGN C3's `PatchContact` reads "backed by a
`SameOpposite` decision", every granularity citing its backing, and
topo's preamble gains the decision beside the record (the text #4322
states).

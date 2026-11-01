---
id: a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys
kind: issue
title: A boolean result reused as an operand drops its records, and under the shared point key a dropped vertex-vertex record no longer refuses at rest
status: open
opened: 2026-10-12
priority: P2
cost: M
design: true
---


## What

Found by FLUSH's touch-key lane (branch `flush/touch-key`). A boolean
now puts every pair of vertices its vertex-vertex records name on one
point key (`crates/topo/src/boolean/zip.rs` `record_classes`, run at
the output stage of each path in `crates/topo/src/boolean/ops.rs`).
The census's same-point rung (`census.rs` `Geo::same_point`) reads a
shared key as structure. So the record is no longer what keeps a
point-touch loud at rest: withheld, the touch reads off the key and
passes.

The records do not ride the operand type. `topo::union` /
`intersect` / `subtract` take `AtRestBody` and pass empty
`BooleanInputs::records_a` / `records_b`, and the natural reuse of a
result is its body: the sweep batteries' `unite_all` does
`acc = bb.body`, and so do the probes that build pinched operands.
Before this change a reused point-touch union shipped its corners on
two keys and the census refused the operand's touch
(`UnrecordedContact { VertexVertex }`) once its record was gone. Now it
passes silently. Measured:

- `sweep` `blend_carries_records::a_blend_away_from_the_kiss_carries_it`:
  the chamfered kiss validated with `ContactRecords::default()` went
  from `UnrecordedContact` to `Ok(())` (the test is restated there).
- An edge-edge overlap whose two ends are recorded vertex pairs passes
  too: its bounds sit on shared keys, which the census reads as
  structure (`census.rs` `ee_bound_backed`, `Bound::Shared`).
  `topo` `union_flush_onto_edge_contact`'s two bricks touching along
  an edge went from `UnrecordedContact { EdgeEdgeOverlap }` without the
  edge-edge row to `Ok(())`, and `pierce_strut_at_a_pinch`'s bare-pinch
  and reflex-wedge rows, whose operand is such a union reused without
  its records, went from 3′ refused to 3′ passing in all twelve ops.
  So did `m3_pr6_tier3prime`'s corner kiss and tangent edge withheld,
  and its kiss-closure rows (`closure_kiss_vs_mover` ∪ and ∖).
- A vertex-on-edge or vertex-on-face touch whose record is dropped
  still refuses, by construction: only vertex pairs share a key (not
  measured beyond the suites above).

What a dropped record still costs: the `VertexFusion` row its record
cited, which the `unproven-coincidence` lint reads (D10's coincidence
paragraph). A result reused without its records loses the lint's
evidence for every value-decided touch it holds.

## Related

- `work/wire/a-boolean-drops-its-operands-own-contact-records.md`
  (closed): the editor's fold now carries records through
  `BooleanInputs`; the Rust API's plain booleans do not.
- `work/flush/boolean-given-pairs-glue-unmeasured-for-any-rust-caller.md`:
  the other half of `BooleanInputs` a Rust caller sees.
- `work/intent/a-boolean-value-glue-into-one-vertex-records-no-coincidence.md`:
  a value glue that leaves no record to cite its row.

## The question

Should a boolean result's records travel with its body (an operand type
that holds them, or a door that takes a `BooleanBody`), so a plain
reuse cannot drop them; or is a dropped record acceptable now that the
key carries the touch, with the lint's evidence the only loss? A
designer's question under D10, not a lane's.

## Measured (PR 4823's review, 2026-10-13)

Both reviewers of PR 4823 (FLUSH's touch-key lane) measured the cost.

- **R1:** `topo` `union_flush_onto_edge_contact`'s three-brick fold run
  with `BooleanInputs::default()` (no records carried) builds 17
  vertices / 25 edges in 4 of its order/span cells, with a vertex the
  join should have taken (non-maximal) that passes at rest. At span
  `(−0.5, 0.5)` base built 16 vertices, `SOUND`.
- **R2:** the corner kiss united with `[1,2]×[0,1]×[1,2]` builds 16
  vertices with the kiss's records carried and 17 without.

So a dropped record now costs maximal edges as well as the loud
refusal: the join cannot free a lump's corner no carried record names
(`edge_join::free_vertices`), and the at-rest check 11 reads that
corner's shared key as a pinch and passes it.

The question is with Ev on the `[ev]` PR 4793 (fork-log row 133:
whether D1's "structural sharing needs no record" covers a key a
boolean shared on a value decision). PR 4823's pins hold this row's
open state: they assert the shared keys, and name this row where a
withheld record passes.

## Since PR 4793

Ev approved PR 4793 (2026-10-14). D1 now says that vertices an op
decides are one point share one point key, whether they lie in one
solid or two, and that the op's `VertexFusion` row, cited by the
pair's vertex-vertex record, is for the lint, not the census. So a
withheld vertex-vertex record passing at rest on its shared key is the
design, not a hazard, and PR 4823's pins now say so rather than naming
this row. The live question narrows to whether a boolean's records ride
its operand type (an operand that holds them, or a door that takes a
`BooleanBody`), because a plain reuse that drops them loses the lint's
evidence; R1's and R2's measurements above add that it also loses the
join's freeing, so a dropped record leaves a non-maximal edge at the
lump's corner.

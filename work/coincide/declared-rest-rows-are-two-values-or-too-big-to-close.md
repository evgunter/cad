---
id: declared-rest-rows-are-two-values-or-too-big-to-close
kind: issue
title: The corpus's declared-rest plane-ladder rows stay unproven at the door's full pass: two separately typed values, or a margin form too big to close?
status: open
opened: 2026-10-11
priority: P2
cost: M
---

INTENT stage 4 C+D's corpus census of the door at its full pass
(`census_the_door_over_the_corpus`, `crates/editor-core/tests/coincidence_door.rs`)
leaves these `PlaneLadder` rows unproven: die 21, corner_table 8,
kitchen_sink 4, kiss_carry 3, crossing_slots 1.

On crossing_slots, probed with `measure_a_corpus_document_at_sym`
(`CORPUS=crossing_slots`, release, `--nocapture`): the row is a
declared rest, and its margin is the declared reading's upper bound, a
`max` of sums of norms over the consumed extent and the witnesses.
That margin has 1327 DAG nodes and 52 parameter symbols, its early
form freezes into opaque atoms, and the undeclared carrier-pair verdict
on its cells (rung 1) is no theorem either.

Owed: for each row, which is it?
- (i) Two variables typed to one value, so the row is correctly
  unproven and the recourse is "read one variable".
- (ii) One construction whose margin is too big to close, so a smaller
  reading proves it. Rung 1's offset alone may be that reading.

Separate them with the probe. A row of kind (ii) belongs on SYM's
slate, or is a reason for rung 1 to read the offset datum on its own.

- 2026-11-14 — **The budget arm moved** (BOXFINISH `boxfinish/sweep-frames`): with a sweep's caps their sketch frames turned rather than Newell fits to boxed corners, the declared-rest plane rows' residuals fit the door's budget and come back read whole and not shown zero (recourse `OneConstruction`, not `Budget`): `coincidence_door::two_blocks_resting_on_one_floor_are_one_plane_at_their_bases`, `a_ring_serving_two_pierces_records_their_fusion_unproven` and `glue_on_zero_rows::a_flush_rest_glues_and_records_its_unproven_rows` re-pinned. So "too big to close" no longer holds on those rows; "two values" is what is left.

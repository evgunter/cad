---
id: run-checks-panics-on-a-cut-of-an-instances-copy
kind: issue
title: run_checks panics (checks.rs at_rest_node's unreachable!) on a cut of an instance's copy or of a pick of its piece
status: open
opened: 2026-10-28
priority: P1
cost: M
refs: [a-boolean-over-an-instance-drops-its-parts-census-findings]
---

Found by PR 5903's review (R1, head `38e6cdd3`; probes on
`review/intent-carry-R1` @ `0fe59811`,
`crates/editor-core/tests/zz_review_probes_5903.rs`). Same on main.

`run_checks` panics in the `at_rest_node` closure
(`crates/editor-core/src/checks.rs:1707`), at
`unreachable!("a gathered copy is a live placement's output")`
(`:1709`), on ordinary documents:

- `probe_h4_cut_of_a_placement_of_an_instance`: the part is a
  `bolt ⊔ plate` seat; the host's product is `stood(instance) − tool`;
- `probe_h2_cut_of_a_pick_of_an_instance_piece`: a cut of a pick of an
  instance's piece.

The closure's premise, that every gathered copy is a live placement's
output, does not hold for a cut door's pieces over an instance's
copy. Fail-loud has become a panic: the check owes a typed finding or
refusal there, or the premise has to hold by construction.

## Done when

Both probes' documents run their checks without a panic, each with a
row.



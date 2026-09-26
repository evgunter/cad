---
id: run-out-carrier-evidence-rows-not-retaken
kind: issue
title: The two ignored interval evidence rows still pin the counts from before the run-out carrier key
status: open
opened: 2026-09-26
---


## The finding

The run-out carrier decision (`PendingRunOut::rides` in
`crates/profile/src/path.rs`, key `path_run_out_carrier`) adds one
decision per fillet run out the chain names. The interval tier proves
each of them zero: a run out lies on its arrival carrier by
construction. So `m10_9_pins_interval::measured_studies` moved
`symbolic_zero` by one on the bracket (1083 → 1084) and by three on the
pad (854 → 857). The gating row `m10_9_no_registrant_lies_on_any_measured_document`
checks those numbers and passes.

Two `#[ignore]`d evidence rows still carry the old counts and were
not re-taken in the PR that made the change. The session had no disk
left for the whole-box replays.

- `m10_9_pins_interval::m10_9_the_pads_four_at_both_dials` asserts
  the pad at `(858, 104, 991, 2750)` with rule F off and
  `(854, 128, 971, 2750)` with rule F on. The rule-F-on side is the
  same document at the same scale as the gating row. Its
  `symbolic_zero` is now 857. The rule-F-off side has not been
  measured. Its doc comment quotes the same numbers.
- `sym11_exact_channel_rows::PAST_THE_CEILING` pins
  `[symbolic_zero, registered, numeric, frozen]` past each ceiling.
  `r2_filleted_bracket` and `r2_rounded_pad` both have fillet run outs,
  so they are expected to move there too.

## Fix direction

Run both rows, `cargo test -p editor-core --test all -- --ignored
<row>`, at each ε row. Check that the only movement is into
`symbolic_zero`, by one per named run out, and that nothing leaves
`registered` or `numeric`. Then re-baseline the tables and the doc
comment that quotes them.

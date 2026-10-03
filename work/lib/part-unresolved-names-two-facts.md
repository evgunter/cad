---
id: part-unresolved-names-two-facts
kind: issue
title: lever_refusal_tag and resolve_fault_tag both publish part_unresolved for different facts
status: open
opened: 2026-09-29
---


## What

Found by D366's review (PR 3469), pre-existing on `origin/main`. Two
maps in `crates/pncad-py/src/tags.rs` publish the word
`part_unresolved`:

- `resolve_fault_tag` answers it for `ResolveFault::Unresolved`: the
  resolver found no document at the pin. `node_error_tag`'s
  `PartUnresolved` class and `inline` reach it through that map.
- `lever_refusal_tag` answers it, through `reach_refusal_tag`, for
  `ReachRefusal::PartUnresolved` inside `LeverRefusal::Reach`
  (`crates/editor-core/src/mate/reach.rs`; `LeverRefusal::PartUnresolved`
  until PR 3680), which carries a whole `eval::PartFault`: no resolver, a pin mismatch, an epsilon seam, a
  failed root, a reference cycle, and so on, as well as the seam's
  `Unresolved`.

So `part_unresolved` on a lever refusal means "this instance's part
did not evaluate", for any of the nine `Part*` classes, and on `EvaluationError` or
`inline` it means exactly one of those nine. A caller that learned the
word at one door reads the other wrongly. `SHARED_TAG_WORDS` counts the
word as shared, which the test's convention reads as one fact at two
doors; it is two facts.

## The fix this wants

Either the lever map projects the carried fault's own word (the
part's class through `node_error_tag`, as `mate_payload` already does
for a placer's refusal), or it gets a word of its own that does not
collide (`lever_part_unresolved`, with the fault's word on the inner
attribute). Either moves a published word, so it is LIB's call.

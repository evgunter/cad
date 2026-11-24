---
id: a-nodes-op-and-its-keys-walk-its-reads-separately
kind: issue
title: A node's op and its keys walk its reads separately: run_op re-resolves each member read that wire::seen_reads already resolved, and rows hold the two equal
status: open
opened: 2026-11-23
---


Left by the member-read-key unit (PR 7512), which made
`eval::wire::seen_reads` the one list a node's keys are derived from:
one `SeenRead` per read, holding the variable the read comes through,
the operation and port, the value the read sees (the entry that stands
in the results, the family whole, or the member an index picks) and
that member's place.

The op does not read through that list. `wire::run_op` resolves the
same reads again: `reads_projected` and `project_selection` project a
member read into a copy of the results, `wire_combine` and
`wire_assemble` call `NodeValue::whole` on a whole-taking seat,
`member_place` and `selection_table` look a selection's member up.
Both sides call the same resolvers (`member_read`, `selection_read`,
`selection_member`), so they agree on which member a read picks; what
no type holds is that the op reads past the value that stands ONLY
where `seen_reads` says it does. `pose::key_words` is a third walk of
the same reads, for the index values.

What holds it today: `member_read_key::every_read_past_the_value_that_stands_is_counted`
(a census of `NodeValue::whole`'s nine callers, all in `eval/wire.rs`)
and the rows of `member_read_key.rs` and `member_read_shapes.rs`.

**What one walk would take.** `run_op` takes the `SeenRead` list in
place of re-resolving: each `wire_*` reader takes its operand's value
from the list by seat rather than from `results` by operation, which
changes the signature of every reader that calls `at(slot, var)` and
`value_of(results, …)` today (about forty call sites in `eval/wire.rs`,
plus `pose::eval_posed`, `trace::drawn` and `mate`'s fold, which read
`results` for poses below a seat). A projected member (`member_projected`:
the member's body with the family's table projected onto it) would be
built once, where the list is built, and `NodeValue::family` and
`whole()` would then have no reader outside that function and could go.
Not local, so not done in PR 7512.

**Owed:** the op reading through the list, or the reason the two walks
stay, written at `seen_reads`.

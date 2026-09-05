---
id: prune-drops-a-hidden-instance-silently
kind: issue
title: prune discards a user's hide choice with no report, the same class as superseded and in the same function
status: closed
opened: 2026-09-04
branch: view/prune-report
pr: 1886
closed: 2026-09-05
refs: [opoutcome-superseded-has-no-production-reader, prune-discards-the-fault-that-explains-the-supersession]
---

Found by the VIEW-6 review (2026-09-04).

## What happens

`DisplayState::prune` (`crates/viewer/src/display.rs`) reconciles three
pieces of display state against a new document. Two of the three are
reported and one is not:

```rust
let dead_hidden: Vec<RecipeNodeId> = self
    .hidden
    .iter()
    .copied()
    .filter(|&i| display_check(doc, i).is_err())
    .collect();
for id in &dead_hidden {
    self.hidden.remove(id);
}
```

`dead_hidden` is computed, used to bump the revision, and dropped on
the floor. Nothing returns it, so **a hide the user chose is silently
undone and the part reappears** with nothing said.

`display_check` fails for more than deletion: it also fails with
`DisplayFault::FusedGeometry` when the instance's geometry is fused
into a boolean with others. So the case is not only "the thing you hid
no longer exists" — it includes "the thing you hid is still there and
is visible again", which is exactly the state a user would report as a
bug against the hide feature.

Compare the line above it: probes discarded by the same call ARE
returned, and since VIEW-6 they reach the status line
(`frame::supersession_notice`). The two sets are the same class of
fact, computed in the same function, one `Vec` apart.

## Why VIEW-6 did not fix it

Out of its fence: `display.rs` was under an open sibling lane
(`view/two-gestures`, PR #1873, which edits `prune`'s body), and the
fix changes `prune`'s signature, both `session.rs` call sites and
`OpOutcome`.

It is also **the blind spot VIEW-6's declared sweep could not reach.**
That sweep was "every public field of `OpOutcome`, grepped for readers
under `crates/viewer/src/`", which by construction cannot find a value
that never becomes a field at all. The sweep that finds this one is
over `prune`'s locals, or more generally over `Vec`s built and not
returned.

## What a fix looks like

Whatever shape `prune`'s report takes for
`prune-discards-the-fault-that-explains-the-supersession`, the hidden
set rides it: `OpOutcome` grows the fact, `frame` grows the notice,
and the two are ranked together. Worth deciding at the same time
whether re-showing a fused instance is a supersession at all or a
different sentence — the user's choice was not superseded by a
constraint, it stopped being expressible.

## Closed by PR #1886

The hidden set rides `PruneReport` as `dropped_hides`, `OpOutcome`
grows the field, and `frame::dropped_hide_notice` renders it.

**The open question is answered: re-showing a fused instance is NOT a
supersession**, and gets its own sentence. A supersession is a
substitution — the mate answers the placement question better than the
hand placement did. A dropped hide is superseded by nothing: the user
asked for the instance not to be *drawn*, and the document made that
question unaskable rather than answering it differently. The two arms
of `display_check` are why one sentence could not carry both — on a
fuse the part is drawn *again*, on a delete nothing reappears — so the
preamble says only what is true of both and the fault says which
happened.

`assembly_display::a_hide_the_picture_can_no_longer_honour_is_dropped_and_reported`
drives both arms through the session; before it, no row anywhere
covered a prune dropping a hide.

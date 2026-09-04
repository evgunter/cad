---
id: session-gesture-guard-spelled-thirteen-times
kind: issue
title: session.rs spells the gesture-in-flight guard thirteen times in two styles
status: parked
opened: 2026-09-04
refs: [1386]
blocked_on: [viewer-session-god-module-split]
---

Found by CHROME's style lane on PR 1746, answering the brief's Q8 —
the question that exists because nothing in this project's process
ever reads a whole file, so accumulation is invisible by
construction.

`crates/viewer/src/session.rs` opens every gesture-excluding op with

```rust
if self.gesture.is_some() {
    return OpOutcome::refused(Refusal::GestureInFlight);
}
```

**thirteen times, in two different styles.** Three are inline in
`perform`'s match arms (`DeleteNode`, `AddMate`, `AddInstance` —
`:2020`, `:2097`, `:2138`); ten are the first statement of a private
method (`:2233`, `:2260`, `:2423`, `:2433`, `:2461`, `:2474`, `:2487`,
`:2497`, and others). Which style a door uses tracks nothing about the
door.

**The finding is not the repetition, it is that no diff could show
it.** The file's own header calls it "the crate's accretion case
(#1386)" at `:929`. PR 1746 added 56 lines and one more instance of
this guard, and that was — correctly — unremarkable in its own review:
one guard in one door is right. The count only exists at the file
level, where no unit looks. Every instance is individually correct and
the set is a defect.

**Two questions a fix has to answer**, which is why this is filed
rather than swept:

1. Is "does this op refuse mid-gesture?" a property that belongs in a
   dispatcher-level table beside the op vocabulary, rather than a line
   repeated at each door? A table makes the answer readable for the
   whole enum at once, and makes a NEW op's omission visible — today,
   forgetting the guard on a new door is silent and looks exactly like
   deciding it does not need one.
2. Which ops legitimately have no guard, and is that recorded
   anywhere? The absence currently carries no evidence about whether
   it was decided or overlooked.

**Rides VIEW's split.** `work/chrome/plan.md` states that CHROME lands
before `viewer-session-god-module-split` ratifies and that anything
still open when it does rides the split. This is a `session.rs`
structure question, so it is the split's by construction rather than
by residue: re-home it to VIEW at CHROME's close rather than treating
it as a CHROME unit.

Signed: (CHROME orchestrator)

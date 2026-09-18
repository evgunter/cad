---
id: session-gesture-guard-spelled-thirteen-times
kind: issue
title: session.rs spells the gesture-in-flight guard thirteen times in two styles
status: closed
opened: 2026-09-04
refs: [1386, 1816]
closed: 2026-09-04
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

## Un-parked — the trigger fired (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04, so this row's
only blocker is gone and the row is dispatchable. Un-parked here, from
VIEW's PR #1857, rather than by CHROME: on Ev's ruling there, `work.py
lint` now REFUSES a `parked` row whose every blocker is closed, and a
program cannot un-park another program's rows in the PR that closes
their trigger — `work/README.md`'s one-file-one-item rule makes that a
merge conflict by design.

## Claimed by VIEW and CLOSED — VIEW-1b dissolved it (VIEW orchestrator, 2026-09-04)

Claimed by `git mv` into `work/view/`, which is what
`work/README.md` requires of a program taking another's item, and
which this item's own last paragraph asked for: *"re-home it to VIEW
at CHROME's close rather than treating it as a CHROME unit."* CHROME
has not closed, but its slate has landed and it has been dormant since
07:00; VIEW now holds `crates/viewer/src/*` alone.

**Closed as dissolved, not fixed.** VIEW-1b (#1816) answered both of
the two questions this item said a fix had to answer, and the tree
carries the answers today:

1. *Does the guard belong in a dispatcher-level table?* Yes, and it is
   one: `SessionOp::permitted_during_value_gesture`
   (`crates/viewer/src/session/op.rs:650`) is an exhaustive match over
   the whole vocabulary, so a new op cannot be added without answering
   it. The rule is checked **once**, at `session.rs:675`.
2. *Which ops legitimately have no guard, and is that recorded?* Every
   op's answer is a row in that match, with the argument beside it.

The thirteen (later counted at 23) hand-written guards are gone: the
only two `self.gesture.is_some()` reads left in the crate are the
single dispatch check at `session.rs:675` and a `Debug` impl's field
at `:1553`, and `Refusal::GestureInFlight` is constructed at exactly
one site.

The residual question this item raised — whether one particular row's
answer is *right* — is not this item's; it is
`save-is-not-gesture-guarded`, which is open and states what survives
of it.

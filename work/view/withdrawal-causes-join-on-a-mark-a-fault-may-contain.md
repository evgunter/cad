---
id: withdrawal-causes-join-on-a-mark-a-fault-may-contain
kind: issue
title: A Withdrawal joins its causes with LIST_SEPARATOR, which a DisplayFault's own text may contain
status: open
opened: 2026-09-15
---


Filed by the unit that closed
`joined-notices-nest-their-own-separator`, which fixed the OUTER level
— the boundary between two of a frame's notices — and left this one
level in.

## The shape

`Display for Withdrawal` (`crates/viewer/src/frame.rs`) renders a
preamble and then joins the withdrawal's causes, each a
`DisplayFault`'s own `Display`, with `frame::LIST_SEPARATOR` (`"; "`).
`DisplayFault::NonRigidFrame`'s text writes a `"; "` inside one
sentence (`crates/viewer/src/display.rs`, the `NonRigidFrame` arm), so
a cause list containing it reads as one item more than it has. That is
the same ambiguity-at-two the outer level had.

## Why it is not wrong today, and why that is the problem

No prune path produces `NonRigidFrame`. `DisplayState::prune` fills
every `Withdrawn.cause` from `free_move_check` or `display_check`
(`crates/viewer/src/display.rs`), whose `# Errors` sections name
`NoSuchNode`, `NotAnInstance`, `FusedGeometry` and `MateConstrained`
and not that one; `NonRigidFrame` is raised by `preview_free_move`
alone, which returns it to the caller rather than into a report.
Verified by reading all four, 2026-09-15.

So the inner join is unambiguous by a property of two functions' error
sets. **No type carries it.** `Withdrawn.cause` is `DisplayFault`,
the whole enum, so a sixth variant carrying a `"; "` — or a widened
`free_move_check` — re-opens it with nothing going red. The outer
level's fix is the contrast: there the mark is a boundary and only
`Message::joined` can write it, so the guarantee is the type's.

## What a fix would have to decide

Either `Withdrawn.cause` narrows to the faults the two checks
actually raise (the compiler then a census, and
`refusal-rank-wildcards-the-display-fault-payload` is nearby), or the
cause join gets the same two-half treatment the notice join got, or
`NonRigidFrame`'s sentence stops carrying the mark — which is the
cheapest and the weakest, because it is a claim about one arm rather
than about the type.

## Home

VIEW's: `crates/viewer/src/frame.rs`, `crates/viewer/src/display.rs`.

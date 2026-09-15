---
id: withdrawal-causes-join-on-a-mark-a-fault-may-contain
kind: issue
title: A Withdrawal joins its causes with LIST_SEPARATOR, which a DisplayFault's own text may contain
status: closed
opened: 2026-09-15
closed: 2026-09-15
branch: view/withdrawal-causes
refs: [joined-notices-nest-their-own-separator, startup-notices-join-on-a-mark-a-prefs-notice-contains, seat-line-spells-the-list-mark-as-a-literal]
pr: 2693
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

## Closed (`view/withdrawal-causes`, 2026-09-15)

**`Withdrawn.cause` narrows** — the first of the three, and the one
this file ranked strongest.

`display::AdmissionFault` is a new enum carrying the four faults the
admission tests answer; the arms MOVED there rather than being copied,
so `DisplayFault` gains `Admission(AdmissionFault)` beside its four own
and no sentence is defined twice. `drawn_targets`, `display_check` and
`free_move_check` answer `AdmissionFault`; every door still answers
`DisplayFault`, through `From` at the `?`. `Withdrawn::cause` is
`AdmissionFault`, so `Display for Withdrawal` joins a type the claim
can be made about.

**What this item claimed, checked by reading all four:**

- `prune` fills every `Withdrawn.cause` from `free_move_check` or
  `display_check`: **true**, at all three sites (`superseded`,
  `dropped_hides`, `killed_gesture`), and `prune` is the only producer
  of a `Withdrawn` in `src/`.
- the two checks raise those four and not `NonRigidFrame`: **true of
  the bodies**, not only of the `# Errors` prose. `free_move_check` is
  `display_check` plus `MateConstrained`; `display_check` is
  `drawn_targets().map`; `drawn_targets` returns `NoSuchNode`,
  `NotAnInstance` and `FusedGeometry` and calls only `doc.node`,
  `instances_by_root` and `BTreeSet` inserts, none of which can fail.
- `NonRigidFrame` is raised by `preview_free_move` alone: **true**, one
  construction site in the workspace (`display.rs`, the `is_rigid`
  else-arm), returned to the caller.
- `refusal-rank-wildcards-the-display-fault-payload` is nearby:
  **closed, at #2053**, and the narrowing touches what it bought.
  `Refusal::rank`'s `Display` arm now walks the admission family inside
  `DisplayFault::Admission` rather than folding it into one
  `Admission(_)`, which would have been that item's own defect one
  level further down. A fifth admission fault reds there too.

**What the fix does not buy.** The claim "no cause writes the mark" is
still a claim about four sentences; what the type carries is the
population it ranges over, so the census that checks it is over a
closed set and a fifth arm cannot join without one. `LIST_SEPARATOR`'s
other consumer is untouched and is a live defect:
`startup-notices-join-on-a-mark-a-prefs-notice-contains`.

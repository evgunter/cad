---
id: a-fifth-spelling-of-this-seat-is-empty
kind: issue
title: A fifth spelling of 'this seat is empty', in a file whose own doc says the sentence is composed centrally
status: open
opened: 2026-09-21
priority: P1
cost: D
---


## Finding

Found by AUTH-1's style reviewer (AUTHOR, PR 2955, 2026-09-21),
confidence `likely`. Filed on CHROME's slate because the subjects are
CHROME's ground — `crates/viewer/src/seats.rs`, `blend.rs`,
`pane/create.rs`, `pane/properties.rs` — and because this is the
entrenching-architecture half CHROME kept at the 2026-09-20 cut, not
one of the doors AUTHOR took.

**One idea, five spellings**, none of which knows about the others:

- `seat_line` → `"no picks yet"` (`crates/viewer/src/seats.rs`);
- `SeatError::Empty`'s `Display` → `"no {} picked yet"` (same file);
- `BlendError::NoEdges` → `"no edges picked yet"`
  (`crates/viewer/src/blend.rs`);
- `add_profile_ui` → `"pick a frame to draw on"`
  (`crates/viewer/src/pane/create.rs`);
- and, as of AUTH-1, `"none picked"` beside the face-frame row plus
  the two `DatumKindChoice::unmet_seat` sentences
  (`crates/viewer/src/forms.rs`).

**The file already argues against itself.** `seat_line`'s own doc says
it is composed centrally *"because it is the same vocabulary a
lost-pick notice is composed from, and two copies is how the two
drift"*. That sentence is the evidence this row rests on: the rule was
stated, and then four more copies were written anyway — which is the
project's standing shape, a duplication self-declared in prose at the
copy site that nothing ever read (`docs/prompts/reviewer-style-lane.md`
Q1).

## Class, not instance

The reviewer named it a class and named where else to look:
`crates/viewer/src/pane/properties.rs` (two sites), and the mate
tool's state lines in `pane/create.rs`. A taker sweeps those before
concluding the population is five.

Two rows already on this slate are the same SHAPE with different
subjects, and a taker should read them first rather than re-deriving
the argument:
`work/chrome/viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red.md`
and `work/chrome/four-spellings-of-one-finiteness-predicate-in-datums-rs.md`.
Whether the three want one unit is the taker's call; they are filed
apart because their subjects are.

## Why P1

`work/README.md`'s Priority: architecture that entrenches as things
are built on it — *"two implementations of one underlying logic"*. Each
new form adds a spelling, and AUTH-1 adding one while closing a
different instance of the same class is the demonstration.

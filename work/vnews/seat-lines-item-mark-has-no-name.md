---
id: seat-lines-item-mark-has-no-name
kind: issue
title: seat_line's item separator is an unnamed literal, and whether a panel line's mark deserves a name is undecided
status: open
opened: 2026-09-20
---



Filed by the unit that closed
`seat-line-spells-the-list-mark-as-a-literal` as not-a-defect. That row
was wrong that the mark belongs to `frame::LIST_SEPARATOR`; it was
right that something is unnamed.

## What is actually unnamed

`seats::seat_line` (`crates/viewer/src/seats.rs`) joins its per-seat
items with a literal `"; "`, and the mate panel
(`crates/viewer/src/pane/create.rs`, the `MateToolState::Two` arm)
writes the same two characters inside a format string. **Neither is a
notice**, so neither takes `LIST_SEPARATOR` — see the closed row for
why, and the doc comment at `seat_line` for the same argument at the
site. What they are is the items of a **panel label**, and that level
has no name in this crate at all.

So the mark is spelled twice in `crates/viewer/src/` with nothing
holding the two together, which is the two-copies-drift shape the
closed row correctly smelled and mis-sited.

## The question, undecided on purpose

**Does a panel line's item separator deserve a constant of its own —
a third level beside `NOTICE_MARK` and `LIST_SEPARATOR`?** This row
does not answer it. The arguments that have to be weighed:

- **For.** Two sites, one mark, no name. The crate already treats "one
  mark, two spellings" as a defect at the other two levels, and the
  README's two-marks section is the record of what it cost to learn.
- **Against.** The other two constants exist because a HOLD rests on
  them — the boundary mark is taken out of every text at
  `Message::new`, and the list mark's claim is carried by
  `AdmissionFault`'s closed population. A panel label has no hold and
  no invertibility requirement: nothing splits it back. A constant
  with no claim attached is a name, not an invariant, and this repo's
  own comment rule is that a comment states the invariant. A third
  constant could also read as a third LEVEL of the notice line, which
  it is not — it is outside that composition entirely — and minting it
  beside the other two is how a later reader re-mints the mistake the
  closed row made.
- **Or neither.** The two sites may not be one population: the mate
  panel's line is a hand-rolled copy of `seat_line`'s whole
  composition (`mate-panel-hand-rolls-the-seat-line`), so routing it
  through one composer would leave ONE spelling and no constant
  needed. Answering that row may dissolve this one.

**Take `mate-panel-hand-rolls-the-seat-line` first**, then ask this
over whatever is left.

## Home

VNEWS's: `crates/viewer/src/seats.rs`,
`crates/viewer/src/pane/create.rs`. `frame.rs` is not in scope — a
constant for this level does not belong beside the notice marks unless
the decision above says it does.

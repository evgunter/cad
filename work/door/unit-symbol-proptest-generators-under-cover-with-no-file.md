---
id: unit-symbol-proptest-generators-under-cover-with-no-file
kind: issue
title: expr.rs discloses that u8a_parse's two proptest generators silently under-cover the unit symbols, and nothing schedules it
status: open
opened: 2026-09-11
---


Filed by the DOOR orchestrator from the review of PR #2391 (style
finding S7), which found it disclosed in prose and scheduled nowhere.

## The disclosure

`crates/editor-core/src/expr.rs:296-300`, inside the "what an added unit
symbol costs" walk, states it plainly:

> `tests/u8a_parse.rs`'s two proptest generators enumerate the symbols
> by hand and do NOT go red; they silently under-cover, so they want an
> edit that nothing announces.

The two generators are `crates/editor-core/tests/u8a_parse.rs:482` and
`:725`, both `prop_oneof!` blocks.

## Why it needs a file

**It is a sentence in a source comment and nothing else.** Before this
file the only tracker row naming `u8a_parse` was
`work/door/dimension-all-has-readers-outside-the-viewer`, which reaches
those same two `prop_oneof!` blocks — but only their **Dimension** half,
and only as sites to convert to `Dimension::ALL`. The **unit-symbol**
half, which is what `expr.rs` discloses, is covered by neither.

So the defect has been disclosed at the site since before either row
existed, is described accurately, and is scheduled by nothing. That is
the exact shape `docs/prompts/implementer-discipline.md` §6 and
`work/README.md` both refuse: *"Disclosing a residue is therefore not
scheduling it — give it its own file at the moment you disclose it."*
The disclosure is honest and the schedule is missing; this is the
schedule.

## What it is not

Not a duplicate of the Dimension row above, and the two should not be
merged: they share two files and nothing else. One is a mirror of a
closed four-variant enum that a published `ALL` retires; this is a
generator over an OPEN, growing set of unit symbols, where the fix is
not a list to project but a way for the generator to draw from the
symbol table itself. A lane taking the Dimension row will be inside both
`prop_oneof!` blocks and should read this file before deciding how much
to touch.

## Not verified here

Whether the two generators are the only under-covering enumerations of
the unit symbols. `expr.rs`'s walk names the wire golden in
`switch_display_units.rs` as the thing that DOES go red on an added
symbol, so the population may be exactly these two — but that is the
source comment's claim, not a measurement, and the lane owes the sweep.

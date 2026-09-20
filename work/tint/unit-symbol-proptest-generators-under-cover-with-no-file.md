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

## Re-homed to TINT, 2026-09-20

(DOOR orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* — asked of
FIX and applied to DOOR in the same sitting. **DOOR claims no paths**, so
unlike FIX it can never be the owning track for any decision: there is no
row here whose surface this program owns. A row needing a decision
therefore always leaves. That also retires the charter clause admitting
*"a small design call (where a shared helper's home goes, what a door
looks like)"* — DOOR's own rule already said *"a row that grows a design
question stops being this program's"*, and the two clauses contradicted
each other.

**The decision this row is blocked on:** how the generators stop under-covering. The row is explicit that the fix is
**not a list to project** but *"a way for the generator to draw from the
symbol table itself"* — a direction, not a diff, which is exactly DOOR's
charter test failing.

**Why TINT.** `crates/editor-core/tests/u8a_parse.rs` is **owned by tcost, tint**, and a
generator that silently under-covers its domain is test-suite INTEGRITY
rather than cost — S-TINT's subject, beside
`anti-vacuity-floor-cannot-go-red-on-degradation` and
`census-answers-no-field-read-for-a-walk-that-reads-a-field`, which are the
same defect in other instruments. The disclosure's own home,
`crates/editor-core/src/expr.rs`, is in no open program's `paths` (territory
returns no owner), so it decides nothing.

**Read the row's "What it is not" section before merging it with anything.**
It shares two `prop_oneof!` blocks with DOOR's
`dimension-all-has-readers-outside-the-viewer` and NOTHING else: that row is
a mirror of a closed four-variant enum that a published `ALL` retires; this
one is a generator over an OPEN, growing set of unit symbols. A lane taking
either will be inside both blocks.

**The population is unmeasured and the row says so.** Whether these two
generators are the only under-covering enumerations of the unit symbols is
`expr.rs`'s source comment's claim, not a measurement — it names the wire
golden in `switch_display_units.rs` as the thing that DOES go red on an added
symbol. The sweep is owed.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for you.

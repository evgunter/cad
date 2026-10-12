---
id: a-feature-an-operation-consumes-has-a-lint
kind: issue
title: lint: a feature an operation consumes (a blend's band or a Boolean's tool swallows it whole) has a lint of its own, separate from unproven-coincidence and independently enabled
status: open
opened: 2026-10-12
priority: P3
cost: M
design: true
---


From Ev on `[ev]` PR 4285 (2026-10-08), which ruled that a feature wholly
inside the material a fillet or chamfer band removes or fills goes with it,
silently at the verb, as a feature wholly inside a Boolean's tool already
does (`BooleanNaming::discards`):

> "i think it could be good to add a new lint--a separate one from the
> coincidence one, which can be enabled or disabled independently--for
> this and the boolean case as one. i guess we might need to figure out how
> lints store their own resolution before that's implemented though (or
> like at least lints probably have to store if they were declined)"

**What it is.** One lint over every verb that consumes an operand feature
whole: an operand entity with no piece in the output. It is derivable from
the name tables: the Boolean's `discards`, and a blend's faces its
predicate-2 verdict reads "wholly inside", once BAND builds that
(`work/band/ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver.md`).
It is separate from `unproven-coincidence` and can be enabled or disabled
on its own.

**Open (design, per Ev's note).** How a lint stores its own resolution: at
least whether a finding was declined, so a consumed feature the author
meant to remove is not re-raised on every evaluation. That storage is
D10's lint machinery, which is why this row is INTENT's and not BAND's or
COINCIDE's. Weigh it with the other lints before building.

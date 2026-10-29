---
id: a-decided-zero-tangency-tells-the-frontier-story-where-recourse-iv-would-escalate
kind: issue
title: A root door's decided Zero (an exact tangency) tells the frontier story, where [tolerance.recourse] (iv) reads it as its in-band sibling's escalation: which story DESIGN owes is open
status: open
opened: 2026-10-29
priority: P2
cost: M
design: true
---


Raised by PR 5993 (`roots-unsettled-folds-band-precision-and-fault-into-one-frontier`) and its DUAL review (2026-10-29). The fix pass reverted the PR's call (d) to main's behaviour, so this row records the open question; it does not record a defect.

## What

When a root door reads an exact tangency (its discriminant decided Zero, not in band), main ends it as the typed frontier, `CurvedPierceUnsupported`. PR 5993's lane escalated it like its in-band sibling instead, which moved about 20 pins, the Steinmetz family among them. Both reviewers found that DESIGN.md does not settle which story is owed:

- **For escalating:** `[tolerance.recourse]` (i): "User-facing messages and recourse never fork on exactly-on vs in-band below ε_input". (iv): "'Definite' means the reading certainly lies on a refused side, whether the band, a number or a proof decided it, so a proved zero tells the story of its exact and in-band siblings."
- **For the frontier:**
  - (iv) applies only to a reading that "certainly lies on a refused side".
  - `[coincidence.booleans]`: "Where its verdicts decide Zero, a union, a cut or a meet glues where an arm exists for the carrier pair ([topology]'s frontier) … each refuses what falls in the sliver band".
  - `[verdicts.consumers]`: for a structural act, "Zero acts and is recorded …, in band refuses with the decision's own lever".
  - `[loud-failure]`: valid input the kernel has not built gets a typed `Unsupported*`.
  - PR 4335's honesty rule: an escalation must not tell the user their input is ill-conditioned when the kernel's arithmetic is. Its exact wording is not in DESIGN.md or the READMEs.
  - `refusal_routes.rs`' per-decision ending docs record the frontier reading.

**The cost of escalating, demonstrated by R1.** In the Steinmetz pair, the seams extrusion places make the decided-Zero reading escalate: the user is told to "move the parts so the edge clearly crosses the wall". Yet the same solids, with each cylinder spun about its own axis, build at the closed form 8π − 16/3. So the touch was a seam the kernel placed, not the user's geometry. R2 also found a cone root door whose decided-Zero discriminant lies outside the segment's span; it escalated about geometry half a metre clear.

## Owed

This is a design fork. It goes to an Opus/Fable designer pair (`docs/DESIGN-FORK-PROTOCOL.md`), then to Ev.

One candidate shape: escalate a decided Zero only where its witness is the user's geometry, and keep the frontier where the touch is at a kernel-placed seam or outside the span.

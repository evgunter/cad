---
id: the-meeting-pair-leaf-mints-a-sliver-without-the-terminal-rule
kind: issue
title: clearance's meeting-pair leaf mints EngineRefusal::Sliver for any undecided reading, bypassing terminal()
status: open
opened: 2026-11-06
priority: P4
cost: E
---


## Finding

Two scopes that meet are one leaf of the clearance sweep: `0` read
against each bound at `crates/topo/src/clearance.rs:1542`
(`q.read(open, zero)`). Every `Reading::Undecided` there becomes
`EngineRefusal::Sliver` (`:1545`–`:1549`), whatever the escalation
says. The sweep's own rule for an undecided reading is `Bound::terminal`
(`:1882`): a poison margin is `PoisonEnclosure`, a terminal sliver
(`Indeterminate::terminal_sliver`, read through `sliver` at `:883`,
documented as the one home of what a sliver is) is `Sliver`, and
anything else is `None`, left to a split. The meeting leaf skips both
tests:

- a poison margin (`MarginDiag::is_invalid`) is refused as a `Sliver`,
  not as `PoisonEnclosure`;
- an escalation the classifier did not mark terminal is refused as a
  `Sliver`, which `Sliver`'s doc (`:482`) defines as an enclosure wholly
  inside the band.

Neither is known to be reached today. The margin is `[0, 0]` minus a
bound the query door validated, so it is finite, and a degenerate
enclosure strictly inside the band reads `terminal_sliver: true`. The
reason for a terminal refusal here is the comment's ("no refinement
moves a distance topology fixed at 0"), which is about the leaf, not
the enclosure. The difference shows only at the edge: an outward-rounded
`0 - bound` that touches a band edge, or a bound that is not finite.

## Why not fixed in the lane that found it

Routing through `terminal(source, x.face, y.face)` handles poison, but
its `None` arm has nowhere to go on this leaf: there is nothing to
split, and today's code answers `Sliver` there. Choosing between
"`None` is unreachable here" (a proof that the margin is a degenerate
interval strictly inside the band) and "a meeting leaf's undecided
reading is terminal by topology" (and saying so in `Sliver`'s doc or
in a new arm) is the engine's call. Found by the review of PR #6543.

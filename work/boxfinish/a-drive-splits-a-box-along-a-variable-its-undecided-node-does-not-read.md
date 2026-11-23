---
id: a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read
kind: unit
title: A drive splits a box along its widest variable, though the undecided node may not read it
status: closed
opened: 2026-11-22
priority: P1
cost: M
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
branch: boxfinish/split-reads
pr: 7458
closed: 2026-11-23
---


Point 4 of the parent row's weighed state, ratified on `[ev]` PR 7368
(`docs/ERROR-DESIGN.md` E6's split rule): of the variables the leaf's
deciding node reads that have depth left, the widest, ties to the
lowest index; a deciding node that reads no variable of the box
refuses the box at once with a reason of its own that names it; a read
set at its depth budget or at the grid stays `Budget`. The order rule
that picks which node speaks first (`drive::classify_replay`) becomes
load-bearing and says so.

Measured (M8): on the tour's study 292 of 511 splits are on a variable
the deciding node does not read, and the tree that splits only where
it reads has 127 replays of 1,023. Independent of the parent's other
units: nothing here needs a node result to cross a replay.

**Letter M; dual tier** (the rule every drive's leaf tree follows: a
read set that misses a variable refuses a box a split would have
decided, and one too wide splits for nothing, and neither shows as a
red row unless one is written for it). Sequential, per the model
comparison's protocol.

## Closed

PR 7458 (2026-11-23). `drive::bisect` cuts an undecided leaf on a
variable its deciding node reads (`Doc::free_reads`), the relatively
widest with depth left, ties to the earliest declared; a deciding node
that reads no variable of the box is refused
`RefusalReason::ReadsNoVariable { node, unsettled }`, which says
whether the node was undecided or failed and with what.

**Corrected: what this file said the rule buys.** The headline above
was "57% of the tour's splits change nothing", and the body's "1,023
replays where 127 would do" is M8's trace with unread splits
collapsed, taken on the probe tree. Measured by the lane on main and
on the probe tree: while the leaf budget binds the rule removes no
replay (the tour's drive is 512 leaves before and after). It shortens
a drive that would otherwise end (a loft: 1 replay against 511; a
two-parameter plate at 64 leaves: 49 against 127) and localizes one
that does not (the plate at 16 leaves certifies 0.9630 of its mass
against 0.8828).

Not closed by this, each with its own file:

- A replay refused whole speaks through its first node:
  `a-leaf-refused-whole-speaks-through-its-first-node`.
- "No split changes it" is exact only once a node's result reads no
  per-replay session state:
  `a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`
  (PR 7492).
- Other subdividers that refine without asking what the undecided
  thing depends on: seven rows filed by the lane on their owners'
  slates (SSIEDGE, SPLICE, CPAIR, ENCL, BAND, QUAD, CLEAR).

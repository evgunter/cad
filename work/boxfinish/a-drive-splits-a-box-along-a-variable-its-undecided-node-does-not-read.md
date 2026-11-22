---
id: a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read
kind: unit
title: A drive splits a box along its widest variable, though the undecided node may not read it: 57% of the tour's splits change nothing
status: open
opened: 2026-11-22
priority: P1
cost: M
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
branch: boxfinish/split-reads
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

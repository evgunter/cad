---
id: a-face-runs-rounds-it-never-intersects
kind: issue
title: A face's rounds are independent sound enclosures of one quantity, and nothing intersects them
status: open
opened: 2026-10-21
priority: P4
cost: E
---


Raised off-question by a designer weighing `quad2-rational-max-rounds-dial-decision`
(2026-10-21).

Every round of a patch lane recomputes the face's flux and area from scratch
and gives a sound enclosure of the same quantity. So the intersection of every
round a face ran is also sound, and it can be narrower than the last round
alone, where an earlier round's pads happened to fall tighter on one side.
Nothing takes it today: `props::quad` and `topo::props` read the last round
only.

Owed: intersect the rounds a face has run before reporting or deciding a sign.
A pin shows the intersection is narrower on at least one face, or the row
records that it never is, and why.

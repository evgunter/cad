---
id: a-derived-pick-index-failure-outshouts-its-cause
kind: issue
title: viewer: a pick-index failure caused by a failed node is the loud banner, and the node's own refusal is the quiet line
status: closed
opened: 2026-09-17
priority: P1
cost: D
closed: 2026-09-25
---

**Ev reported this** (in chat, 2026-09-17), from unioning the two
halves of a local dumbbell document (a git-ignored
`demos/tour/gallery/dumbbell.pncad`). The top of the window showed:

> pick index: root 11's bodies could not be tessellated or indexed:
> hit test: node 11 failed, so it has no name table to invert — fix
> the node's own failure before picking against it

and "much quieter", lower down, the actual cause: node 13's Boolean
refusal (a torus×plane operand pair, which is CURVED's
`c5-plane-torus-cone-cylinder-arms` / `torus-operand-gate-admission`).

The loud message is a CONSEQUENCE of the quiet one: the pick index
cannot invert the name table of a node that failed to evaluate
(`viewer`'s `pickindex.rs`, the "could not be tessellated or indexed"
arm; `editor-core`'s `resolve/hit.rs`, "no name table to invert"). A
downstream effect of a failure the user already has in front of them
should not outrank that failure. Either drop it, when the failure it
depends on is already shown, or place it under that failure. It also
names a different node (root 11) from the one that failed (13), so a
reader cannot tell the two messages are about the same event.

Not investigated beyond locating the two message sites.

## Closed 2026-09-25 — placed under its cause

**Reproduced**, headlessly, by replaying `dumbbell.pncad`'s edit log
through `DocSession` and `PickCache::inline`: inserting node 11 (the
torus×plane Union) gives exactly the quoted badge, and the tree's row
11 is `FAILED` with the Boolean's refusal.

**Defect 1, the ranking.** `frame::index_badge` drew every pick-index
refusal `Tone::Actionable` in the index's own words. A refusal that is
`NodePickError::Standing` on a root the tree badges `Failed` or
`Poisoned` is now `Tone::Advisory` — the tree's own tone for a
downstream row — and reads *"pick index: waits on feature N, which
failed — no pick is answered and the picture is not redrawn until it
builds"*, with the index's words kept as the tooltip. It is placed
under the cause rather than dropped because it says two things the
cause does not: every pick is refused, the healthy roots' included,
and the viewport keeps its last picture (the scene is drawn from the
index). Any other refusal, and a standing one the tree names no failed
row for, keeps `Actionable` and its own words. Row:
`frame_policy::a_refusal_that_follows_from_a_failed_node_is_quieter_than_it_and_names_it`.

**Defect 2, the node named.** The guess that `resolve/hit.rs` cannot
see a poisoning is false: `HitTestError::NodePoisoned` names `through`,
and `NodePick`'s `standing_value` produces it. In the replay every
refusal named the node that had itself failed — root 11 while 11 was
the failing Boolean, root 13 while 13 was — and 11 and 13 were never
live at once in the saved log (11 was deleted, 12 inserted and deleted,
then 13 inserted), so the two messages Ev saw together were not
reproduced as simultaneous. What IS fixed is the naming seam: the badge
now names `tree::cause_row` — the row the tree badges `FAILED`,
through a poisoning or a mate refusal — in the tree's own spelling
(`tree::node_number`), so the badge and the row cannot name different
nodes for one evaluation. Nothing filed on EDIT.

**Filed**: `work/fit/one-failed-root-refuses-the-whole-pick-index.md`
(why the refusal exists at all — one failed root takes picking and the
redraw of every healthy root with it) and
`work/vnews/the-at-rest-badge-restates-a-failed-root-louder-than-its-row.md`
(the sweep's one live sibling).

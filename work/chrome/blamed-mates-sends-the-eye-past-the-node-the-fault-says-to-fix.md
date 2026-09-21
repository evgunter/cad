---
id: blamed-mates-sends-the-eye-past-the-node-the-fault-says-to-fix
kind: issue
title: blamed_mates sends the eye to the mate where three fault arms name the node an author actually fixes
status: open
opened: 2026-09-15
priority: P1
cost: D
---


Found by CHROME's `chrome/band-refusal-badging` lane, as the class
residue of sweeping `MateFault` for arms that name no mate. The sweep's
other half — arms that name a mate **and something else** — turned this
up, and it is not the band row's to fix.

## The finding

`crates/viewer/src/tree.rs`'s `blamed_mates` answers `vec![*mate]` for
every arm that carries a `mate` field. Three of those arms carry a
SECOND node id, and the kernel's own doc comments say that second node
is the one an author goes and repairs:

- `MateFault::PlacerRefused { mate, placer, .. }` —
  `crates/editor-core/src/mate.rs`, the `placer` field: *"the node an
  author goes and fixes"*, in those words.
- `MateFault::PartSelectsAnotherCopy { mate, part, .. }` — `part` is
  *"the `Part` node whose index expression disagrees"*; the mate is
  merely where the disagreement was noticed.
- `MateFault::DanglingHead { mate, head, .. }` — `head` is *"the node
  at which the reference resolves to no member"*, and the same doc says
  the mate's own head is *"often live and fine"*.

So for these three the tree badges the mate `Failed` and draws the
named node's row — when that node has a row at all — as quiet.

## Why it may still be right, which is why this is a row and not a fix

For `PlacerRefused` the kernel argues the opposite way in the same
file: a mate fault poisons the document, so the placer never evaluates
and *"this fault is the only place that cause appears"* — the mate's
row carries the placer's typed refusal verbatim, so the eye lands on
words to read. `Poisoned`'s walkable-in-one-hop invariant also holds
only for a row THIS TREE badges `Failed`, and the placer's row is
`Unevaluated` or poisoned, not failed — so pointing at it would need
more than a one-line change to `blamed_mates`.

What is not settled is whether a reader who is told "mate 7 refused:
pattern 3's count does not evaluate" can find pattern 3's row, and
whether the tree should say so structurally rather than only inside the
payload's sentence.

## Why the class is three of five, and not five

`Under { mate, parent, child }` and `SelfMate { mate, instance }` also
carry node ids beside the mate, and they are deliberately NOT in the
list above. The test is not *"does the arm carry a second id"* — it is
*"does the kernel's own doc call that id the thing an author
repairs"*. For the three named arms it does, in those words. For these
two it does not, and the ids are a different kind of thing:

- `Under`'s `parent` and `child` are **the two instances the fold was
  extending between** — evidence about WHERE the residual survived, not
  a node to go and edit. What an author changes is the mate's own
  constraint, or the recourse `UNDER_RECOURSE` names.
- `SelfMate`'s `instance` is **the instance named twice**, and it is
  named twice BY the mate. The repair is on the mate's references; the
  instance is fine and editing it would be editing the wrong node.

So the sweep's stopping point is a judgement about the second id's
role, not an oversight — stated here because a reader counting arms
with second ids gets five and should be able to see why two are out.
Whoever takes this row should re-test that judgement rather than
inherit it.

## A tension with the tree's own header, for whoever writes the fix

`crates/viewer/src/tree.rs`'s module header says a mate refusal's
subject *"is a mate node"*. This row argues that for three arms the
node an author repairs is NOT the mate. Both can be true — the
*blamed* node and the *repaired* node need not be one — but the header
does not say so, and that sentence is already on its third generation
(it has been rewritten once for the `Band` carve-out and once before
that). If this row is taken up, the header's claim is part of its
subject, not a neighbour to leave alone.

## Where to look

`crates/viewer/src/tree.rs` — `blamed_mates` and `downstream_of_mate`.
The three arms' own documentation in
`crates/editor-core/src/mate.rs` (MSOLVE's ground; nothing here asks
for a kernel change). Coverage today: `crates/viewer/tests/
msolve3_placer_refused.rs` exercises `PlacerRefused`, so whichever way
this is decided there is a row to change rather than one to write from
nothing.

Signed: (CHROME implementer lane, `chrome/band-refusal-badging`)

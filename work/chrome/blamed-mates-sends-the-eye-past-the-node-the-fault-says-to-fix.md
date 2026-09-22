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

## Re-tested 2026-09-22 (`chrome/badge-attribution`, PR 3090) — three of five is one of five; that one is a question

**Two of the three arms are not in the class; the row's premise is
false for them**, read off `crates/editor-core/src/mate.rs`'s
`enum MateFault` and its `Display`:

- `DanglingHead`: the arm's doc says the node the walk stopped at
  *"may be perfectly live"*, and its `Display` names the recourse —
  *"— rebind it"*, a change to the mate's reference. `solve::undecided`
  calls deleting the offending mate *"the recourse every such refusal
  names"*. The quotation this row carries (*"the mate's own head is
  often live and fine"*) is about the REFERENCE's head, not about the
  `head` field.
- `PartSelectsAnotherCopy`: the kernel refuses *"rather than choosing"*
  between the name and the `Part`; neither is the one to fix.
- Measured: both reached the way a user reaches them (the named node
  edited after the mate was authored), the named node's row is `Ok`.
  A `Poisoned` row may only point at a row the tree badges `Failed`,
  so blaming it would point at a green row. Pinned by
  `msolve3_placer_refused.rs`'s
  `a_stranded_copy_blames_the_mate_and_not_the_pattern_it_stopped_at`
  and `a_part_selecting_another_copy_blames_the_mate_and_not_the_part`,
  each proved red by moving the blame to the named node.

`Under` and `SelfMate` re-tested too: out, for the reasons above.
`tree.rs`'s module header now says the blamed node is the row carrying
the fault's words and gives the per-arm reason; the claim that a
subject *"is a mate node"* is gone.

**Open: `PlacerRefused`.** The kernel does call the placer *"the node
an author goes and fixes"*. Where the refusal reaches the placer's
row, that row is `Poisoned` (through an instance the tree redraws as
downstream of the mate; `the_mate_row_names_the_direction_and_not_a_dangling_head`
measures it), and the mate's `Failed` row carries the words. Two
defensible answers, and they change which row is loud:

- **(a) Status quo.** The mate is `Failed` with the placer's refusal,
  naming it by number; the placer points at the mate; the eye goes
  mate → reads "node P refuses — …" → P. Nothing is drawn that the
  evaluation did not record.
- **(b) The tree draws the placer `Failed`** from the fault's own
  `error: NodeRefusal` — the placer's typed refusal, carried
  unaltered — and points the mate and the cluster's instances at it.
  The eye lands on the node to fix; but the tree then draws a
  failure for a node the evaluation reports as never run, which the
  module header's *"invents none of them"* has to be re-argued for.

A cheaper middle is a structural link from the mate's `Failed` row to
the node its payload names, which changes nothing about which row is
loud. Where the placer fails in its own right (a count that does not
evaluate), both rows are `Failed` already and nothing is open.

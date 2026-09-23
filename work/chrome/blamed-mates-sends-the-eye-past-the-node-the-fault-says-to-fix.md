---
id: blamed-mates-sends-the-eye-past-the-node-the-fault-says-to-fix
kind: issue
title: blamed_mates sends the eye to the mate where three fault arms name the node an author actually fixes
status: open
opened: 2026-09-15
priority: P1
cost: E
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

## Re-tested 2026-09-22 (`chrome/badge-attribution`, PR 3090) — decided by reach for every arm but one raise path of `PlacerRefused`

**What decides it is where the solve records the fault**
(`crates/editor-core/src/mate/solve.rs`, `solve_with_env`):

- **Raised where the solve reads one mate's references** —
  `mate::member::check_reference` and the walk, collected as `broken`
  and inserted with `out.faults.insert(mate, fault)`: every
  `DanglingHead`, every `PartSelectsAnotherCopy`, and a
  `PlacerRefused` whose placer's own slot does not evaluate. **The
  fault reaches the mate only.** So `blamed_mates`' answer for these
  decides one row — whether the mate reads `Failed` with the words,
  or `Poisoned`, pointing away from the only row that has them. There
  is nothing to redirect: blaming the named node could only demote
  the cause's row.
- **Raised while a cluster's fold derives an offset** —
  `mate::member::derived_offset`, called by the fold: a
  `PlacerRefused` only. It fans out to every instance and mate of the
  cluster, and those rows point at the mate.

The kernel's per-arm docs agree, as a second argument: `DanglingHead`'s
`head` *"may be perfectly live"* and its `Display` names the recourse
*"— rebind it"*; `PartSelectsAnotherCopy` is refused *"rather than
choosing"*. (This row's quotation *"the mate's own head is often live
and fine"* is about the REFERENCE's head, not the `head` field.) The
named node's own row is whatever the evaluation says of it:
`Ok` in the stranding fixtures, and `Failed` beside the mate — both
loud, neither pointing — when the node fails in its own right (a `Part`
indexed past its pattern's count; a pattern of zero copies). Pinned by
four rows in `crates/viewer/tests/msolve3_placer_refused.rs`, each
proved red by moving the blame to the named node.

`Under` and `SelfMate` re-tested too: out, for the reasons above.
`tree.rs`'s module header carries this as the one statement of why;
`blamed_mates`' doc points at it.

**Open: `PlacerRefused` on the `derived_offset` path only.** The
kernel calls the placer *"the node an author goes and fixes"*. On the
`check_reference` path the fault reaches the mate alone and the placer
fails in its own right, so both rows are `Failed` and nothing is open.
On the `derived_offset` path the placer sits on the chain above an
instance the fault reached, so the evaluation POISONS it even when its
own slots are broken (`the_mate_row_names_the_direction_and_not_a_dangling_head`
measures a pattern with a `1e200` direction drawn `Poisoned` through
the mate); its row carries the pointer, and the mate's `Failed` row
carries the placer's refusal verbatim. Two defensible answers, and
they change which row is loud:

- **(a) Status quo.** Mate `Failed` naming the placer by number; the
  placer and the cluster's instances point at the mate. Nothing is
  drawn that the evaluation did not record.
- **(b) The tree draws the placer `Failed`** from the fault's own
  `error: NodeRefusal` — the placer's typed refusal, carried
  unaltered — and points the mate and the cluster's instances at it.
  The eye lands on the node to fix; the tree then draws a failure for
  a node the evaluation reports as poisoned, which the module
  header's *"invents none of them"* has to be re-argued for. Changes
  nothing on the `check_reference` path.

A cheaper middle is a structural link from the mate's `Failed` row to
the node its payload names, which changes nothing about which row is
loud. Related, filed on MSOLVE's slate: `check_reference` sites a
`Part`'s non-evaluating index at the pattern below it, so on that path
the named placer can be the wrong node
(`work/msolve/placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate.md`).

## Ev's ruling on Q2 (in chat, 2026-09-23) — option (c)

Shown the concrete case (a mate onto a pattern copy whose direction
slot does not evaluate: the mate row carries FAILED and the message
naming the pattern; the pattern row is poisoned and points at the
mate) and three options — (a) leave it, (b) draw the placer `Failed`,
(c) keep the blame on the mate and make its message LINK to the node
it names — Ev answered: *"(c) makes sense!"*.

So the work this row owes is now small and written: blame stays where
`tree::blamed_mates` puts it; a `Failed` mate row whose fault names a
placer (`MateFault::PlacerRefused`'s `placer`) gets a link to that
node's row, the way a `Poisoned` row's pointer already links to
`through`. The row's other arms need no link: D1 of PR 3090 showed the
nodes `DanglingHead` and `PartSelectsAnotherCopy` name are not what an
author fixes.

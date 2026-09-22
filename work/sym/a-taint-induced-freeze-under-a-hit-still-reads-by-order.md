---
id: a-taint-induced-freeze-under-a-hit-still-reads-by-order
kind: issue
title: A taint-induced freeze under a memo hit is still read by order
status: open
priority: P2
cost: D
opened: 2026-09-22
refs: [SYM-13, 3054]
---

Filed by SYM-13 as it landed the leaf receipt's NEED column, against
the residue its first reviewer's delta found — not a defect in the
drive-scoped plain memo Ev ratified on `[ev]` #2581, whose own header
has named this branch since SYM-7.

## What

A leaf's `frozen` column is its NEED: the drive's frozen set inside
the closure of its plain-walk roots, unioned with the ids its table
does not hold (`Session::foreign`) and the freezes it made and could
not publish. The first two are functions of the leaf's box. The third
is not, in exactly one case:

**a freeze the TAINT caused — a recorded node whose form the leaf
builds out of an unrecorded one, which fits the budget for a leaf that
RECORDS the node and does not for this one — reached in one order and
skipped in another.** No leaf publishes such a freeze (the taint guard
keeps it in the leaf), so the drive's set cannot carry it, and the
leaf's table cannot predict it: whether the leaf makes it at all
depends on whether the memo served it a form at an ancestor first.

Measured at the tier's own door
(`geom-core/tests/sym_drive_memo::a_taint_induced_freeze_under_a_hit_is_read_by_order`,
which pins the reading): `outside = 0.0` minted outside the session,
`sum(x) = x + a + b` at a two-term budget. A leaf that records the
zero folds it away — two terms, no freeze, and it publishes the form.
A leaf that does not carries an indeterminate in its place — three
terms, frozen. Run first that leaf reads `frozen 2`; run after the
recording leaf it takes the published form at the decision root, never
walks there, and reads `frozen 1`. No drive's frozen set is non-empty
in either order.

## Why it is not urgent

It is inside the branch `geom_core::sym::memo`'s header already
discloses and that `editor-core`'s
`no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded` pins at
zero over five drives — both measured documents and all three
adversaries, `FreezeCause::Unrecorded` zero in every one. A drive
mints every node inside its own session, so no leaf of a drive holds a
foreign id at all; reaching this needs the scalar door and a node
minted before the session was installed.

The same branch is where a leaf's DECISIONS are order-dependent too,
and that is the older disclosure: SYM-13's first reviewer demonstrated
both halves — `the_inherit_branch_moves_the_decision_while_the_column_stands_still`
(the decision moves, the column does not) and
`an_inherited_form_does_not_move_the_leafs_need` (neither moves). What
this row adds is the one shape where the column moves alone.

## The options, none taken here

- **Compute the counterfactual**: count what the leaf's plain walk
  WOULD have frozen with no memo installed. That is the walk the memo
  exists to skip, so it costs the unit's whole saving.
- **Drop the leaf's own unpublishable side** and count only the
  drive's set and the foreign ids. Then this case reads 1 in both
  orders — but `a_tainted_freeze_is_the_leafs_own_need_in_either_order`
  starts reading by order instead, because a tainted freeze the
  recording leaf ALSO makes is published in one order and not yet in
  the other. It trades one shape for a more reachable one.
- **Refuse the hit under a foreign id**: a leaf that holds a foreign
  id could decline the memo for every node above it, which would make
  its reasoning the memo-free one throughout. That is a change to the
  memo's ratified design (#2581) and costs hits on a document that
  mints anything outside its session; it wants Ev's word rather than a
  lane's.

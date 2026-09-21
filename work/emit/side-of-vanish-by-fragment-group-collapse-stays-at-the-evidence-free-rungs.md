---
id: side-of-vanish-by-fragment-group-collapse-stays-at-the-evidence-free-rungs
kind: issue
title: A SideOf fragment name that vanishes because its fragment GROUP collapsed (the partners unmoved) is not recovered by the shadow-exec rung and falls to the evidence-free RecipeEdit fallback
status: open
opened: 2026-09-16
refs: [2755, 134]
priority: P1
cost: H
---

Found by BOOL-7's R1 review (PR 2755) and filed by the S-BOOL
orchestrator at the unit's merge. Issue 134's recovery rung fires when
a vanished `SideOf` fragment name's minting node recorded no
`name_frag_side_of` verdict in one of the two runs, and re-executes
the pair per partner against the boolean's operands. Two events empty
that population: the C10 sweep PRUNING the pair (the bar withdrawn
clear of the plate), and the fragment GROUP COLLAPSING to a single
fragment while the partner walls stay where they were relative to the
survivor (the bar slid along the cap by 2.5 or 3.5 in y). In the
second case no partner's side verdict changes — the rung, correctly,
finds no flip and returns None — so the vanish falls to
`RecipeEdit { NodeChanged }`, the cause-not-in-evidence answer. The
rung's docs, `FlipSource`'s docs and vdiff.rs's blind-spot paragraph
now state this as a limit, and a row pins it. What would recover it is
a different statement than a predicate flip: the group's cardinality
moved (three fragments became one) with every side verdict intact —
a `GroupCollapsed { was, now }` diagnosis read off the two name
tables, not off any verdict log. Names-lane design surface, adjacent
to the OrderAlong-partner item on WIRE's slate. Measured, not acted
on; difficulty S–M.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to WIRE (the names lane is WIRE's territory) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

---
id: pick-and-pickindex-are-named-against-their-contents
kind: issue
title: the module called pick holds no picking, and the policy it claims is in pickindex
status: open
opened: 2026-09-06
refs: [2079, pickindex-holds-the-frames-marks-as-well-as-the-index]
---



Found by the style review of #2079.

## What

`crates/viewer/src/pick.rs:1-2` opens *"Keeping a pick index current
with the session, and refusing a pick when there is none — **the policy
half of picking**"*, and `:21-29` explains that the index is elsewhere
because *"the index and this policy are two layers"*.

The picking policy is not in `pick.rs`. It is in `pickindex.rs`:

- `PickIndex::op_for` / `op_under` (`pickindex.rs:1489`, `:1518`) turn
  a `PickAction` into a `SessionOp` — the miss rule, and what a click
  means;
- `PickIndex::hovered_for` (`pickindex.rs:1280`) is the priority rule,
  *"an edge within `EDGE_PICK_RADIUS_PX` beats the face behind it"*
  (`:1477`);
- `pane/viewport.rs:183` says so from the outside: *"the un-projection,
  the ray service, the miss rule — lives in
  `pickindex::PickIndex::op_for`"*.

What `pick.rs` actually holds is a **cache**: when a build is asked
for, what is done with the answer, and the typed refusal for the window
in between. That is lifecycle, not picking policy — and the split's real
boundary is *stateful, seam-driving* against *pure*, which is a better
boundary than the one the header claims.

The names inherit the mismatch. A reader asking how a pick works opens
`pick`; nothing there answers. `pickindex` reads as *the index*, and it
is the index plus every decision taken over it.

## Confidence

`sure` that `op_for`, `hovered_for` and the priority rule are in
`pickindex` and not in `pick`. `likely` on the naming being worth
revisiting — renaming a module twice in a week is its own cost, and
re-wording both headers may be the whole fix.

## The false sentence is fixed; the rename is DECLINED, and this row is the schedule (#2079 fix pass, 2026-09-06)

The half that was owed is paid. `pick.rs:1-3` no longer claims to be
*"the policy half of picking"*; it now opens *"The pick index's
LIFECYCLE"* and says in its own words that it decides nothing about
picking, that `op_for`, `hovered_for` and the priority rule are
`pickindex`'s, and that **the boundary the split actually landed is
stateful-against-pure**. `pickindex`'s header says the matching thing
from the other side. So no header now asserts the mismatch this item
found.

The rename is declined here, for three reasons in descending weight.

**1. A rename is not a move, and this PR's whole warrant is that it is
a move.** The reviewer certified it with a whitespace-sensitive
sorted-line diff — one line removed, 43 added, all headers and imports;
no signature, `derive`, field or `impl` touched. Renaming `pick` and
`pickindex` rewrites every call site in fifteen source files, twelve
test files and two examples, and that diff is not certifiable the same
way: the check that made this unit safe to read stops working on the
change that would ride it.

**2. The right names are not knowable yet, because the second split is
not taken.** `pickindex-holds-the-frames-marks-as-well-as-the-index`
names ~405 lines — `highlight`, `edge_overlay`, `focus`,
`cursor_projection` and their companions — as a clean second boundary
with a different consumer set. If that comes off, what is left is
genuinely *the index and the cursor questions over it*, at which point
`pickindex` may be the right name after all, and the third module needs
one too. Naming these modules once, after the split that decides what
they hold, is one decision; naming them now is two.

**3. Even the easy half is not clean.** `pick.rs` → `pickcache` would
leave `NotIndexed` and `unindexed` — a refusal vocabulary, not a cache
— in a module named for a cache. The obvious rename does not survive
its own contents either.

**What this row is now**: the schedule, parked behind the second split
rather than behind nothing. A taker should decide the names for both
modules and the third at once.

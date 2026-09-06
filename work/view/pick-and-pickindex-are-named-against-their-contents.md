---
id: pick-and-pickindex-are-named-against-their-contents
kind: issue
title: the module called pick holds no picking, and the policy it claims is in pickindex
status: open
opened: 2026-09-06
refs: [2079]
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

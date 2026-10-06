---
id: a-blend-refuses-a-solid-of-several-shells
kind: issue
title: blend: a solid of several outer shells refuses UnsupportedBody before any chain is read, so a split half that came out in two pieces cannot be blended
status: closed
opened: 2026-10-02
priority: P2
cost: M
pr: 4113
closed: 2026-10-06
---

Found by SHOW's `split-node-chords-by-name-has-no-demo` while varying
the half of the bracket's split (`demos/tour/src/bracket.rs`,
`split_and_break`).

The bracket split at `x + y = 2.75` leaves its two leg tips on the
`Above` side: one solid of two disjoint outer shells, which is a legal
body (DESIGN.md's structural conventions; `topo`'s `Solid` doc, settled
by `work/tquery/a-solid-is-documented-connected-but-split-returns-disjoint-pieces-in-one.md`).
`Node::Chamfer` and `Node::Fillet` over any of that half's section
edges refuse `BlendError::UnsupportedBody { solids: 1, shells: 2 }`
before any chain is read; the recourse (`FILLET3_BODY_RECOURSE`) is
"a single solid with a single shell".

The edges asked for each lie wholly in one shell, so the request never
crosses shells. A door that blends per shell, or that admits a body
whose requested chains stay inside one shell, would serve it. Behind
this refusal the same selection still meets
`a-plane-plane-blend-cannot-end-at-an-unrequested-corner`, so this row
alone does not unblock that scene.

## Priority

P2. A solid of several outer shells is what a split leaves wherever its
plane cuts a part into separate pieces on one side, and what a disjoint
union is, so ordinary ops on ordinary parts hand the blend this body.
The special case is the door's: a request whose chains each stay in one
shell should be handled uniformly, shell by shell, rather than the body
refused. Below the run-out row because that one blocks the same scene
first and blocks a single solid of one shell too.

## Findings (the lane that closed it)

- The premise was off on main: the split's `Above` half is **two solids
  of one shell each** (`topo::pieces` sorts a split's result into
  pieces), so the refusal read `UnsupportedBody { solids: 2, shells: 2 }`,
  not `{ solids: 1, shells: 2 }`. The fix is the same for both shapes:
  the surgery no longer reads the body's inventory at all.
- The blend carves each chain inside the shell its links lie in
  (`blend_surgery`'s `chain_shell`); every other shell and solid is
  carried through under its own keys, and `Blended::shells` reports
  the shells carved. A chain or corner whose faces span two shells is
  tier-1 invalid input and refuses `BlendError::BodyNotIntact` at that
  edge or vertex. `UnsupportedBody` and `FILLET3_BODY_RECOURSE` retire.
- A sealed void (one solid, an outer and a void shell) blends the
  same way: its concave edges fillet and chamfer at their closed forms.
- Behind the lifted refusal, the bracket's leg tips meet
  `UnsupportedRunOut` (walls 4 and 5), which is
  `a-plane-plane-blend-cannot-end-at-an-unrequested-corner`.

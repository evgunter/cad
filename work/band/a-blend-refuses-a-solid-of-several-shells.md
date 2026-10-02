---
id: a-blend-refuses-a-solid-of-several-shells
kind: issue
title: blend: a solid of several outer shells refuses UnsupportedBody before any chain is read, so a split half that came out in two pieces cannot be blended
status: open
opened: 2026-10-02
priority: P4
cost: M
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

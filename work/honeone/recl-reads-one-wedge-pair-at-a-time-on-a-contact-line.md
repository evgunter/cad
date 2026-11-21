---
id: recl-reads-one-wedge-pair-at-a-time-on-a-contact-line
kind: issue
title: recl_edges reads each wedge pair on a contact line apart, where one line cone of each solid would hold every wedge
status: open
opened: 2026-11-01
priority: P2
cost: M
---


Filed by `recl-edge-edge-composes-the-other-solids-material-beside-the-cone`'s
sweep (branch `oneread/recl-line-cone`).

## What

`crates/topo/src/boolean/recl.rs` `recl_edges`, where a solid mentions
the common ray more than once (`a_ms.len() > 1 || b_ms.len() > 1`),
pairs one wedge of each solid at a time through `resolve_edge_edge`,
whose line cone holds that wedge and the faces folded into it but none
of the solid's other wedges on the ray: the closure's own comment says
the reading is "blind to its other edges' wedges", with
`touch_rows` and the carry (`ops::Descendants`) left to reconcile a
wedge another one swallows. One line cone per solid holding every
wedge on the ray (each edge's two flanks, each a half-plane) reads the
whole material about the line once, by the same parity.

## Owed

Weigh building each solid's line cone once per contact ray from all
its mentions, and reading every wedge pair's flanks against it; or say
why reading the wedges apart is the right construction there (the
germ placing, `crossings`, is per wedge).

Moved from ONEREAD to HONEONE on 2026-11-20, by `git mv`, when ONEREAD merged into HONEONE: ONEREAD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one HONEONE carries.

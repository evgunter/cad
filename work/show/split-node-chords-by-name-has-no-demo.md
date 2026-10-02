---
id: split-node-chords-by-name-has-no-demo
kind: unit
title: Node::Split in a document, with its section chords chamfered by name downstream, has no demo
status: dispatched
opened: 2026-10-02
priority: P4
cost: M
---

## What

PR #3629: `Node::Split` no longer refuses over a tied operand, and its
section chords are named by their end vertices, so a downstream node
can select them (`crates/editor-core/tests/lib_g14_split_walls.rs`, an
L-extrude cut across both legs with the chords reachable one by one).
`Node::Split` has no use in `demos/tour`.

Candidate home: `bracket`, re-authored as a document whose last nodes
split it and chamfer one half's section chords by name — which also
puts `bracket` in the gallery. Take it last; close it with the reason
if it does not make the bracket a better part.

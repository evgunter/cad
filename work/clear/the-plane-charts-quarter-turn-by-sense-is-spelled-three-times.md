---
id: the-plane-charts-quarter-turn-by-sense-is-spelled-three-times
kind: issue
title: injective.rs spells the plane chart's quarter-turn toward the face, by the sense bit, three times, and the fold census keeps one sanctioned tuple per copy
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), from the review of that PR.

## What

`crates/topo/src/injective.rs` turns a line edge's chart direction a quarter toward its face, by the face's sense bit, in three presentations:
- `present_plane`'s `s` and its `rot` closure;
- `present_plane_corner`'s inline `bv = [-a[1] * s, a[0] * s]`;
- `present_plane_vertex`'s `n = [-tau[1] * at(s), tau[0] * at(s)]` (PR 6907).

The rule is one rule: the face lies left of the edge's traversal about the outward normal. It is spelled three times, and `face_normal::tests::no_source_file_folds_the_bit_by_hand` keeps one identical `("crates/topo/src/injective.rs", "1.0")` tuple in `sanctioned` per copy. Each new plane presentation adds a fourth spelling and a fourth tuple, and the census cannot tell a fourth sanctioned quarter-turn from a new hand fold.

## Owed

One home for "the face side of a line edge in a plane's chart", which the three presentations call, so that the census sees one site and `sanctioned` holds one tuple.

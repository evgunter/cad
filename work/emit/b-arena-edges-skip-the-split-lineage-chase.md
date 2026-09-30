---
id: b-arena-edges-skip-the-split-lineage-chase
kind: issue
title: In a result that is B's clone, the boolean edge pass does not chase a split sub-edge's lineage, so `union(tip, bar)` refuses SharedRim where `union(bar, tip)` names the edges
status: closed
opened: 2026-09-23
priority: P0
cost: D
closed: 2026-09-23
pr: 3114
branch: emit/b-arena-edges
---

## What

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`, the
operand-descended root read (after `operand_key` has put the edge on its
side):

- side A: `chase_edge_to_table(body, a.table, e)` walks the edge's
  `SplitEdge` provenance in the result body until A's table names a key.
- side B: `chase_b(e)`, whose walk steps through `fwd_edges` — the graft
  rows. In `(Absent, Direct)` (the result is B's clone) those are EMPTY,
  so a sub-edge the reduction split off a B edge is returned unchased,
  reads as `resolves == false`, and goes to `chord_kind`, which
  re-derives the root as the ONE rim its two faces share
  (`rim_between`) and refuses `NamingError::SharedRim` when they share
  more than one.

In that layout the result body IS B's clone, so its provenance speaks
B keys exactly as it speaks A keys in the mirror layout; the A lane's
direct chase is available and is not used.

## Measured: an order-dependent refusal on a legal recipe

PR #3103's reviewer built it. `bar` is a declared union of two flush,
x-offset placements of one unit block (a Seamed result whose merged
front and top faces keep two collinear top-front edges); `tip` is a
small prism whose apex touches the top-front edge at (0.6, 0, 1) from
inside the bar.

- `union(bar, tip)` → `OperandA`, the split edge's halves named
  `FromA(<bar's edge>)` with fragment ordinals.
- `union(tip, bar)` → **refuses** `SharedRim`: "faces FaceKey(1v1) and
  FaceKey(3v1) of operand node 6's body share more than one edge where
  a seam chord's rim, derived from adjacency alone, needs exactly one".
- Both intersections (`OperandA` / `OperandB`, the result is the tip)
  name the same edges with sides swapped.

A normal verb refusing on normal geometry, and only in one operand
order: P0.

## The red row

Add to `crates/editor-core/tests/emit_boolean_vertex_keys.rs` and extend
`swapping_the_operands_swaps_the_sides_of_every_name` with it as a
sixth fixture (union must name both orders, sides swapped):

```rust
fn bar_and_tip(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let place = |doc: ProfileDoc, dx: f64| {
        insert(doc, Node::Transform {
            input: proto,
            translation: [len(dx), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        })
    };
    let (doc, m1) = place(doc, 0.0);
    let (doc, m2) = place(doc, 0.5);
    let (doc, bar, _) = declared_union(doc, &[m1, m2], flush_pairs((m1, proto), (m2, proto)));
    let s3 = 3f64.sqrt();
    let n = [1.0 / s3, 1.0 / s3, -1.0 / s3];
    let l = 1.5f64.sqrt();
    let e1 = [-1.0 / l, 0.5 / l, -0.5 / l];
    let e2 = [
        n[1] * e1[2] - n[2] * e1[1],
        n[2] * e1[0] - n[0] * e1[2],
        n[0] * e1[1] - n[1] * e1[0],
    ];
    let (doc, tp) = on_frame(doc, [0.6, 0.0, 1.0], e1, e2,
        vec![vec![(0.0, 0.0), (0.3, -0.1), (0.3, 0.1)]]);
    let (doc, tip) = insert(doc, Node::Extrude { profile: tp, distance: len(0.3) });
    (doc, bar, tip)
}
```

(`declared_union` / `flush_pairs` from `docm7_union_declare`; `scl`,
`ang` from `fixture`.) On main at the time of filing, `union(tip, bar)`
refuses and the row is red.

## Fix shape

Give side B in the `(Absent, Direct)` layout the A lane's chase
(`chase_edge_to_table(body, b.table, e)`), keeping `chase_b` for the
grafted layout, whose verbatim keys it exists for
(`work/origin/graft-copies-provenance-keys-verbatim.md`). `operand_key`
already says which layout a key came through.

---
id: delete-the-box-scalar-sweep-leaf-switch
kind: issue
title: Delete the box-scalar sweep-leaf switch: the sweep doors finish their own result at every scalar
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [the-side-reader-cannot-read-a-solids-orientation-over-a-box, tier-3-at-every-box-replay-leaf-multiplies-the-decisions, check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends]
refs: [f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves, 6371]
---

The final state Ev ruled on PR 6371 (2026-11-02): "keep aiming for
this final state, but … keep it disabled/selectively disabled for now".

**What goes.**

- `topo::AtRestPolicy::leaf` (`crates/topo/src/props.rs`, the switch), the `Leaf` type and `topo::LeafBody`: `T::Leaf` reads as
  `T::Finished` and `T::leaf` as `T::finish`;
- `topo::finish_leaf`;
- `topo::Replay`'s second role: it stays a dual's finished body;
- `NodeErrorKind::UnfinishedLeaf` (`crates/editor-core/src/eval/mod.rs`)
  and its class, tags and rendered rows;
- `Framed`'s leaf storage, `own_leaf`/`in_frame_leaf` and the fallible
  `Framed::finished` (`crates/editor-core/src/eval/framed.rs`): it holds
  `T::Finished` again;
- the connectedness check's finish (`crates/editor-core/src/checks.rs`);
- the tour's `Operand` impl for `Replay` (`demos/tour/src/booleans.rs`).

**What waits.** It is parked on the three rows that make the box
scalars' tier 3 read what the box studies build. The `f64` row
(`f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves`) is
not a blocker: `f64` is not behind the switch.

**The test that says it is done.** The rows each blocker names pass with
their main-era assertions after the deletion.

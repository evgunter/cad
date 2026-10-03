---
id: pxn-wall-refinement-may-be-unneeded-for-the-tube
kind: issue
title: edge_nurbs: localized() refines every plane x NURBS wall to 16 spans for a tube reason the cut derivative box may have removed
status: open
opened: 2026-10-02
priority: P3
cost: M
---


Found in review of the `ssi/dome-tube` lane (PR 3903), 2026-10-02.

## What

`localized` in `crates/geom-brep/src/edge_nurbs.rs` knot-refines every
plane × NURBS wall to `PXN_WALL_SPANS = 16` spans per direction before
the hull and tube limbs run. Its docs argued that the tube's
`NurbsBoxes` derivative boxes were cell-granular: on a one-span quarter
cylinder the enclosure "straddles zero forever" at every rung.

That premise no longer holds. `NurbsBoxes::deriv_box`
(`crates/geom-brep/src/ssi/enclose.rs`) now cuts each span cell to the
tube window (`CellNet::cut`) and meets the result with the whole
cell's box. A one-span wall therefore localizes without refinement: on
the 3×3 dome of `plane-nurbs-ssi-does-not-certify-a-curved-dome`,
every cut certifies at the widest rung. The docs were corrected in
that PR to say the tube's need is unmeasured.

## Next

Measure the edge lane's certifying fixtures (the quarter cylinder
above among them) with `localized` skipped for the tube, keeping it for
the hull limb. Compare:
- the rung chosen,
- the transversality margin,
- the box count,
- the cost.

If the tube no longer needs the refinement:
- Decide whether the hull limb alone justifies it. Limb 2's composite
  is hulled per span, so finer spans tighten it.
- Restate `PXN_WALL_SPANS` for that reason alone.

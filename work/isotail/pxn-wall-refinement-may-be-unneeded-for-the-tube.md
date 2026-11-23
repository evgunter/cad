---
id: pxn-wall-refinement-may-be-unneeded-for-the-tube
kind: issue
title: edge_nurbs: does the plane x NURBS tube need the wall refined to PXN_WALL_SPANS? (measured: it does)
status: closed
opened: 2026-10-02
priority: P3
cost: M
closed: 2026-10-26
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

## Measured (NURBS, PR 4553, 2026-10-10)

The refinement could not stay as it was: at `Interval` it ran the
projective knot algebra, which no longer typechecks there
(`the-projective-applier-still-lerps-so-a-nurbs-refined-at-t-interval-pays-twice`).

**The tube needs it.** PR 4553 first dropped it, on an `f64`-only
measurement of the suites' certificates in which no verdict moved. Its
review measured the `Interval` lane on walls of its own and found
otherwise. On a rational cubic wall whose weights run 0.03…30, 19 vertical
rulings certified at `Interval`:

| ε | `main` | refinement dropped |
|---|---|---|
| 1e-9 | 19 | 17 |
| 1e-12 | 15 | 6 |

Every new refusal was `TubeNotOneArc`. Read off the wall's own cells,
the minted whole-domain chart speed is 115× (`u`) and 1000× (`v`) the
sampled one. The pads divide the rung by that speed, so they collapse
by as much. Over sixteenths the minted speed is 2.8× and 15.8× the
sampled one.

**So the tube reads the wall refined to `PXN_WALL_SPANS` in the
certification ring** (`edge_nurbs::wall_grid`, `ssi::enclose::CellGrid`).
The schedule is `main`'s, but the homogeneous channels `(w·P, w)` are
refined by `CurvePlan::apply_certified`, so the cells enclose the
described wall's refined net. `NurbsBoxes` reads its cells, the minted
speeds among them, off that grid. Limb 2's hull reads the wall itself,
cut in the ring by its own composite, and no limb-2 reading needed the
refinement. On the review's walls at all three ε and both scalars, no
ruling certifies fewer than on `main`. The pads match `main`'s to about
1e-13 relative. The `Interval` hull is 0.007–0.32× of `main`'s (`main`'s
was the projective form's width), and at 1e-12 every decades ruling
certifies, where `main` refuses 4 to 6 per family on `HullSup`.
`tests/review_4553_pxn_probes.rs` holds the rulings and the speed ratio
as rows.

This row's question is answered: the cut derivative box localizes
the tube's windows, but the minted speed is a whole-domain sup over
cells, and that needs fine cells. It stays ISO's to close.

## Closed (2026-10-26, tracker sweep)

**Fixed: the question is answered and the code states the answer.** PR 4553's measurement (above) showed the tube does need the refined cells, and the code now says so as the constant's reason: `crates/geom-brep/src/edge_nurbs.rs`'s `PXN_WALL_SPANS` (~2026) and `wall_grid`'s doc (~2039–2057) state that the tube's chart readings are hulls over span cells and that a wall with few spans reads its minted speed up to a thousand times the sampled one, citing `crates/geom-brep/tests/review_4553_pxn_probes.rs`, which holds the rulings and the speed ratio as rows. The "unmeasured" wording this row was filed against is gone, and limb 2's hull reads the wall itself, so the "restate the constant for the hull alone" branch never arises. Nothing is owed.

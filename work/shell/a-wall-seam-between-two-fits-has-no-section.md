---
id: a-wall-seam-between-two-fits-has-no-section
kind: issue
title: C5 implements no NURBS x NURBS section, so a shelled loft's wall-wall seams (two moved fits meeting at a crease) refuse; the next gate for every lofted shell
status: open
opened: 2026-10-08
priority: P2
cost: H
refs: [a-fitted-wall-has-no-section-with-a-moved-cap]
---

Filed from the designer pair on `a-fitted-wall-has-no-section-with-a-moved-cap`
(`analysis/design-fork/shell-fitted-wall-section`). Once plane × `Approx`
routes over the fit, a shelled loft's cap edges have a section, but
every wall moves too: the twisted loft's wall–wall seams are creases
between two moved fits, a NURBS × NURBS section, which C5
(`crates/geom-brep/README.md`) lists as unimplemented even for plain
NURBS (it is the same gap on the straight prism's walls). It refuses
`NeighborPairUnroutable(Nurbs, Nurbs)`.

Owed: a NURBS × NURBS arm — a marcher, both charts' uniqueness tubes,
an exhaustiveness and seeding story — or a measured reason the seam can
be read another way. Before pricing, re-measure the 2026-09-25 rows the
designers cited: the twisted loft's wall fit failing the default ε
(4.14e-9 achieved) and the vase's interior-knot crease gate.

## Measured

2026-10-09, after plane × `Approx` routed over the fit
(`a-fitted-wall-has-no-section-with-a-moved-cap`). The twisted loft never
reaches `NeighborPairUnroutable(Nurbs, Nurbs)`:

- at ε = 1e-9 (the default) and 1e-12 the first wall's offset fit refuses
  before any edge (`Fit { BudgetExhausted }`, best bound 4.12e-9 m on a
  (27, 17) grid);
- at ε = 1e-6 the wall refuses at its seam with the next wall, at the
  iso-row arm's guard, ahead of routing: `FittedBoundaryUnsupported {
  what: "a row of this fit shared with a spline face" }`
  (`crates/topo/src/replace_face.rs:2033`). The seam is a row of the
  moving wall's own fit, and the neighbour is the unmoved NURBS wall, so
  in `shell` the pair is never two fits.

What each of the wall's edges does on its own, read per edge at its plan
(`topo::offset_edge_plans_for_tests`), one wall moved alone:

- the door plans the edges in the order top rim, seam, bottom rim, other
  seam, and stops at the first seam, so one rim is planned before it;
- by `d = 5e-10` (offd's row): the bottom rim derives as the cap plane's
  section of the fit; the top rim's section is refused and deferred to
  the corners, `NoBranch` at ε ≥ 1e-9 (the fit's window, the base's own,
  stops short of the top cap plane on this twisted wall) and an `Ssi`
  tube refusal at 1e-12;
- by `d = −0.05` (the shell's thickness) at 1e-6: both rims derive;
- the other seam is not a row of the fit, so it routes as
  `Approx × Nurbs` and refuses `NeighborPairUnroutable`.

Pinned in `crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_a_walls_fit`, and in
`crates/sweep/tests/offd_r1_probes.rs`,
`the_fitted_obstruction_holds_on_a_curved_fit` (the seam) and
`a_fitted_walls_rims_answer_for_themselves_behind_its_seams` (each
edge).

## Designed

A designer pair weighed this item on 2026-10-10 (fork-log row 106). Both designers agree on the following:

- **The seam.** A crease seam between two moved spline walls is `Intersection { fit_i, fit_j }`. That is a rung-3 section of the two fits, certified by C2's three limbs on both charts, with its witness pinned. Nothing about either fit is composed into the edge's bound: each fit's distance from its description is its face's claim, re-derived by O5.
  - The row a loft writes for a seam is exact only at rest. After a move at a crease, the new seam lies `d·cot(φ/2)` inside each moved row, where φ is the interior dihedral. So neither fit's row is the seam.
- **One door for the whole solid.** `shell` moves every chart of a solid at once through one general simultaneous door. Each edge is the section of its two moved surfaces through C5, and each corner is the root of the moved surfaces that meet there. `offset_planes_together` and `offset_charts_together` are that door's closed forms. `replace_faces_offset` stays the single-face verb.
  - Why not one face at a time: a lone moved wall's section with a held neighbour sits `d·cot φ` from the moved row. That changes sign at 90°, so on the twisted loft it falls outside the fit's window along part of the seam.
- **The iso-row arm narrows.** `plan_edge`'s iso-row arm fires only for a self-shared image, or where the distinct neighbour holds the move (`holds_the_move` gains its NURBS-mover arms). Every other iso image on a moving fit takes the section route, so both seams of a wall answer alike. The guard's two strings, "a row of this fit shared with a spline face" and "with another fitted face", retire.
  - The same change closes a gap: today the arm extracts a row off an analytic neighbour that does not hold the move (a tilted side plane), and only tier 2 catches it, without naming it.
- **Clearance becomes a gate.** Curved wall clearance (geom-brep README, Open) is the gate before a shelled loft is sound at rest. `moved_walls_cross` reads planar pairs only.
- **D2 does not change now.** D2's predicate owes `Intersection` only for a definitely-transverse edge. `dihedral.rs` answers `InBand` for any spline operand, and `validate.rs` exempts NURBS-adjacent edges by kind. A NURBS × NURBS arm alone therefore owes nothing on a loft seam.
  - `loft.rs`'s phase-6 docs gain one line: the strut is an exact shared boundary row of both walls, and its chart image states it exactly.
  - A shared-row carve-out goes into D2 with the change that teaches the dihedral classifier spline kinds.
- **Still refused:**
  - a smooth (G1) seam between two fits, which is `two-fits-sharing-a-smooth-seam-disagree-by-their-certificates`;
  - a concave crease, whose section lies outside both windows and refuses by its own window verdict;
  - the twisted wall's fit at the default ε (`Fit { BudgetExhausted }`, 4.12e-9 achieved), its own gate;
  - the Interval scalar (`NurbsLaneUnsupported`).

## Decided

Ev, PR 4515, 2026-10-10 ("ok sweet, B then!"): the NURBS × NURBS arm is one **complete** operation. The section is the full solution set of `S₁ ∩ S₂` over the two domains, and the consumer selects from it. There is no seeded/complete service split in C5. The arm is built so that a consumer pays only for what it uses:

- **The handle.** `Section::of(s₁, s₂, domain)` runs the boundary pass once: each surface against each side of the other's knot rectangle, giving the crossings and the `Side`/`Corner` regions. It returns a handle holding them, with two methods under one contract:
  - **`branch_at(seed)`.** It settles the seed onto both surfaces by Newton. If the seed lands in an already-certified tube or a region, that is the answer. Otherwise it traces from the seed to the crossings it reaches, then fits, refines and certifies limbs 1–3 and the two-chart tube. One branch is marched and one certified. The branch returned is the one whose certified one-arc tube contains the settled seed, so the selection is proven, not sampled.
  - **`all()`.** It traces every open branch between crossings and every closed loop from the subdivision's seeds, then runs the accounting: every cell is excluded or accounted to a tube or region. Exhaustiveness is `all()`'s obligation, which is how C3's "exhaustiveness is an in-op obligation" already reads.
- **The callers.** The offset door calls `branch_at` with the old seam's midpoint, so `shell` never runs the 4-D subdivision. The boolean calls `all()`.
- **What C5 and C3 say.** C5 keeps one row with one meaning: `implemented` covers the whole operation. C5 lists NURBS × NURBS as implemented when the arm lands. C3 states the two-chart tube and the surface × side boundary pass at the same time.
- **The plane × NURBS door.** `offset_derive::plane_wall_section` moves from nearest-sample selection to `branch_at` on the plane × NURBS door. That door gets the same handle, in the arm's unit or the next.
- **One unproven step.** Newton's convergence from the seed to the near component is not proven. The old seam's `d·cot(φ/2)` proximity makes it benign, and the tube check and `VertexDisagreement` catch it if not.
- **Before the unit:** grep `intersect::route`'s callers for any that read `implemented`.

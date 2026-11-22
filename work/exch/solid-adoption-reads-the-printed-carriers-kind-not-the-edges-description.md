---
id: solid-adoption-reads-the-printed-carriers-kind-not-the-edges-description
kind: issue
title: solid adoption picks an edge's arm by the printed carrier's kind (a parse-time promotion of evidence), where DESIGN [import] step 2 says it reads the edge's description
status: closed
opened: 2026-11-20
closed: 2026-11-21
priority: P1
cost: H
---


Found by the design question under unit B's review (PR 7170), on which two designers agreed independently.

DESIGN.md [import] step 2 has the importer verify that the printed curve lies within ε_in of the edge's description and then rebuild from the description: "no value the file printed becomes a cache". The solid path does not do that today.
- `build.rs` `iso_of_wall` chooses its arm by the printed carrier's KIND. A NURBS carrier gets the bitwise column match; a `Line` carrier gets `line_iso_match`, which ships the affine `Running::Chord`.
- Elsewhere, a NURBS rim on analytic walls refuses. Main reached this only through a parse-time promotion: PR 7170's review fixtures `cyl_cubic` and `cyl_single` on `exch/arc-map-obligation-rev2`, which import on main only because the rim promotes.

So whether a solid imports depends on whether a piece of evidence was promoted. Fixes that compensate in recognition follow from that. INV-C5 was placed on the printed carrier when its reason concerns the wall's column. `circle_iso_match` re-recognizes a column whose bits the promotion discarded.

What is owed:
- Solid adoption builds each edge's carrier from its description: the section of an analytic pair, or the wall's column for an iso.
- It certifies the printed carrier on that carrier, over its trim at ε_in, by locus. This replaces today's 3-sample `on_description`.
- Curve recognition then matters only where the promoted curve is the output (wireframe, re-export), and "recognition failing never refuses an edge that imports today" holds by construction.

Whether parse-time promotion leaves the solid path entirely depends on a sweep for any remaining reader of a printed carrier's kind: the conic arm of `endpoint_params`, the region and extent levers.

Cut after unit B, PR 7170, which brings recognition to locus-only and moves the map obligation onto the column readers. Unit B's refusal-site finding, recorded on that PR, says where M3's refusal originates.

Signed: (EXCH orchestrator)

## Closed

Closed 2026-11-21 by PR 7304 (`exch/adopt-by-description`). Solid adoption now certifies each edge's printed carrier on the carrier built from the edge's description, over its whole trim, using one certified bound in metres (`crates/step-import/src/on_built.rs`). The old 3-sample `on_description` read is gone except where no arm serves the pair. The iso rung chooses by the column's own law, curved columns included. The sweep's answer is option A: parse-time promotion stays a hint for the pre-build readers (`knot_split`, the normalize rim readers, singularity cuts), which move topology before any description exists. No build reader chooses an arm by the printed kind. On the 113 committed fixtures at three kernel-ε and three ε_in rows, head and merge-base agree in every row.

What is left is filed:
- `the-printed-on-built-certificate-has-no-arm-for-a-built-spline` (P2): the pairs that still read 3 samples.
- `the-printed-on-built-certificate-reads-curved-pairs-loosely` (P2): the demonstrated over-refusal, pinned by `declined/cyl_declined_loose.step`.
- `the-knot-split-cuts-a-printed-spline-at-a-crease-on-its-trims-end` (P2).
- `a-spline-section-is-swapped-for-its-chord-after-it-is-certified` (P3, pre-existing).

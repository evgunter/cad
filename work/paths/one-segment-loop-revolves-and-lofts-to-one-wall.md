---
id: one-segment-loop-revolves-and-lofts-to-one-wall
kind: unit
title: A one-segment loop revolves and lofts to one wall: the wrap edge (#4175), the torus readers as loop integrals, the one-face full torus probe-built
status: dispatched
opened: 2026-10-06
priority: P1
cost: H
branch: claude/clever-bardeen-4itqb3
---


Implements Ev's ruling on #4175 (2026-10-06). The design is D1's seam sentence: a **wrap edge** is a property of a face, placed where the construction cut. Both designers' reports are in #4175's body. Unit 3 (#4169) refuses revolve and loft of a one-segment loop with `OneSegmentLoop` until this unit lands.

- **The seam flag.** `EdgeDescriptionSpec::seam` becomes a wrap flag. The closed direction comes from the carrier's chart class:
  - a meridian wraps `u`;
  - a torus parallel wraps `v`;
  - a closed spline net's boundary column wraps `u`.

  The `u_ref` half-plane predicates (`SeamHalfplane`, `SeamSide`, `seam_frame`) retire in favour of the walk's certified joint (`pcurve_loop_continuity`). Validation adds a check that both halves of a wrap edge bound one face.
- **Torus readers.** The flux and material-sign readers become `∮ F(v) du` over the loop's lift (`geom_brep::props::curved`, replacing `torus_parse`/`torus_rims_at_extremes`). This also lets ringed and L-shaped torus faces measure.
- **Mesh** identifies a wrap edge's two columns in whichever direction it wraps.
- **Revolve and loft** build the one wall and drop `OneSegmentLoop`. The loft needs a one-segment assembly arm: with the refusal removed, `loft::assemble` currently panics on an index.
- **The n = 1 arms unit 3 left.** `sweep::revolve::chain::build_chain` lays a closed meridian chain of n ≥ 2 (`qs[1 % n]` and its closing `mef` assume two vertices); `editor-core`'s `names::emit_sweep::resolve_chain_opt` refuses n < 2 ("revolve chain/rim length mismatch"). Each needs a one-segment arm: one wall over one segment, the strut its wrap edge. Neither is reached until revolve builds.
- **The one-face full torus**, which closes in both directions, is probe-built first: loop walk, mesh, the boolean's torus arms. If it fails, the full revolve alone cuts at the antipodal latitude.
- **STEP import.** `normalize::full_torus` is deleted.
- **Renaming.** "seam" also names `BooleanCoincidence::Seam` (a declared G1 join, held by D10) and a props usage. Rename the chart sense to "wrap edge" in code where it reads clearly, and leave the held homonym alone.

**As landed.** The probe built the one-face full torus first (torus.step adopted as stated): it passed tiers 1–3, measured 2π²Rr², meshed closed, and answered the boolean's torus arms as the two-face form does once a face wrapping both coordinates is served as the whole chart (`torus_face_windows`). No fallback. The wrap flag (`ChartCurve::wrap`, `EdgeDescriptionSpec::wrap`) replaces the half-plane predicates; tier 3 holds a wrap edge's halves to one face, and a split or boolean that parts them restates the edge as an ordinary image. The torus readers are `torus_chart`'s Green form over each loop's lift. Revolve builds the one wall (`revolve::turn`), the full turn one face closed both ways; loft builds one spline wall whose strut is the column run back (`geom_brep::reversed_column`, a backward seam class in the pcurve iso lane). `normalize::full_torus` and `NormalizationKind::FullPeriodTorus` are deleted. A split that parts a wrap edge at one vertex is filed on CLEAVE (`a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it`).

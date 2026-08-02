# M5 PR 13 — curved STEP subset: conics + NURBS entities (binding spec)

Branch `ev/m5-pr13-curved-step` from current main. Plan line 13
(docs/M5-PLAN.md:308-312); the M5 envelope's export growth
commitment (CURVED-DESIGN :705-707). Depends on: S10's sense bit
(same_sense — already wired for planes), PR 5/6 (conic carriers +
pcurves), PR 3 (NURBS types). Independent of the die-pips chain.

## 1. Writer growth (AP214, §12.3.2 exact forms)

- Conic edge carriers export as exact STEP entities: CIRCLE and
  ELLIPSE with AXIS2_PLACEMENT_3D (Book §12.3.2; the C1
  rational-quadratic export form for conics that need it —
  verify which R5 curves are representable as native
  CIRCLE/ELLIPSE vs needing B_SPLINE_CURVE_WITH_KNOTS rational
  form, and use the NATIVE entity wherever exact).
- Curved surfaces: CYLINDRICAL_SURFACE, SPHERICAL_SURFACE (the
  R5 set; CONICAL_/TOROIDAL_ if any corpus shape carries them —
  enumerate what is constructible at rest and cover exactly
  that; frontier text for the rest).
- NURBS entities (B_SPLINE_CURVE/SURFACE_WITH_KNOTS, rational
  where weighted): the substrate exists at rest only as rung-3
  edge carriers (SSI branches) — check what IS at rest post-#158
  and export what exists; the loft-assembly unit brings NURBS
  faces later (frontier text names it).
- same_sense: Face::sense verbatim (the S10 alignment — extend
  the existing planar wiring to the curved ADVANCED_FACEs).
- ADVANCED_FACE bounds for curved faces: EDGE_CURVEs on the
  curved carriers with the correct bound orientation composition
  (the S10 review's composition rule: loops wind CCW about the
  outward normal; same_sense carries the reversal — do NOT
  double-compose).
- The writer stays byte-deterministic (the byte-golden
  convention); goldens for the new entities minted and committed.

## 2. Import acceptance (FreeCAD oracle)

- The R5 corpus shapes constructible at rest — cut_cylinder
  (shape (i), Ellipse edges), boss_union (shape (ii), Circle
  seams), the S11/S13-era bodies as they exist — export and
  import into FreeCAD 1.1.2 headless (scripts/check_step.sh,
  the CI step-import job; memories/freecad-oracle.md) with
  geometry checks: volume agreement within FreeCAD's tolerance,
  face/edge counts, no import errors. The filleted die rides
  PR 12 (add its row then — frontier note now).
- Round-trip honesty: what the kernel cannot yet import (import
  is M7) is stated in the writer docs — export-only subset,
  said plainly.

## 3. Demo narration

- The tour's STEP stops gain curved narration (the S11
  m5_s11_same_sense .F. emission row's flip lands here if the
  curved writer makes the notched body exportable — check its
  doc comment and flip per instructions).

## 4. Acceptance

- Byte-golden rows for every new entity kind; FreeCAD import
  green on the R5-at-rest set (volume + counts); the orientation
  oracle extended to curved faces (winding × same_sense
  composition pinned both senses — a reversed sphere face from
  S12's revert exports and imports with the correct material
  side, if FreeCAD's checker can see it; else pin the emitted
  text); multi-ε where numeric (exports are exact structure —
  most rows are ε-independent, say so); two-tolerance on any
  new refusal arm.

## 5. Out of scope

STEP import (M7); NURBS FACES at rest (loft assembly unit);
assembly structure/colors/PMI; AP242. Frontier errors name the
real blocker.

## 6. Process

One implementer + one blinded adversarial reviewer + one fix
pass. Review charter musts: independent STEP-semantics check of
the new entities against ISO 10303-42 (the references/ scans if
needed); an adversarial orientation attack (construct a body
where a double-composition bug would flip material side and
verify via the FreeCAD volume); byte-determinism across two
builds; golden diff audit. Local scope: touched crates
(step-export + consumers) default ε; the FreeCAD oracle rows
locally (NOT in the narrowed CI path — verify which CI job runs
them and rely on it if hosted); CI proves the matrix. Push per
unit; foreground only; OUTPUT DISCIPLINE per standing process.

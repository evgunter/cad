---
id: plane-nurbs-not-transverse-says-coincident-on-a-zero-arm
kind: issue
title: geom-brep: PlaneNurbsRefusal::NotTransverse says the tangent planes coincide on a Zero verdict, and prints no margin
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

- `crates/geom-brep/src/edge_nurbs.rs`, `impl Display for PlaneNurbsRefusal`, `Self::NotTransverse`: "the plane and the NURBS wall have coincident tangent planes at interior sample {sample}, …". The verdict is a Zero (or Negative) classification of the levered angle, not a proof the planes coincide, and the margin is not printed. 
- Same file, `impl Display for AnalyticRung3Refusal`, `Self::TubeStraddles`: "— it straddles zero, from a near-tangency …". It prints the margin, but asserts a straddle whatever the reading is; a positive sub-band point margin is not a straddle.

## Repair shape

Say "not shown to cross" of the reading and print `(margin … m)`, as `CertifyError::NotTransverse` now does ("at sample {s} the faces are not shown to cross (margin … m, their angle levered by …), though the edge's description says they do"; its Negative arm says "the faces do not cross"); describe the reading the margin shows rather than the arm.

## Also: the numbers print in exponent form (added by `encl/zero-arm-wording`)

Both `TubeStraddles` Displays (`PlaneNurbsRefusal`'s and `AnalyticRung3Refusal`'s) print the clearance with `{:e}`, and that clearance is `0.0` on every straddling refusal by construction (the variant's own doc), so the rendered text reads "proven clearance from zero is 0e0 m". The `Limb` arms print `{margin:e}` the same way. ENCL's payloads now print a margin through `MarginDiag::readable()` (geom-core `predicate.rs`), which is `geom_core::Readable` applied to each number: "0 m", "2 m", "5e-10 m". The repair is to use it here too.

## Progress noted 2026-10-20 (ENCL, PR 5437)

The first bullet is fixed by PR 5437: `PlaneNurbsRefusal::NotTransverse`'s `Display` (`crates/geom-brep/src/edge_nurbs.rs`) now renders certification's `CertifyError::NotTransverse` sentence, so it says "not shown to cross" on a Zero verdict, "do not cross" on a Negative one, and prints `(margin … m, their angle levered by …)`. The rest stays open: `TubeStraddles`' straddle wording on every reading, and the `{:e}` numbers of the section above.

## Noted 2026-10-20 (ENCL, `encl/check-word-one-noun`)

Both `TubeStraddles` payloads now open with the tube decision's one noun, `TUBE_CROSSING_BOUND` (`crates/geom-brep/src/certify.rs`): "the uniqueness tube's lower bound on the crossing angle is not clear of the zero band over … boxes of the chain …; the bound reads … m". `AnalyticRung3Refusal::TubeStraddles` says "the crossing angle's enclosure straddles zero" where it said "it straddles zero". Both open items above stay open: that straddle claim on every reading, and the `{:e}` numbers.

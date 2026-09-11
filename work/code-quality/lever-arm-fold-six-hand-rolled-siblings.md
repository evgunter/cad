---
id: lever-arm-fold-six-hand-rolled-siblings
kind: issue
title: The two-surface lever-arm fold has one documented home and six hand-rolled siblings; contact_tangent_opposed is classify_material_pairing's undisclosed twin
status: open
opened: 2026-09-01
github: 1439
refs: [1423]
---

## From GitHub issue 1439

Opened 2026-09-01; 0 comments.

Filed from the MATE-3 dual review (PR #1423; found bilaterally — one arm from the prose, one from the constants/data sweep). MATE-3 introduced `geom_brep::folded_lever_arm` documented as "one home for the fold" (three consumers named, margins comparable only if levered against the same arm) — but the identical three-way `min` fold survives hand-rolled at six sites: `topo/boolean/contact_verify.rs:351-353` (the fold's own stated origin), `geom-brep/certify.rs:1685-1687` and `geom-brep/ssi.rs:894-896` (the helper's OWN crate), `topo/boolean/ops.rs:1020-1022`, `sweep/revolve/upgrade.rs:233-235`, `sweep/extrude.rs:882-884`. Two of eight sites use the home.

Deeper half: `contact_tangent_opposed` (`contact_verify.rs:349-360`) is arithmetic-for-arithmetic the new `classify_material_pairing` (`normalize(∇F)·sense`, dot, `Margin::levered`) under a second predicate name — the dependency direction admits calling the shared home.

All six sites were legitimately outside MATE-3's fence; scheduling this here rather than half-fixing one crate in passing. The consolidation is mechanical but crosses four crates and re-homes a predicate name, so it wants its own small unit (any program touching those lanes may take it; S-MATE has no claim on the boolean/sweep sites).

Signed: (S-MATE orchestrator)

## Progress (FILLET-H6, PR 1891)

Three of the six are gone, without this issue being taken: FILLET-H6 hoisted
the second-order margin into `geom_brep::tangent_second_order`, which folds the
arm through the documented home, and migrated the three sites that read that
margin — `geom-brep/certify.rs` (the tangency certificate's interior samples),
`sweep/extrude.rs` (the strut join) and `sweep/revolve/upgrade.rs`
(`jet_determinate`). It also migrated `topo/boolean/rim_wedge.rs`, which was a
hand-rolled sibling of the SECOND-ORDER rule but already levered against
`folded_lever_arm`.

**The three that remain**, and why each was left:

- `topo/boolean/contact_verify.rs:351-353` — the fold's own stated origin, and
  the site whose `contact_tangent_opposed` is `classify_material_pairing`'s
  undisclosed twin. Re-homing that predicate name is the deeper half of this
  issue and wants its own decision.
- `geom-brep/ssi.rs:894-896` — surface–surface intersection's own arm, not a
  second-order margin, so `tangent_second_order` does not reach it; it needs the
  bare `folded_lever_arm` swap.
- `topo/boolean/ops.rs:1037-1039` — folds the margin into a per-sample rebuild
  walk it already runs, like the tier-3 validator's.

The tier-3 validator (`topo/validate.rs`) was never on this list — it already
levers against `folded_lever_arm` — but it does hand-roll the second-order
DECIDE, and it and `boolean/ops.rs` are now the only two that do.

## Home

`work/code-quality/` — a duplicated-spelling structural finding (one documented home, six hand-rolled twins) crossing four crates, which the issue explicitly declines to route to any one program.

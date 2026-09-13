---
id: smooth-arm-siblings-disagree-on-the-in-band-case
kind: unit
title: sweep: the two must-carry Smooth arms disagree on the in-band case and on how much of the edge they read
status: review
opened: 2026-09-05
branch: blend/9-must-carry-one-home
pr: 2491
---


## Finding (FILLET-H6's lane, PR 1891, not changed there)

The must-carry rule now has one home, `geom_brep::tangent_second_order`
(`crates/geom-brep/src/dihedral.rs`), returning a VERDICT; the two callers
keep their own escalation policy and they disagree:

- `crates/sweep/src/extrude.rs`'s strut arm escalates the in-band case
  typed (`SliverJoin`), reading ONE point with no lane gate;
- `crates/sweep/src/revolve/upgrade.rs::jet_determinate` folds `Err` into
  `false` and KEEPS the conventional description, gating on
  `tangent_certificate_lane` and sampling seven interior points.

`docs/FILLET-H6-SPEC.md`'s summary ("in-band escalates typed") was wrong
about revolve. For a strut the two agree in fact (κ_rel is constant along a
ruling; a `Line` on plane/cylinder pairs is in the lane) — an argument, not
a shared spelling. Unifying is a behaviour change: a revolve that builds
today would start refusing in-band. Making the strut sample seven points
multiplies its rows in the K stream. Needs a decision (which policy is the
rule's), then one wrapper. `folded_lever_arm`'s doc still names issue 1439's
six hand-rolled siblings; H6 removed two — the tier-3 validator's and the
boolean rebuild's remain.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `extrude.rs` and `revolve/upgrade.rs` are BLEND's; the shared wrapper it wants sits beside `geom_brep::tangent_second_order` in `crates/geom-brep/src/dihedral.rs`, which no program's `paths` reach — announced to PROPS when the unit is cut.

## Landed

**The rule's one home at the edge level** is `geom_brep::must_carry_over_edge`:
lane gate before any metering, then the certification schedule's interior
stations (`1..CERT_SAMPLES-1`) through `sample_param`, in order, the first
non-`Positive` station deciding — the same walk, order and early exit as the
tier-3 must-carry arm. Both sweep verbs call it and nothing else decides a
smooth join's description on either.

**The final shape of the answer.** `must_carry_over_edge` returns
`MustCarryVerdict` — `JetDeterminate` / `UnderDetermined` /
`InBand(Indeterminate)` — and nothing beside it. An earlier shape carried the
FIRST station's `SecondOrder` alongside; that reading is definitely positive
exactly when the verdict is a refusal, so a caller reporting its margin as the
cause would report a margin that passed. The only number a caller needs is the
DECIDING station's, and it is already inside `InBand`'s `Indeterminate` (its
margin, band and predicate name), which is what both callers put in their typed
error.

**What the lane row pins.** The lane TABLE is pinned exhaustively over
`SurfaceKind` by `review_must_carry_rule_r2_probes::the_lane_census_is_exhaustive_over_surface_kind`
(a new kind stops it compiling). `must_carry_rule::every_edge_the_two_fixtures_mint_presents_the_rule_a_lane_admitted_triple`
pins the other half over the verbs' OUTPUT — every edge of the two fixtures
presents a triple the table admits. Neither reads the verbs' source, so a door
that minted a wall kind outside the lane on some other profile would leave both
green; that limit is stated at the row.

**Filed from this unit's sweep**, all in the same PR:

- `work/blend/cap-rim-smooth-arm-decides-by-argument-not-by-the-rule.md` —
  `upgrade_rim`'s smooth arm decides by a comment argument; the arm is in fact
  unreachable through the obliquity bound on both extrusion doors, and the
  comment's stated reason is narrower than the one that holds.
- `work/blend/blend-contact-edges-mint-the-intrinsic-description-without-the-rule.md`
  — `blend::surgery::attach_contact` mints `TangentIntersection` on a
  structural flag, with the measured margins and the closed form.
- `work/bool/boolean-rebuild-folds-an-in-band-second-order-into-conventional.md`
  — `topo::boolean::ops` still folds an in-band verdict into the conventional
  posture, citing a tier-3 stance tier 3 does not take.

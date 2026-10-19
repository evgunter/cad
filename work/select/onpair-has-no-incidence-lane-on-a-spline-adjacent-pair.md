---
id: onpair-has-no-incidence-lane-on-a-spline-adjacent-pair
kind: issue
title: Plane × spline rims stay Chart(plane) because the pair's incidence lane does not certify arc rims and no lane exists at the interval or dual scalars
status: open
opened: 2026-10-13
priority: P2
cost: H
refs: [plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps, onpair-has-no-incidence-lane-on-two-spline-charts]
---


## Finding

Filed by TANG's `OnPair` lane (`tang/onpair-description`), from its review (m3).

`OnPair`'s certificate is incidence on both surfaces through each
surface's implicit residual (`geom_brep::certify`, `Resolved::OnPair`),
and a `Surface::Nurbs` or `Approx` surface has no implicit form
(`resolve` refuses it `Unimplemented`). So two doors keep a two-surface
locus on a plane × spline pair as `Chart(plane)` instead of the pair:

- the loft's cap rims (`sweep::swept::describe_face_rim_at_rest`): the
  cap's own profile against its spline wall;
- STEP adoption's conventional rung for any plane × described-NURBS
  edge (`step-import/src/adopt.rs`, `nurbs_rim` ← `nurbs_plane_pair`),
  which is not limited to cap rims.

That is a capability gap, not D2's classification: D2's `Chart` list
(wraps, iso lines, same-surface splits, stated spline isos) does not
name a cap profile against a spline wall, and the STEP arm re-opens a
per-door chart choice for the class. A smooth plane × spline seam has no
honest description today; a transverse one would be `Intersection`
through the plane × NURBS lane (`PlaneNurbs`), exempt at tier 3 by kind.

The way through is a foot-point incidence lane for `OnPair` on a
spline surface (the plane × NURBS lane's certified foot residual is the
model), after which both doors state the pair or the intersection the
rule reads.

## The lane, built (`tang/onpair-spline-incidence`)

`OnPair` on a plane × spline pair now certifies incidence through the plane × NURBS lane, and nothing about how the surfaces meet:
- **The lane:** `geom_brep::plane_nurbs_incidence`, which is limbs 1 and 2 of `ssi::certify::incidence_limbs` over the lane's chart image. It reaches certify as `NurbsLane.incidence`, and `run_checks` resolves the pair as `Resolved::OnPairNurbs` after `plane_nurbs_pair`.
- **`Approx`:** an `Approx` wall is metered against its fit. Incidence on its description is that bound plus the fit's own recorded distance, which nothing adds.
- **The pcurve:** `topo/src/pcurves.rs` gives an `OnPair` on a spline chart the rim and seam-column arms an `Intersection` takes.
- **Rows:** `crates/geom-brep/tests/onpair_plane_nurbs.rs`. Two spline charts are `onpair-has-no-incidence-lane-on-two-spline-charts`.

## The doors: still `Chart(plane)`, blocked on two things

**Measured.** Both doors switched to the pair (`sweep::swept::describe_face_rim_at_rest` → `at_rest_on_pair(cap, wall)`; `step-import/src/adopt.rs`'s `nurbs_rim` → `on_pair`) fail 68 rows across sweep, topo and step-import at the default ε, against 0 on main:
- **37 × `PlaneNurbs(Limb { HullValueChart })`, margins 1.4e-5 … 1.6e-3.** These are the arc rims. A polygon section's line rims certify on the pair (`loft_profiles_door::at_f64_loft_profiles_is_loft_body` with its polygon alone).
- **5 × `PlaneNurbs(Unsupported { "the analytic operand's structural parameters are not exact at this scalar…" })`**, at the interval scalar.
- **5 × `NurbsLaneUnsupported`**, at the dual scalar, which has no lane. `Chart(plane)` certifies there with none.
- **About 20 knock-ons** of the same bodies: pcurves, tier 3, census, transform.

A door may not state the pair where the lane certifies and the chart elsewhere: that is a per-edge choice keyed on what a lane can certify, which D2's form rule refuses. So both doors waited for:
1. **The limb-2 image for arc rims.** Limb 2's hull reads the lane's FITTED chart image (`edge_nurbs::chart_image`), whose foot path a circle bends. The exact image is the boundary row the pcurve mint already derives for an arc rim (`Pcurve::IsoArc`). This is the limb-2 half of `plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps`, whose limb-3 refusals `OnPair` no longer asks.
2. **A lane at the interval and dual scalars.** What a door states where no lane exists at its scalar is a design question in its own right, and is not decided here.

## Blocker 1, built (`tang/onpair-arc-rim-image`)

The lane now certifies a CIRCLE carrier, an arc rim, through its exact `Pcurve::IsoArc` image (`geom_brep::plane_nurbs_incidence`):
- **The exact image's certificate** is the pcurve mint's own: `PcurveCache::certify`, `MapResidualIsoHull`. Limb 1 is its sampled residual and limb 2 its envelope.
- **The image** is the one the body stores for the rim where one is stored, at the op and at rest alike (topo hands it in through `EdgeCurve::certify_via_stated`, and tier 3 through `recertify_via_stated`). Otherwise it is `geom_brep::pick_arc_rim`'s: the one of `arc_rim_candidates` that holds the carrier at the interior probe. That pick is the one selection of an arc rim's image, and `topo::pcurves`' mint calls it too.
- **Refusal.** An exact image that escalates (at the pick or in its certificate) escalates the edge, as the mint's does: the fitted path never makes it definite. Where the exact image refuses definitely, or no candidate holds, the fitted path decides. Where both refuse, the edge refuses `CertifyError::OnPairArcRim`, carrying both causes, and it reads as the exact image's check where it names one.

**Its limits** (also on the fn's doc):
- the arc-rim certificate compares the wall's whole boundary row with the circle, so it needs a rim spanning the chart's full `u` domain (a partial rim falls to the fitted path);
- it reads the row's weight pattern on the `f64` value channel;
- it does not check that the edge's span lies in the image's arc (`an-iso-arc-certificate-never-checks-the-edge-span-lies-in-its-arc`, older than this lane).

**The doors, measured again with (C)** (both switched to the pair; sweep + topo + step-import, default ε): **34 failures, against 68 without it.**
- All 37 `HullValueChart` arc-rim refusals are gone.
- 6 × `NurbsLaneUnsupported` (Dual) and 5 × `PlaneNurbs(Unsupported …)` (Interval) remain; that is blocker 2.
- No rim refuses `OnPairArcRim`.
- The rest are rows that pin the rims' present `Chart(plane)` form, or knock-ons of the same bodies.

So the doors now wait on blocker 2 alone, plus re-baselining the rows that pin `Chart(plane)`.


## Blocker 2, decided (D2, 2026-10-16)

Ev decided blocker 2 on the `[ev]` PR (fork-log row 152), and it dissolves the question. D2 no longer has a conventional pair form. A two-surface edge is one form, `{ pair, selector }`:
- a curve known in advance (a closed form of the pair, such as a loft's boundary row, or a curve chosen in a 2-D overlap) takes a **structural** selector, certified by its own meter and each surface's chart-image certificate, with no pair lane at any scalar;
- a curve that must be derived numerically takes a **witness**, admitted where its transversality or order-k jet margin certifies.

So the doors state the cap rim by its closed form at every scalar, and no lane-less scalar remains for them. The build is these units, in this order. Units 3 and 4 land before the conventional form is removed, so nothing that passes today refuses:
1. The selector type, with per-surface certification of structural selectors.
2. Sweeps, lofts and blend rails state structural selectors, and construction-time `classify_dihedral` at joins retires. This includes this item's doors.
3. Witness certificates through the analytic pose catalogue.
4. The second-order lever arm read over the faces, not the edge's chord.
5. An in-band coincidence refuses at the split / boolean / finish decision. `split_through_an_apex`'s in-band corner rows re-baseline to refusals, which Ev ruled fine.
6. Import recovery: closed-form structural selectors (boundary isos, the analytic catalogue), typed refusals otherwise; STEP's synthesized sketches retire.
7. Offset and shell read the selector.
8. Order-k and freeform witness lanes, later.

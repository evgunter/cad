---
id: spline-incidence-is-reached-from-incidence-readers-open-to-a-dual
kind: issue
title: spline_incidence.rs is certification arithmetic reached from point_incident and incident_as at T: Decide, which a dual instantiates
status: closed
closed: 2026-11-25
opened: 2026-11-20
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found building `certification-interiors-are-closed-to-a-dual-by-a-marker-bound`, 2026-11-20. Measured by type-check: the bound was written and `cargo check -p geom-brep` read.

## What

`crates/geom-brep/src/pcurve_cache/spline_incidence.rs` is one algorithm with private types (`Net`, `Cell`, `Queued`, `Read`): a point's distance to a spline chart, by subdivision with `f64` rounding pads. Nothing in it is a body's data, so the whole file can take `Certifies` and be listed in `scripts/gates/certification-interiors.sh`.

Its two doors, `spline_incidence_as` and `spline_point_incidence_as`, are called from `pcurve_cache.rs`'s `incidence::incident_as` and `incidence::point_incident`, both `T: Decide`. Those are the incidence readers every pcurve door ends through, including the arm a scalar with no fitted lane takes (`certify_stated_image`'s `None` arm, `incidence_ended_unwitnessed`), so a dual with an edge on a spline chart instantiates the subdivision today.

## What it asks

The spline arm of the two readers split from the analytic arms: the analytic arms stay at `T: Decide`, and the spline arm is reached through a lane the scalar holds (`FittedLaneOf`, or a door of its own), so that a dual refuses there by type as it does at the fitted checks. What a dual's edge on a spline chart reads instead is the design question: the readers return a verdict the mint acts on.

Then `spline_incidence.rs` is bounded and listed.

## What is left, as of the fix pass on PR 7190 (2026-11-20)

Unchanged by the fix pass: `incidence::incident_as` and `incidence::point_incident` are `T: Decide` and call the subdivision with no lane, so this is the same shape as `the-certification-files-hold-body-data-beside-certification`'s (a): certification arithmetic a dual runs outside any lane. Unmeasured, as there: whether a dual test binary instantiates `spline_incidence_as` today (it needs a dual edge on a spline chart, and the dual corpus's bodies were not read for one).

## Dispatch, 2026-11-24

Dispatched as a unit on branch `scalar/spline-incidence-is-reached-from-incidence-readers-open-to-a-dual`. Letter **M**; tier **SINGLE, FULL review**: the change is a bound on certification arithmetic plus a move of where a dual is refused, which reads as right or wrong only by following who matches on the verdict, so the review carries claims to falsify alongside the style lane. The implementer measures first whether any dual reaches the spline arm today, and stops with the question if DL5.1–DL5.3 do not settle what a dual's edge on a spline chart reads.

## Measured, 2026-11-24: stopped at what a dual reads

**Who reaches the spline arm.** `spline_incidence_as` has one caller, `incidence::incident_as`; `spline_point_incidence_as` has one, `incidence::point_incident` (and the `rows` test, directly, at `f64` and `Dual64`). `incident_as`'s callers `derive_class_zero` and `spiric_chart_pcurve` are on analytic charts only, so its spline arm is reached through `read_edge_incidence` alone: from `read_shown` (so `read_incidence`), from `incidence_ended` (`PcurveCache::certify`, `certify_fitted`, `certify_general`, `recertify`), from `incidence_ended_unwitnessed` (`stated.rs`, `certify_stated_image`'s `None` arm), and from `topo`'s `pcurves::mint::nurbs_iso_derive` (the general arm and the catch-all arm). `point_incident` is reached through `read_point_incidence` alone, whose production caller (`topo::validate`, tier 3) passes a plane. `certify.rs`'s `stated_analytic_read` and `read_net_bound` reach only analytic charts. Every one of these is generic at `T: Decide` (topo's at `T: AtRestPolicy`), so each is instantiated at every scalar its callers build: `f64`, `Interval`, `Probe`, `Sym<_>`, `Dual64`, `DualInterval`.

**A dual executes the arm today, in tests that pin what it reads:** `geom-brep` `spline_door_rows::at_a_dual_an_off_carrier_ends_off_the_chart_not_at_the_fitted_door` and `a_defective_general_row_refuses_as_its_defect_at_a_dual` (through `certify_general` → `incidence_ended`), `spline_incidence::rows::at_a_dual_a_point_reads_what_it_reads_at_f64_across_the_band` (directly), and `topo` `a_spline_faces_stated_image_ends_by_its_carriers_incidence::at_a_dual_an_off_carrier_is_a_finding` (through `validate_pcurves` → `certify_stated_image`'s `None` arm) and `an_off_iso_straight_edge_takes_its_general_row::at_a_dual_the_diagonal_has_no_fitted_door_and_the_faces_stay_uncached` (through `nurbs_iso_derive`'s general arm). Read from the code paths, not run with a probe. Whether the dual corpus (`editor-core` `m10_di_dual_corpus`'s `loft_prism` at `Dual64` and `DualInterval`) executes it is unmeasured: a runtime probe over every test module naming a dual in seven crates was built twice locally and timed out twice in the build (the `editor-core` test binary did not link inside a four-hour slot hold). Read statically, the loft's wall edges take closed-form iso doors, which reach the read only where the door refuses or does not cover its span.

**What the verdicts feed.** `nurbs_iso_derive` returns a definite off read as the mint's refusal and goes on to `derive_general_image` otherwise, which at a dual refuses `FittedLaneUnsupported`; `topo`'s `not_owed` excuses exactly that refusal at a scalar with no fitted lane (`mint_rows`, `tier3`'s `validate_pcurves`), so the read is what keeps a dual's carrier off its spline wall a finding and not an excused face. `incidence_ended` replaces a door's refusal with the read's where the read refuses, and reads where a held door's certificate does not cover the span asked.

**What a dual reads at the arm today, in value terms.** The doors take the chart and carrier at any scalar and read them through `value_view()` / `without_tangent()`; everything below (`span_incidence_as`, `read_samples_as`, `Net`, the pads) is bounded `Decide<Value = T>`, which no dual meets. So the subdivision is never instantiated at a dual: a `Dual64` runs it at `f64` on its value channel, a `DualInterval` at `Interval`, a `Sym<Dual64>` at `Sym<f64>`, and the verdict is the base scalar's bit for bit (pinned by the `rows` test above across the band). The pads are `f64` pads applied to `f64` arithmetic, and the tangent never enters. The verdict is right, by construction and not by luck, as the base scalar's verdict on the value channel. What remains open is the two doors: they are generic at `T: Decide`, so the file cannot be listed whole.

**Whether DL5.1–DL5.3 settle what a dual reads once the arm is closed: they do not.** DL5.3 says "A dual belongs where the build and its measurements run, and passes through decisions on its value alone (DL1, DL3). Code that certifies is closed to it by type, on its interior as well as at its doors". The read is both: the mint's decision whether a carrier lies on its face (which the first sentence passes through on the value), and the read that ends a certificate door (which the second closes, as does DL5.1's "A read that mints a certificate object ... takes a lane door"). DL3 points both ways too ("refuse typed at the check that asks for it"; "a dual's door mints and never checks"). The readings and their costs:

- (a) **A typed refusal naming the missing lane.** If it is `FittedLaneUnsupported`, `not_owed` excuses it, so a dual's carrier off its spline wall is carried silently: both `at_a_dual_an_off_carrier_*` rows flip, and the catch-all arm's `IsoUnsupported` becomes an excused refusal. If it is a kind of its own, nothing excuses it, so every dual face on a spline chart whose mint reaches the read refuses (the diagonal row's `Ok(0)` becomes a refusal; the corpus is unmeasured). Either way every pcurve door's refusal at a dual on a spline chart ends as the lane refusal rather than as the read.
- (b) **The base scalar's verdict, with its claim stated.** Today's values, no consumer moves. The doors take `T: Decide` with `T::Value: Certifies` (a scalar whose value channel certifies), the interior takes `Certifies`, and the file can be listed (the gate reads the name in the record). The claim to state: a dual's incidence verdict is the base build's decision, not a certification of the dual.
- (c) **No read at a dual** ("mints and never checks"). The off-carrier rows flip as in (a), and `incidence_ended` at a dual takes a door's own refusal and its uncovered spans as they stand.

## Superseded, 2026-11-25

Ev ruled the question this stopped on (`[ev]` PR 7719): a dual never runs the incidence read, and the pcurve door splits into derive and certify. The work is `the-pcurve-door-is-two-functions-and-a-dual-holds-no-proposal`; this file's measurements are that unit's starting point. No code changed under this id.

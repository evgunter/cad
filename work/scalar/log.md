# SCALAR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/scalar/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, all seven from `work/code-quality/` — this is the one track
the cut built entirely out of that directory.

It is small on purpose. The scalar rows that already had a fix written
(`placement-lifts-its-affine-by-hand-beside-affine3-map`, the two
profile lift doors) went to the seat's successor WIRE and to DOOR, where
their class puts them; what is left here is what actually needs the
substrate decided first.

No branch exists yet. The first act is an `[ev]` PR carrying `D6`,
`D283` and the unit-vector question as one conversation.

## Orchestrator seated; the eighth row on the table (2026-09-12)

A SCALAR orchestrator is seated (remote box; branch prefix `scalar/`,
orchestrator branch `scalar/orchestrator`). DOOR re-homed
`curve3-eval-and-deriv-at-one-t-run-two-basis-passes` here on
2026-09-12 after the plan's slate table was written; the table now
carries it at class **M** beside `S393`, and the program text counts
eight rows. Sequencing, per the plan and Ev in-chat (2026-09-12):
the `[ev]` PR carrying `D6`, `D283` and the unit-vector question is
drafted first; `D290` dispatches beside it rather than behind it;
`H5`'s own questions (Q1, RingInterval) go to a SECOND `[ev]` sitting
once the door rows are in and its decomposition is cut.

## The first `[ev]` sitting: D6, D283, the unit vector (2026-09-12)

Three surveys of the tree, read by the orchestrator before the
recommendations were written; each row's `## Put to Ev` section carries
the decision and the rejected alternative, this entry the evidence.

**D6.** No `SenseSign` exists. `OutwardNormal<T>` (`geom-brep/src/enters.rs`)
wraps the product and takes the bit; `ReferenceNormal` is its sibling.
`Face::sense_sign<T>()` (`topo/src/entity.rs`) is the only mint of the
±1. Bare-`T` doors: `geom_brep::classify_material_pairing`
(`dihedral.rs`; callers `census.rs` `ee_cross_backed`, `validate.rs`
check 4, `rim_wedge.rs`), `material_kappa_rel` (`dihedral.rs`),
`rim_wedge::classify_shared_rim` (caller `boolean/mod.rs`
`verify_tangent_declaration`), `curved_face` and its private `sphere`
(`props/curved.rs`; caller `topo/src/props.rs` `face_flux`). Hand
multiplies `normal * face.sense_sign()`: `emit_topo.rs` `face_plane`,
`blend/build.rs` `outward_of`, `blend/battery.rs` `outward`,
`boolean/join.rs` `ring_run_ccw`, `boolean/rest.rs` `face_carrier`,
`boolean/solid_contain.rs` `face_plane` and `face_geo`, `validate.rs`
check 6, `merge_faces.rs` `merged_outline_ring` and
`planes_declared_equal`, `mesh/walk.rs` `loop_polygon` (`f64`, an
area). No comparison on the ±1 anywhere; every comparison is on the
bit. Test-side hand-multiplies: `sweep/tests/common/orient.rs` and
three blend suites; `dihedral.rs`'s own tests pass `1.0`/`-1.0`
literals into the bare doors. `sphere`'s `s_f` is also minted from
`t_sign(linear_rim_side(..))`, a decided `Sign` that can be zero.
Defect filed: `work/bool/missing-face-in-verify-tangent-declaration-reads-as-sense-true.md`.

**D283.** `Exhaustiveness` and its writer `sweep` in
`geom-brep/src/ssi/exhaust.rs`; the lanes `account_r3` and
`account_chart_plane`, called from `ssi.rs` with `domain.floor(band)`
and `domain.floor(band) / speed`; `speed` from `NurbsBoxes::deriv_box`
via `nan_propagating_max`, a local. `SsiOutcome` carries no domain,
chart, lane or speed. Readers: `Display` (`ssi.rs`), a wildcard match in
`pcurve_cache.rs`, and `geom-brep/tests/m5_pr7_ssi.rs` with its helper
`assert_floor_is_the_meters_floor_over_the_chart_speed` and the pinned
`WALL_CHART_SPEED`. The guard for #762 is `!speed.is_finite()` and
`speed <= 0.0`; `work/curved/ssi-chart-speed-usability-boundary.md`
holds the finite-but-huge case. Precedents: `Margin`'s constructor doors
(`predicate.rs`, `docs/predicate-dimension-audit.md`), `Tol`
(`tolerance.rs`), `quantity::Length` (boundary-only, restates D6),
`PatchRegularity` (`offset_meters.rs`: `floor`, `speed_u`, `speed_v`,
`thinness()`), `SsiLimb` (`ssi/certify.rs`). Blast radius of the tag:
the two `Exhaustiveness` literals in `sweep`, the two chart-lane
signatures and call sites, a manual `Default`, `Display`; every test
destructure compiles as is.

**Unit vector.** `topo::query::UnitVec3<T>` (`new(v, band)` over
`decide_unit_direction`, funnel `DATUM_UNIT_NORM`, `get()`), consumers
`DatumValue::{Plane, Axis, Frame, AxisInPlane}`, `eval/wire.rs`
`frame_axes`, `AxisFrame`, `tube_args`, `frame_plane_lane`.
`profile::path::Dir<T>` (`from_unit` does not re-decide; mints
`unit_from_components`, `arc_fillet::carrier_tangent`, `reversed`).
The ladder: `Vec3::normalize` is a bare divide (poison on zero,
overflow/underflow rows open on PROPS' and FIX's slates);
`frame::definitely_positive` then `normalize` in `point_at`,
`path_start_frame`, `mirror_across_plane`; `sweep::revolve::axis`,
`profile::path::unit_from_components` decide first; `enters_material`
normalizes without the length question (FIX's row). Unchecked prose
preconditions: `Affine3::from_frame` (callers `wire.rs` ×2,
`anchor.rs`, `emit_topo.rs` ×4, `viewer/sketch.rs`, `pncad-py/doc.rs`
×2), `Vec3::orthonormal_basis` (`newell.rs`, `step-import/recognize.rs`),
`frame_from_unit_aim` (private; fed), the geom carrier fields under
`geom/src/lib.rs`'s at-rest rule, `SplitPlane.normal`, `slab_extent`,
`axial_radial`, `tangent::perp`, `blend/arms.rs` `perp_unit`,
`mesh/cert.rs` `dist_line_triangle`, `mate/coset.rs` `Subgroup`
fields, `measure.rs` `Carrier`. The opposite posture exists once:
`revolve/tube.rs` refuses a non-unit axis at tolerance. At `Interval`,
`UnitVec3::new` decides `‖u‖ − 1` to `Zero` for tight inputs and stays
sound on an overflowed enclosure (`query.rs` tests). `cross_len` is a
carried witness for `perp`'s length only; nothing carries `aim`'s.

The `[ev]` PR is `scalar/ev-newtypes`; `needs_ev` is set on all three.

## D290 and S393 dispatched; the seams announced (2026-09-12)

Both door rows are in flight on their own branches with a spec each
(`docs/D290-SPEC.md`, `docs/S393-SPEC.md`), block SCALAR-B1 slots 0 and
1; the block record is branch-side per the A/B log's redaction shape.

**Seams, announced here and on each PR when it opens.** `D290` reaches
PROPS' `crates/geom-core/src/spline/knots.rs`, `crates/geom/src/curves/nurbs.rs`
and `crates/geom-brep/src/offset_fit.rs`, and TRIM's
`crates/geom-brep/src/edge_nurbs.rs` — one `KnotVector` rescale door
with exact pinned ends, replacing the private `offset_fit::rescaled_knots`
and the inline map in `edge_nurbs::on_carrier_domain` (which does not
pin its ends, so its image domain can sit an ulp off the carrier
interval — the one behaviour change, argued in the spec). `S393`
reaches S-TCOST's and S-TINT's `crates/sweep/tests/*`, BLEND's
`crates/sweep/src/skin.rs` (docs only) and `demos/tour/src/skinned.rs`.

**S393's premise corrected before dispatch.** The row says no public
door hands out the start frame. `geom_core::linalg::frame::path_start_frame`
does, is public, and is already bound into Python; the unit is the two
copies going onto it, with the one semantic difference (a hard 0.9
helper cone against the door's decided reference ladder) measured
fixture by fixture. Class corrects M → E in the plan table via the PR.

**The third door row waits on D290.** The v-reversal door on
`NurbsSurface` needs the same exact-ends argument for a REFLECTED knot
vector (`k ↦ lo + hi − k`) that D290 makes for a rescaled one; the test
that rebuilds the net carries the knots verbatim, which is the same
point set only when the v knots are symmetric — a door has to say what
it does when they are not.

## The rate census, for the `[ev]` sitting (2026-09-12)

Ev asked on PR 2457 whether a kernel-wide `Rate` would improve
uniformity and semantic correctness. Every parameter ↔ meters crossing
in `crates/*/src`, by shape (sup = certified upper bound on a speed,
inf = certified lower, pt = pointwise sampled, exact = closed form):

- **Through `Margin::metered` (span · rate)**: `certify.rs`
  `nurbs_span_meter`/`interval_span_forward` (inf,
  `speed_lower_bound`); `pcurve_cache.rs` `param_rate` and its four
  consumers (line 1 exact / nurbs inf / circle radius / ellipse minor);
  `pcurve_cache.rs` `trim_containment` (SUP, `chart_stretch_sup`);
  the `pcurve_iso_*` slack meters (SUP, `nurbs_stretch_bounds`);
  `topo/pcurves.rs` `pcurve_loop_continuity` v-channel (`v_meter`:
  exact polar arm or SUP); `topo/split.rs`
  `split_edge_param_interior` (inf per kind); `splitting/classify.rs`
  conic roots; `chord_join.rs` `split_tangent_chord_forward` (exact).
- **Through `Margin::levered` (angle · arm)**: `pcurve_loop_continuity`
  u-channel (`azimuth_arm`: exact per analytic kind, SUP on splines);
  `pcurve_azimuth_period`, `chart_windings` (`azimuth_lever`);
  `chart_bound.rs` `assembled`.
- **Through `levered_inv` (÷ rate)**: `ssi/certify.rs`
  `ssi_foot_orthogonality` (pt jet norms); two curvature uses
  (`kappa_rel`, 1/m) and one length use — not rates.
- **Through `over_lever`**: `offset_meters.rs` `offset_normal_floor`
  and `PatchRegularity::thinness` (inf area rate ÷ max SUP speed; the
  only struct storing named speeds).
- **By hand beside a predicate**: `ssi.rs` `plane_nurbs_ssi` (SUP,
  u/v max-folded, three divisions: seed floor, pad, floor);
  `ssi/certify.rs` limb-3 tube (SUP per axis) and transversality
  (`stretch`); `chart_region.rs` `certified_arms` (inf per kind,
  gated through `Margin::of`); `pcurve_cache.rs` slack products
  outside the door; the sphere `polar_rate` composition debt.
- **By hand outside the decide seam, by design**: the SSI march
  (`coordinate_scale`, `tangent_speed`, pt); `offset_fit.rs`
  `directional_mark` (structure selection); `coherence.rs`
  `gap_is_noise` and `mesh/walk.rs` (lever 0 = every gap noise, on
  purpose); `mesh/sizing.rs` `sagitta_step`/`curvature_step`/
  `ellipse_step`/`torus_grid_steps` and `mesh/nurbs_cert.rs`
  `split_steps` (second-order, m/param²); `mesh/chords.rs`
  `nurbs_tighten` (chart units per `t`); the projection Newton
  acceptances (pt); `enters_material_order2` (pt, squared).
- **Hazards a single type would blur**: bound direction (the
  `metered` doc promises inf, three sites pass SUP — PROPS' row
  `metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup`);
  per-axis vs max-folded (`ssi.rs` vs limb 3 — TRIM's row
  `ssi-tube-pad-folds-both-axes-by-max-speed-where-limb-3-proved-per-axis`);
  the u/v door asymmetry in one predicate (TRIM's row
  `loop-continuity-meters-u-through-levered-and-v-through-metered`);
  certified vs sampled; second-order and param→param rates; 0 and ∞
  meaning different things per site (the #762 guard, `thinness`
  unguarded by contract, `gap_is_noise`'s zero lever); the NaN-fold
  divergence `nan_propagating_max` vs plain `f64::max`
  (`work/curved/ssi-lever-arm-min-fold-hides-poison.md`).

Counts: door-mediated 20, hand-spelled beside a predicate 8, outside
the seam 9, non-rate `levered_inv` uses 3. The recommendation put to
Ev: `SupSpeed`/`InfSpeed` in `geom-core` beside `Margin`, scoped to the
linear m-per-parameter crossing; everything in the last bullet stays
out.

## The first sitting closed; six units cut (2026-09-15)

PR 2457 merged with all three answers: `D6` — bit and product, never a
±1 type, `sense_sign` retires; the unit vector — a witness in
`geom-core` minted by the decided ladder, then a frame witness the tube
door takes; `D283` — route A, the `SupSpeed`/`InfSpeed` pair in
`geom-core` with the receipt's lane tag riding it (Ev, in-chat,
2026-09-15: "that plan re D283 sounds good"). The three rows are closed
and six units are on the slate (plan §The ratified units). `D290` and
`S393` are in review; the third door row waits on lane capacity.

## D290 merged (2026-09-15)

PR 2461, block SCALAR-B1 slot 0, ordinal 4100; dual review, both arms
APPROVE (with fixes), no MAJOR on either — no tally candidate. What
landed and what moved: `work/scalar/D290.md` §Closed. The spec is
deleted per the ledger. `S393`'s fix pass and VREV's dual are in
flight; the block closes when VREV's reviews conclude.

## S393 merged (2026-09-15)

PR 2466, block SCALAR-B1 slot 1, ordinal 4101; dual review, both arms
APPROVE WITH FIXES, no MAJOR — no tally candidate. Both reviews
independently measured the quarter-turn identity's sign and the
merge conflict; one widened the class of hand-rolled frames to nine
sites, the other found an identity placement in the unit's own file.
What landed: `work/scalar/S393.md` §Closed. The spec is deleted per the
ledger. Two door rows down, VREV's dual in flight.

## SENSE-DOORS dual concluded; the sample numbers re-derived (2026-09-15)

`sense-sign-doors-take-the-bit` (PR 2649, block SCALAR-B2 slot 0,
ordinal 4104): both arms APPROVE WITH FIXES, no MAJOR — no tally
candidate. Bilateral: the "bit for bit" receipt at `material_kappa_rel`
is false as written (NaN sign, signed zero, and at `Interval` the old
`±1` product was PADDED where the negation is exact — head tighter, no
verdict moves); stale `sense_sign` prose at the moved sites including
`crates/topo/README.md`; `blend/arms.rs` `trace` minting a `±1`
`side: T` field from a face bit — D6's class through all three sweep
patterns' blind spot, handed to the second unit. Unique R1: the sphere
guard's unguarded cylinder twin, `SphereFluxSide` beside
`MaterialSign`, a `classify_shared_rim` mutant the whole suite
survives. Unique R2: `classify_material_pairing` respelling
`from_chart` by hand. Both exercised the public surface and found the
same thing: a ball with ONE band reversed measures volume exactly
`0.0` through `mass_properties` with no error, and tier 3 names it
`LaminaWedge`. Twelve items adjudicated; fix pass dispatched on the
implementer's arm.

Found while preparing the block close: **the sample numbers collided.**
TRIM-3 PR-2 (PR 2554) recorded #201 on main ten minutes after D290
(PR 2461) merged with #201 branch-side; S393 then copied the visible
#201 and wrote #202. Main's first-parent order rules (the CERT
precedent): D290 #201, TRIM-3 #202, S393 #203, VREV #204. S393's row
is corrected here; TRIM's is TRIM's, announced on their PR; the
protocol gap is filed on META's slate as
`work/meta/ab-log-sample-numbers-collide-under-branch-side-block-records.md`.

## VREV merged; block SCALAR-B1 closes (2026-09-15)

PR 2627, block SCALAR-B1 slot 2, ordinal 4102; dual review, both arms
APPROVE WITH FIXES, no MAJOR — no tally candidate. One arm found the
door's exactness cost (decimal-symmetric knots refuse; the upstream
fix is BLEND's), the other that a reversed chart re-attached leaves its
pcurves stale and that `two_sum` was the tree's third copy. What
landed: the item's §Closed. The spec is deleted per the ledger. With
its reviews concluded the block's last slot is done: the branch-side
block record (pre-draw fields, draw, three rows) merges to main.

## Block SCALAR-B1 landed (2026-09-15)

The branch-side block record — pre-draw fields, the draw (byte 69),
and the three rows D290 (#201), S393 (#203), VREV (#204) — merges to
main with this entry, the META row for the sample-number collision
beside it. Block SCALAR-B2's pre-draw and draw stay on
`scalar/orchestrator` until its last slot's reviews conclude.

## UNITVEC merged (2026-09-15)

PR 2646, block SCALAR-B2 slot 1, ordinal 4103; dual review, both arms
APPROVE WITH FIXES. R2 ranked three findings MAJOR — the sweep's
freshness claim, the mirror ladder's "hands it on", the bare
`orthonormal_basis` still standing — all three about what the PR body
and docs CLAIMED rather than what the code computes (no verdict, bit
or refusal moved), so they are recorded as claim-class and excluded
from the tally per the instrument; no tally candidate. What landed:
the item's §Closed. The spec is deleted per the ledger. The `sin_cos`
mint waits for a customer; the frame doors taking the witness are the
next unit's (`frame-witness-and-the-tube-door`).

## SENSE-DOORS merged (2026-09-15)

PR 2649, block SCALAR-B2 slot 0, ordinal 4104; dual review, both arms
APPROVE WITH FIXES, no MAJOR — no tally candidate. What landed: the
item's §Closed. The spec is deleted per the ledger. Handed on: the
`blend/arms.rs` `±1` field (second unit), the frame doors (third).

## RATE-PAIR merged; block SCALAR-B2 closes (2026-09-15)

PR 2657, block SCALAR-B2 slot 2, ordinal 4105. The fix pass took all
twelve items and demonstrated M1 by execution (the frozen head's
`chart_stretch_sup` answered `SupSpeed(1)` for a cone whose true sup at
`v = 4` is `2`; the door now refuses and a `v`-arm door mints the exact
1), so R1's finding **counts** for the OPUS arm under the instrument.
What landed: the item's §Closed. The spec is deleted per the ledger.
With its reviews concluded the block's last slot is done: the
branch-side block record (pre-draw fields, draw, three rows) merges to
main. Handed on: the angular arms (TRIM), the second D283 unit
(`exhaustiveness-receipt-carries-its-lane`).

## Block SCALAR-B2 landed (2026-09-15)

The branch-side block record — pre-draw fields, the draw (byte 187),
and the three rows UNITVEC (#205), SENSE-DOORS (#206), RATE-PAIR
(#207) — merges to main with this entry. One tally candidate in the
block, RATE-PAIR's M1, counted for the OPUS arm (unilateral,
contract-API class, demonstrated by execution in the fix pass); the
other five arms found no MAJOR. Block SCALAR-B3's pre-draw and draw
stay on `scalar/orchestrator` until its last slot's reviews conclude.

## EXHAUST-LANE merged (2026-09-15)

PR 2667, block SCALAR-B3 slot 2, ordinal 4106. What landed: the item's
§Closed. The spec is deleted per the ledger. The fix pass corrected a
causal story nobody had run — the lesson is on the TRIM row's
provenance note — and gave the two chart doors one crossing shape.

## FRAME-WITNESS merged (2026-09-15)

PR 2675, block SCALAR-B3 slot 1, ordinal 4108. What landed: the item's
§Closed. The spec is deleted per the ledger. The one finding both arms
made — `from_aim` trusting a perpendicular — is what the spec asked
for by name; the fix pass sealed it inside the module and gave the
five hand-rolled axis-and-reference ladders one door. The lily digest
moved last-bit under the mint and is re-baselined with its cause named.

## SENSE-FOLD merged (2026-09-15)

PR 2668, block SCALAR-B3 slot 0, ordinal 4107. What landed: the item's
§Closed. The spec is deleted per the ledger. The `arms.rs` question the
item posed resolved to the first reading — a sense wearing a `T` — with
the selection given one home rather than a negation at every consumer;
the tree-wide guard pins the three sanctioned scalar negations by name
instead of asserting zero, which is what D6 §0 prescribes.

## Block SCALAR-B3 landed (2026-09-15)

The branch-side block record — pre-draw fields, the draw (byte 207 ⇒
slot 0 FABLE), and the three rows SENSE-FOLD (#211), FRAME-WITNESS
(#210), EXHAUST-LANE (#208) — merges to main with this entry. No tally
candidate in the block: SENSE-FOLD's arms found no MAJOR; EXHAUST-LANE's
one MAJOR and FRAME-WITNESS's headline MAJOR were each found by both
arms (the second FRAME-WITNESS MAJOR is R1's two MINORs, the same
class). The v6 tally is unchanged from B2. The sample numbers collided
a second time under the branch-side shape — PROPS' mignitude-floor
(#2469) claimed #208 while EXHAUST-LANE held it branch-side and had
merged earlier in main's first-parent order; PROPS is told on its PR
that its row is #209, and the META row carries the instance. With B3
landed every unit the ratifications named is on main; what remains on
the slate is the plan's residue.

## RATE-PAIR dual concluded; a tally candidate (2026-09-15)

`rate-pair-in-geom-core` (PR 2657, block SCALAR-B2 slot 2, ordinal
4105; byte 94 ⇒ R1 OPUS, R2 FABLE on frozen `711236057`). Both arms
APPROVE WITH FIXES. R1 ranked two findings MAJOR: **M1** —
`chart_stretch_sup`'s Cone arm mints `SupSpeed::new(1)` for a stretch
its own comment says no surface-level constant dominates (`|S_u| =
v·sin α`), a typed certification the code denies on a `pub` door; **M2**
— the sentence fencing the pair ("an azimuth arm is metres per radian")
is false for the plane/spline kinds four lines below it, where the u
channel is the same crossing the v channel was just moved to
`metered_sup`, and the TRIM row was closed on that sentence. R2 found M2
from the other side (its MINOR-1/MINOR-2: the u channel on spline
charts; `certify.rs`'s circle/ellipse arms levered where `param_rate`
mints the same numbers as `InfSpeed`) and reported "no mis-tag found"
on the producers — it did not see M1. Under the instrument M1 is a
**tally candidate**: unilateral (R1, the OPUS arm), contract-API
class, not a duplicate of a filed row (the cone's unit-arm placeholder
was a known limitation as a bare `T`; the false CERTIFIED claim is new
in this PR), fair pair (identical briefs). Its demonstration by
execution is owed by the fix pass (a cone row red on the frozen head's
semantics); the candidate is recorded as such branch-side and counts
when that row exists. Bilateral: the narration digest not
reproducible from its recipe; the direction rule stated four to six
times with two copies claiming uniqueness; `thinness()`'s new
dimensional sentence wrong on its own premise; class members the
census could not see (`clearance.rs` `chart_arms`, `ScaledFace::build`'s
arms, `chart_bound`'s `assembled`). Twelve items adjudicated; fix pass
dispatched on the implementer's arm.

## Block SCALAR-B3 drawn; two of three dispatched (2026-09-15)

The three second units are cut as block SCALAR-B3 — `docs/SENSE-FOLD-SPEC.md`,
`docs/FRAME-WITNESS-SPEC.md`, `docs/EXHAUST-LANE-SPEC.md`, each
ratified at dispatch from a read-only survey of the tree at `d71bb6a78`
(the surveys are the orchestrator's, stored off-tree). Pre-draw fields
and the draw (byte 207 ⇒ fable position 0: SENSE-FOLD FABLE,
FRAME-WITNESS OPUS, EXHAUST-LANE OPUS) are branch-side on
`scalar/orchestrator`. SENSE-FOLD and FRAME-WITNESS are dispatched;
EXHAUST-LANE waits for disk (two lanes building at once is what the
box holds while a third target dir would push it under 10 G). Seams:
SENSE-FOLD reaches TOPO, BOOL/CURVED, BLEND, WIRE, S-MESH, TCOST/TINT
and the unowned `entity.rs`/`face_normal.rs`/`enters.rs`; FRAME-WITNESS
reaches PROPS, WIRE, BOOL (`profile`), BLEND (`revolve/tube.rs`), LIB
(`pncad`, `pncad-py` tags and census) and the unowned `demos/tour`;
EXHAUST-LANE reaches TRIM's `ssi*` and TCOST/TINT's `m5_pr7_ssi.rs` —
announced on each PR when it opens. Found by the surveys and written
into the specs: two more `±1`-from-a-bit mints no `sense_sign` grep
sees (`boolean/contact_verify.rs:303-304`) and a probe-only helper
spelling the bit by hand to dodge the census (`topo/src/r2_probes.rs`);
two private types named `AxisFrame` in two crates (editor-core's
origin+u+v pair, sweep's 2-D revolve frame); the tube door deciding
unit-ness under a LEVERED band the frame mint will not use, so two
predicate names retire with it.

## B3: EXHAUST-LANE and SENSE-FOLD in review (2026-09-15)

EXHAUST-LANE landed first as PR 2667 (ordinal 4106; byte 14 ⇒ R1 OPUS,
R2 FABLE on frozen `0414760f5`) — the lane tag with the chart lane's
`SupSpeed`, metres by one method, the pinned `WALL_CHART_SPEED`
retired and reproduced by the kernel's own rate; one deviation of
substance (the receipt is no longer written by `sweep` — a private
tally, the two accounting doors attach the lane, because a seeding
path has no honest lane). SENSE-FOLD landed as PR 2668 (ordinal 4107;
byte 234 ⇒ R1 OPUS, R2 FABLE on frozen `8a3ca626d`) — fifteen sites
folded, `Face::sense_sign` deleted, the census guard retired to the
type system, the `arms.rs` `side` field now the bit with a conditional
negation at every consumer, digests identical. Both duals are
concurrent on their frozen heads; SENSE-FOLD's waited on disk until
EXHAUST-LANE's reviewers were done building. FRAME-WITNESS is still
implementing.

## EXHAUST-LANE dual concluded (2026-09-15)

PR 2667 (ordinal 4106): both arms APPROVE WITH FIXES. R1 ranked one
finding MAJOR and R2 the same finding MINOR — bilateral, so no tally
candidate: the TRIM row the unit filed states a zero-speed mechanism
execution contradicts (the NaN pads are swallowed at the span grid and
limb 3 certifies silently over the wrong cell — worse than filed), and
a new comment in `ssi/certify.rs` enshrines the same story. Bilateral
too: the FLOOR-TIE tolerance's rationale counting two roundings where
there are five; unit-less fields in the touched types. Unique R1: the
refusal's `Display` not printing the speed the doc says a refusal
reader needs; no public reader of the rate; the derivative-box sup
computed three times. Unique R2: a substring hole in one text
assertion. Twelve items adjudicated; fix pass waits for a lane (disk),
SENSE-FOLD's dual dispatched first (R1 OPUS, R2 FABLE on frozen
`8a3ca626d`).

## SENSE-FOLD dual concluded (2026-09-15)

PR 2668 (ordinal 4107): both arms APPROVE WITH FIXES, no MAJOR — no
tally candidate. Both reproduced the fold's bit identity (R2 the tour
digests at both commits; R1 a thirteen-digest user-shaped
differential) and the mutant tables. Bilateral: the prose overclaims —
"single place", "two homes", "the type system is the guard" — against
D6's class, which has no mechanical cover once the literal is gone.
Unique R2: three production sites negating a chart normal under the
bit by hand with no door (`shell.rs`, `measure.rs`, `wire.rs`), the
curved fold spelled four times with two behaviours, `outward_of` a
body-for-body copy of the `pub(crate)` door. Unique R1: the planar
mutant's survivors at two more sites unfiled, `sided`'s two siblings in
the same file, three open rows on other slates citing the retired
symbol (routed on the PR). Twelve items adjudicated; the fix pass waits
for disk behind the EXHAUST-LANE fix pass and FRAME-WITNESS.

## FRAME-WITNESS in review; block SCALAR-B3 fully in flight (2026-09-15)

FRAME-WITNESS landed as PR 2675 (ordinal 4108; byte 247 ⇒ R1 FABLE, R2
OPUS on frozen `c1d8a7ffe`) — `OrthoFrame` with four mints (one more
than the spec named: `from_aim_and_reference`, argued for the tube's
arbitrary reference), `Affine3::from_frame` retired, the sketch plane
and the tube door take the witness, three tube refusals and their
Python tags retired, two predicate names retired and one joined the
swept roster, `DatumValue::Frame` carrying the type wider than the
fence (announced). One golden moved and is named: the tour listing
digest on twenty lily files at ~1e-16, because the demo authored
frames from directions unit by intent and the mint normalizes them —
the reviewers are asked to judge that. The EXHAUST-LANE fix pass is
running; SENSE-FOLD's waits for disk.

## FRAME-WITNESS dual concluded (2026-09-15)

PR 2675 (ordinal 4108): both arms APPROVE WITH FIXES. Both ranked the
same finding MAJOR — `OrthoFrame::from_aim` is a public mint that
trusts its caller for perpendicularity, and both built a frame with
`det = 0.5` through it — so it is bilateral, no tally candidate; its
fix is a visibility change (the two callers are in `frame.rs`). R2's
second MAJOR — the retired "rigidity is an unchecked convention"
paragraph surviving in five places, two of them user docs now
describing the opposite of the Python door's shipped behaviour — R1
found in part (its MINOR-1/MINOR-2, the same class); bilateral too.
Unique R2: `to_affine`'s "`w = u × v`" false for aim-minted frames (the
aim is stored verbatim; the pin's corpus never built one), a row made
vacuous by the retired tube refusals, a new production funnel name
invisible to every K sweep. Unique R1: a misleading refusal text for a
reference on the axis, the lily's own hand Gram–Schmidt left in front
of the mint, the tube-frame helper in four copies. Thirteen items
adjudicated; the fix pass (OPUS, the implementer's arm) waits for disk
behind the two fix passes already building.

## Six units landed; the slate after the first sitting (2026-09-15)

With block SCALAR-B3 on main, every unit the first `[ev]` sitting
ratified is landed and the three door rows before them. The plan's
slate now carries two rows: the Curve3 jet door (dispatchable, opens
block SCALAR-B4 as slot 0) and `H5`, gated on the second sitting. This
entry lands with the seven branch-side entries above it, which were the
program's record while the blocks were open.

## The second `[ev]` sitting: H5 re-taken (2026-09-15)

`H5`'s five findings re-derived against main `51556b947` by a survey
lane (`/home/user/scalar-briefs/survey-h5.md` is the working copy; the
row's `## Put to Ev` carries what matters). The evidence behind each
verdict:

- **S1 MOVED.** Ring references: 753 lines / 25 files in `crates/*/src`
  (by crate: geom-brep 387, geom-core 269, topo 47, mesh 41, geom 9;
  heaviest `props/quad.rs` 195, `ssi/enclose.rs` 97, `spline/compose.rs`
  68, `topo/src/props.rs` 47). `Interval` cfg sites: 62 lines / 29
  files; `pub mod interval` gated (`geom-core/Cargo.toml:19`), the
  backend an unconditional dev-dependency (`:103`). Ingress:
  `RingInterval::from_certified` (`ring_interval.rs:184`), 80 sites,
  refusing on `Interval::is_certified()` (`interval.rs:226`, `:557`);
  `impl Bounds for Interval` (`:537`) decoration-blind by design,
  pinned by `decoration_seam.rs` and `certified_door.rs`. Inside the
  ring: 117 `is_poison()` reads in 24 files; ≤ 28 one-sided endpoint
  comparisons over the 14 heaviest files. Division: `ring_interval.rs:516-548`
  poisons a divisor not proven one-signed; the backend gives a
  half-line with `dec = Trv` (`interval-transcendentals/src/arith.rs:80-96`).
  The ring implements `CertifiedEnclosure` (`:310`) and `Enclosure`
  (`:316`); `spline::hull` reads `CertifiedEnclosure` (`hull.rs:105,252`).
  Ratified text touched by a move: C9 (`geom-brep/README.md:249-258`),
  `DESIGN.md:1297-1298` and `:1433`, DUAL-DESIGN DL4. Ev's only primary
  trace on S1: the 2026-08-18 in-chat verdict, agent record `1c48fb03d`.
- **S2 GONE.** `stackup.rs:485-488` (`Evaluation<Dual64>`, M10-4, PR
  1627); `analysis.rs:452/531`, `measure.rs:609`, `lane.rs:96`,
  `memo.rs:179`; 57 `src` files name `Dual`. DL1 ratified PR 1146
  (Ev, 2026-08-29); `CertifiedEnclosure` has no `Dual` impl
  (`real.rs:1418-1425`). `EvalScalar` eleven terms (`eval/mod.rs:2183-2195`).
- **S3 MOVED.** Impls at `props.rs:1903/1938/1979/2025/2068`,
  `pcurve_cache.rs:1608/1663/1711/1762/1819`, `chart_region.rs:483/509/535/565/595`;
  lines ChartRegionLane 182, PropsQuadLane 360, PcurveFittedLane 417 =
  959. `lane_name` readers `pcurve_cache.rs:3519`, `topo/pcurves.rs:1090`,
  `topo/transform.rs:454`, all on the absence arm. `Sym` arm two
  absences `props.rs:2025-2064`. `chart_region_overlap`
  `Decide + CertifiedBounds` (`chart_region.rs:680`). Eleven refusing
  `Dual` arms in `src`: the four kernel ones plus editor-core's `Lane`,
  `MinClearanceLane`, `ShellLane`, `SectionScalar`, `AxisScalar`,
  `SeedScalar`. DEFER trigger unbuilt (`topo/README.md:289-290`). The
  ruling: PR 867 (Ev, 2026-08-21), commit `5239936c2`, H-R3.
- **S44 GONE.** Earliest paraphrase `24e24ba29` (2026-08-01); the
  `Bounds` half on DL1 and `DESIGN.md:1330-1337` (`a0d2f441f`); the
  lane half on PR 867.
- **S55 CONFIRMED, folds into (1).** `real.rs:1331` trait, `:1352`
  blanket impl, `ring_interval.rs:316`; zero `T: Enclosure` bounds in
  `crates/*/src`; DL4 names it on `scripts/gates/bounds-allowlist.sh`.

Not determined: the build cost of ungating `geom_core::interval` (the
"24 s vs 24 s" has no home in the tree); which PR tightened
`chart_region_overlap` (only `-S` hit is merge `60a83e489`); whether any
of the 28 endpoint reads follows a division (U0 answers it); the
ring-specific test line count. The PR is `[ev]` and waits.

## The second sitting ratified (2026-09-21)

Ev answered PR 2701 on 2026-09-16 and confirmed the summary on
2026-09-21: the ring retired in the aggressive form (feature dropped at
the dissolve), a tighter bound re-baselines, the full no-trait cut with
the certified name kept on the `CertifiedBounds` door and a
`_structural` twin. `H5` carries the RATIFIED section; the plan carries
nine units in two chains (RING-0…3, LANE-0…4). Evidence: the survey
lanes' figures on the PR thread (compile cost measured on the shared
4-vCPU box; the no-lane count: 3 kernel traits, 15 impls, 959 lines, 14
call sites, 104 bounded signatures).

## CURVE3-JET merged (2026-09-21)

PR 2708, block SCALAR-B4 slot 0, ordinal 4109. What landed: the item's
§Closed. The spec is deleted per the ledger. The implementer lane died
on a usage limit after its final push and the box sat idle five days;
main's new `Spiric` curve variant then broke the exhaustive match at
the merge and took one arm on the implementer's arm. The fix pass
folded the one pair the spec had excused and wrote the located-span
walk once. Two protocol exposures recorded on the row.
## RING-1 merged (2026-09-21)

PR 2971, block SCALAR-B4 slot 2, ordinal 4110. What landed: the item's
§Closed. The spec is deleted per the ledger. The dual concluded with no
tally candidate (the one MAJOR bilateral and pre-fixed); the fix pass
took thirteen items and declined one. Both reviews' end-to-end programs
established that the lane-trait impls, not the feature, are what keeps a
default-build caller out of the kernel doors at `Interval` — RING-3 and
LANE-1..4 specs carry it. One exposure: the PR body's auto-appended
footer names a vendor and re-appends on every body update; the row
records it.

## LANE-0 merged (2026-09-21)

PR 2981, block SCALAR-B4 slot 1, ordinal 4111. What landed: the item's
§Closed. The spec is deleted per the ledger. The dual's bilateral
MAJOR — a fourth per-scalar trait where the spec said stop — was
settled by the orchestrator's ruling (the seam is the one method on
`AtRestPolicy`), at a cost the fix pass measured at 47 bound edits
and zero call-site edits. Two exposures recorded on the row: the item
file named the arm until the freeze; the frozen head's run was red on
the tracker lint after main closed FIX. Block SCALAR-B4's three rows
are complete; slot 2's reviews concluded earlier, so the block closes
with this merge.

## Block SCALAR-B4 closed (2026-09-21)

Three slots, three duals, zero counted tally candidates: CURVE3-JET
(FABLE, #227, no MAJOR on either arm), LANE-0 (OPUS, #229, both MAJORs
bilateral — the design settled by the orchestrator's ruling), RING-1
(OPUS, #228, the one MAJOR bilateral and pre-fixed). The block's
record — pre-draw fields (slots 1–2's class recorded as the plan's H
before the byte, the cut coming out E), the draw (byte 75), the three
rows — lands on main with this PR. Exposures the rows carry: the spec
commits' trailers naming a model (CURVE3-JET's R1 saw one; unit-branch
orchestrator commits carry none since), the item files naming the arm
until LANE-0's freeze, the PR bodies' auto-appended vendor footer, two
sample-number collisions with other programs' branch-side records
(the META row).
## RING-0 merged (2026-09-21)

PR 2993, block SCALAR-B5 slot 0, ordinal 4112. What landed: the item's
§Closed. The spec is deleted per the ledger. The dual's one unilateral
MAJOR (the allowlist sampled six exponents; `powi(-1)` disagrees the
other way) is recorded as the block's first tally candidate, on the
OPUS reviewing arm. Three facts carried into RING-2's spec: what
`from_certified` returns under cut (ii) decides whether two register
sites are hazards; the dry run's red set is conditional on the `hull`
and `clamped_to` guards; the register wants to be executable.
## Block SCALAR-B4 drawn; CURVE3-JET dispatched (2026-09-15)

The Curve3 jet door opens block SCALAR-B4 as slot 0 (byte 75 ⇒ slot 0
FABLE; slots 1–2, the first `H5` sub-units, OPUS — their pre-draw class
is the plan's H, fixed before the byte). Spec `docs/CURVE3-JET-SPEC.md`
on `scalar/curve3-jet`. The survey corrected the row: the whole-curve
order-2 jet `NurbsCurve3::ders` exists, so the door is `ders1`, its
sibling; the `upgrade.rs` pair moved to `geom-brep/src/dihedral.rs`; six
`NurbsCurve3<f64>` pair sites in the tour and the `pncad` example feed
S393's `path_start_frame` and are in the unit. Fence announced here:
PROPS (`crates/geom/src/curves*`), TOPO (`validate.rs`), BOOL and CURVED
(`boolean/{ops,contact_verify}.rs`), BLEND (`blend/battery.rs`,
`skin.rs`), LIB (`pncad/examples`), TINT/TCOST (`geom/tests`), the
unowned `certify.rs`/`dihedral.rs` (fence drawn in the PR, PROPS told)
and `demos/tour/src/skinned.rs`. Full v6 dual at review.

## CURVE3-JET in review; the second sitting answered (2026-09-21)

CURVE3-JET landed as PR 2708 (ordinal 4109; byte 84 ⇒ R1 OPUS, R2
FABLE on frozen `88c47557b`, impl CI 35039865934 green): `ders1` on
`NurbsCurve3` (and `NurbsCurve2` through the macro) and on `Curve3`,
thirteen sites and fourteen pairs folded, digests identical, one BOOL
row filed. The implementer lane died on a usage limit after its final
push; the PR body is its report. The box then sat idle five days. On
PR 2701 Ev answered the second sitting on 2026-09-16: the aggressive
unification (the ring retired, the feature dropped) sounds good; a
tighter bound re-baselines; the full no-trait cut sounds good, asked
as "a structure that lets duals run in the `Bounds`-only parts and
never enter `CertifiedEnclosure`" — which is what the cut is. The
RATIFIED section and the unit cut go on the sitting's branch next.

## LANE-0 and RING-1 specified; block SCALAR-B4 slots 1–2 (2026-09-21)

The first two units of the second sitting's cut, both E: LANE-0 (the
`f64`-only offset-fit absence becomes an `Option` hook on
`tier3_local_checks_marked`, `mint_offset`, `map_approx`; the three
methods leave `PropsQuadLane`/`PcurveFittedLane`) on `scalar/lane-0`,
and RING-1 (`geom_core::interval` unconditional, `interval-transcendentals`
a normal dependency, the feature gates only the instantiation; the
measured five-file patch) on `scalar/ring-1`. Both take the B4 draw's
OPUS arm (byte 75, slots 1–2); their pre-draw sentences are recorded
with the note that the arm was known when the cut was made. Fences
announced here: LANE-0 reaches TOPO (`validate.rs`), SHELL
(`replace_face.rs`, `transform.rs`), TRIM (`pcurve_cache.rs`), WIRE
(`verbs/shell.rs`), LIB, the unowned `topo/src/props.rs`, TINT/TCOST;
RING-1 reaches PROPS (`geom-core`), CIW (`ci.yml` only if a step
changes), GUARD (`scripts/gates/*`), `docs/DESIGN.md` Q1's phrase
(naming-only) and `docs/GENERICS-BUILD-COST.md`. Dispatch waits for
lane slots: the CURVE3-JET dual and its fix pass come first.

## CURVE3-JET dual concluded (2026-09-21)

PR 2708 (ordinal 4109): both arms APPROVE WITH FIXES, no MAJOR — no
tally candidate. Both executed the mutant table exactly (M1 reds the
differential row only; M5 141 rows in `sweep::all`; M3 nothing, and both
showed why: a NURBS-carried `Curve3` cannot reach the enum-door sites
through any user program today — the boolean refuses NURBS input
carriers before tier 3), both attacked the `Interval` hull fold with
their own corpora and found no bit. Bilateral: `circle_at`'s "one
expression" doc falsified by the `Circle` arm's re-spelling; the
located-span walk now written four times; `normal_start_place`'s stated
reason backwards. Unique R1: `param_near`'s `Circle` pair left standing
on a cost argument the door retires; the one-frame claim unguardable by
any bit row (a two-frame mutant stays green); the meter fixture a
638-byte copy; `ders1` missing from three totality rows; no curve value
door in `pncad-py`. Unique R2: the filed rim-wedge row misdescribes its
site and understates a bit move; the naming split with `Surface::jet`.
Thirteen fix-pass items adjudicated; the fix pass runs on FABLE. Two
exposures to record: R1 saw the spec commit's harness trailer name a
model in the PR's commit list (the orchestrator's commit — from here
unit-branch commits by the orchestrator carry no trailer); the review
brief named the pre-rustfmt head and the red first run as claim 9 (both
arms corrected it; both reviewed `88c47557b` and read run 35039865934).

## RING-1 in review (2026-09-21)

RING-1 landed as PR 2971 (ordinal 4110; byte 41 ⇒ R1 FABLE, R2 OPUS on
frozen `cf15f29bc5`, impl CI 35555608396 green): the interval scalar
compiles in every build, `interval-transcendentals` a normal
dependency, the feature gating only the instantiation; seven gates
gone in `geom-core`, nine test gates kept, 48 + 4 + 126 outside for
RING-3; a four-row ungated pin; the two prose re-wordings; the
measurement re-taken (153 → 152 s). One row filed
(`gate-on-the-type-in-prose-outside-geom-core`). The PR body's
auto-appended footer carried the session URL and was stripped on the
orchestrator's instruction — recorded as an exposure with the
spec-commit trailer. The brief tells both arms the spec commit is the
orchestrator's.

## RING-1 dual concluded (2026-09-21)

PR 2971 (ordinal 4110): both arms APPROVE WITH FIXES. R2's one MAJOR —
four excluded-root lockfiles at the frozen head without the backend
edge, `cargo metadata --locked` exiting 101 — is R1's MINOR 1, the same
finding, and the branch had fixed it (`30ca6a8697`) before either
review landed: bilateral, no tally candidate. All ten claims held on
both arms by execution (the cfg re-count, the pin under three reverse
patches, the nextest lists at both feature sets, the 22 gate scripts,
the wasm32 check at default, the run's per-job conclusions). Bilateral
too: stale gate prose in files the PR edits; the build-cost doc's §9
re-interpreting §3's cell and thin on environment. Unique R1: the root
manifest's dev-dependency sentence; the `locate.rs` gate having no
independent pin. Unique R2: two pin rows monotone the wrong way (probe
rows filed by both arms); the module doc's "two roles" now separated by
prose alone; and the e2e fact that carries forward — with
`CertifiedBounds` unconditional, the lane-trait impls are the whole
remaining gate on the kernel doors at `Interval`, so ruling 3's cut,
not RING-3's feature drop, is what opens them. Fourteen fix-pass
items adjudicated; the fix pass runs on OPUS.


## LANE-0 in review; RING-1 state-synced (2026-09-21)

LANE-0 (PR 2981) froze at `667e77f7e5` after its code head `5306a7b81b`
ran green (35565435308; the first head red on three rows of the
`interval, eps = 1e-12` shard, fixed by the lane). Ordinal 4111 claimed
on main (PR 2988); byte 16 parity 0 ⇒ R1 OPUS, R2 FABLE, concurrent,
briefs stored before dispatch. The unit's deviation — the `Some` read at
a new per-scalar seam trait `OffsetFitScalar` (supertrait of
`PcurveFittedLane`) rather than threaded from the `f64` seam arms,
priced at 18 public doors and ~1,100 call sites — is the brief's first
claim and decides the verdict's severity. The freeze commit is the
orchestrator's: the item file said "slot 1 (OPUS)", the spec commit's
wording; RING-1's item carried the same words through its whole dual
(recorded on its row as an exposure). RING-1 is state-synced
(`859cbd3880`, main merged) and waits on its run before merging.

## RING-0 specified; block SCALAR-B5 drawn (2026-09-21)

RING-0 (`docs/RING-0-SPEC.md`, item `ring-0-poison-differential`, branch
`scalar/ring-0`): the per-op verdict assertion (ring poison ⇔ backend
`dec < Def`, NaI or empty) with a closed allowlist and an adversarial
corpus, merged; the newtype-over-`DInterval` dry run on
`scalar/ring-0-dry-run`, never merged, its red rows classed for RING-2;
the 28 endpoint reads dispositioned. Fence announced here: TCOST/TINT
(`crates/geom-core/tests/ring_interval_differential.rs`, `test-utils`'s
fuzz helpers only if needed). Block SCALAR-B5's pre-draw fields (RING-0
M / test lane; LANE-1 M; RING-2 H, Fable spec) were recorded before the
byte: **244** ⇒ fable position 1 (slot 0 RING-0 = OPUS, slot 1 LANE-1 =
FABLE, slot 2 RING-2 = OPUS). RING-0 dispatches when a lane slot frees
(three are taken: LANE-0's implementer closing out, its two reviewers).

## LANE-0 dual concluded; the seam ruled (2026-09-21)

Both arms REQUEST CHANGES on frozen `667e77f7e5`: R1 (OPUS) 2 MAJOR /
4 MINOR / 5 NOTE, R2 (FABLE) 1 MAJOR / 4 MINOR / 9 NOTE; both executed
base-vs-head byte identity through the public doors at every scalar
and moved no bit. Both MAJORs bilateral ⇒ no tally candidate: the
design (`OffsetFitScalar`, a fourth per-scalar trait bundled as a
supertrait of `PcurveFittedLane`, where the spec said stop), and the
"bit for bit" rows that compare the door against its own bodies and
stay green under a `recertify` wired to the `_at` instrument or a
`remap` without the window rule (R1 at MAJOR, R2 at MINOR). The
orchestrator's ruling, recorded in the fix brief: the `Some` is read
at the seam ruling 3 keeps — one method on `AtRestPolicy` (DL3's
per-scalar policy home), `f64` answering `Some`, the four others
`None` with their reason; the trait goes; the passes keep their
`Option` parameter; bound edits measured and listed, zero call-site
edits. Threading through the 18 public doors was declined because a
door only `f64` can construct would still be read off a per-scalar
seam by every generic caller. Unique R1: the fixture's bow shrunk
10⁴× so the refinement loop never runs (undisclosed); the `CHAIN`
roster not extended to the new file; `MinClearanceLane`'s §4 row
wrong twice. Unique R2: the census admits the instrument through the
whole new file; the plan and item still describe the spec's shape.
Fix pass dispatched on the OPUS arm; it also re-homes the FIX row to
WIRE (FIX left the tracker on main between the two runs).

## RING-0 in review (2026-09-21)

PR 2993 froze at `146ef44906` (run 35577652630 green, 39 jobs); the
dry run sits on `scalar/ring-0-dry-run` at `a1555ab6d0`, never merged.
What the implementer reports: a three-class allowlist (a zero times an
unbounded operand under `×`, unbounded over unbounded under `÷`, a
`powi` chain spanning zero and infinity), one fact — an indeterminate
IEEE corner the ring poisons on and the backend resolves at `Dac`;
division agreed on ~395k refusals per lane with zero disagreements;
the dry run 22 red of 3,771 at default, sixteen tighter pins, zero
looser, zero division verdicts, `topo` 0; 436 tighter / 0 looser over
the 960-row coefficient corpus (ruling 2's number); 23 endpoint reads
over 9 production files (not the survey's 28/14), 17 unguarded, none
reached by the dry run — RING-2's hazard list. Ordinal 4112 claimed on
main; byte 23 parity 1 ⇒ R1 FABLE, R2 OPUS, concurrent, briefs stored
before dispatch. Two rows filed on this slate.

## RING-0's fence, announced late (2026-09-21)

R1 noted the log carried no TCOST/TINT announcement for RING-0's reach
into `crates/geom-core/tests/ring_interval_differential.rs` (the
"RING-0 specified" entry named the ground but not the file). Announced
here: TCOST/TINT own that file; the fix pass also adds one witness row
to `crates/geom-core/tests/certified_door.rs` (the same ground).

## RING-0 dual concluded; the block's first tally candidate (2026-09-21)

Both arms APPROVE WITH FIXES on frozen `146ef44906`: R1 (FABLE) 0
MAJOR / 4 MINOR / 6 NOTE, R2 (OPUS) 2 MAJOR / 5 MINOR / 9 NOTE; both
reproduced the EFFORT-20 tallies bit for bit, the red-first, and the
960-row coefficient split (524 unchanged / 436 tighter / 0 looser);
R1 re-ran the whole dry run at default (22 red, every row and payload
matching the table), R2 most of it. **R2's M1 is unilateral, by
execution, class test-gap ⇒ a tally candidate**: the allowlist is
closed over six sampled exponents, not over `powi`; at `powi(-1)` the
backend refuses where the ring certifies (fourteen unmatched
disagreements on the file's own corner corpus, the direction the
module doc denies; `pow_pos` pads the base and the reciprocal divides
by a zero-touching bracket where the ring's `n = ±1` path never pads);
`-3` is the one negative exponent that agrees; production exposure
today zero. R1 found the same predicate's other face (class 3 wider
than the disagreement it excuses; no predicate asserts direction) at
MINOR and not the hole. R2's M2 (the endpoint register short by at
least five, `props.rs:2731` unable to launder under the dry run's
`from_certified`) is bilateral with R1's MINOR 2/3 ⇒ excluded. Both
corrected the dispatch: the survey's "division should show zero
disagreements" is false (one class, unbounded over unbounded; zero on
zero-touching divisors), and `ssi/enclose.rs` has no red row. For
RING-2's spec: what `from_certified` returns under cut (ii) decides
half the hazard list; the red set is conditional on the `hull` /
`clamped_to` guards; the register wants to be executable. Fix pass
dispatched on the OPUS arm.

## LANE-0's bound cascade, announced (2026-09-21)

The fix pass's ruling reached ground the spec's fence did not name: the
`AtRestPolicy` bound propagated to REACH's `crates/topo/src/boolean/ops.rs`
(nine doors), WIRE's `crates/editor-core/src/eval/wire.rs` (eleven
helpers) and `crates/verbs/src/run.rs` (two `impl` blocks), plus
`shell.rs` — 47 bound edits, zero call-site edits, disclosed at that
width in the PR body. Announced here to REACH and WIRE.

## LANE-1 dispatched; block SCALAR-B5 slot 1 (2026-09-21)

LANE-1 (`docs/LANE-1-SPEC.md`, item `lane-1-props-quad-lane-deleted`,
branch `scalar/lane-1`, the FABLE arm by byte 244) dispatched on
LANE-0's merged shape. The spec applies ruling 3's door rename
uniformly — the certified name keeps its quadrature at `Decide +
CertifiedBounds`, `_structural` twins carry the `None`, the existing
`_certified` twins fold into the plain names — and resolves the plan
sentence's gap the survey found: the lane-keeping tier-3′ doors that
run at `Dual` become the `_structural` twins, H-R3's capability moving
by name. Fences announced here: ATREST (`validate.rs`), SHELL, REACH
(`boolean/ops.rs`), CHART (prose), LIB/BIND (the prelude re-export, the
binding census entry), WIRE (`verbs/*`, editor-core's `checks.rs`,
`eval/mod.rs`, `verbs/shell.rs`), PROPS (`real.rs`'s allowlist prose,
DL3's sentence), GUARD (two gate entries, naming-only), PCERT (prose),
TCOST/TINT (eleven test files), the unowned `props.rs`.

## RING-2 dispatched; block SCALAR-B5 slot 2 (2026-09-21)

RING-2 (`docs/RING-2-SPEC.md`, a Fable spec; item
`ring-2-newtype-over-dinterval`; branch `scalar/ring-2`; the OPUS arm
by byte 244) dispatched on RING-0's merged artefacts: the dry-run
branch at `f70cdcee67` as the starting diff, the 22-row red table with
its classes, the 31-site register with its regenerating command, the
allowlist's four classes. The spec makes the three choices RING-0
could not: `from_certified` carries the decoration (the crossing
RING-3 needs), `hull`/`clamped_to` keep the refusing guards, and the
register becomes an executable census. Ruling 2 governs the re-pins:
tighter with the cause named, looser filed and left refusing. Fences
announced here: PROPS (`ring_interval.rs`, `spline/*`, `props/*`,
`offset_fit.rs`, `patch_bound.rs`, `ssi/*`, `geom/src/*`), TRIM
(`pcurve_cache.rs`), MESH (`chords.rs`, `nurbs_cert.rs`), SHELL
(`offset_meters.rs`), INSTR (`docs/tess-budget-data/`, by its recipe),
the unowned `topo/src/props.rs`, TCOST/TINT (36 test files),
`crates/geom-brep/README.md` C9 (naming-only). Two lanes live (LANE-1,
RING-2); the ring and the lane chains run in parallel until RING-3 and
LANE-4, which both wait.

## LANE-1 in review (2026-09-21)

PR 3010 froze at `e6e66d99f3` (run 35605853860 green; the first head
red on two reader-census rows that still ledgered the deleted identity
test). Ordinal 4113 claimed on main; byte 0 parity 0 ⇒ R1 OPUS, R2
FABLE, concurrent, briefs stored before dispatch. The brief's claims
lead with the fold's content (PR deviation 6): plain-name `f64`
callers of `validate_pseudomanifold`/`contact_marks` now run check 2's
plane × NURBS lane, which the lane-keeping door skipped — the spec's
table chose it without saying so, and both arms are asked to build the
body on which a verdict changes. The other deviations: `AtRestPolicy`
without `Bounds` as a supertrait (the allowlist gate), `connectedness`
certified (not callable at `Dual`), the fold's rename reaching six
unlisted files, three allowlist counts moved, the tour's `Scalar`
trait gaining `CertifiedBounds`.

## The box restarted mid-RING-2 (2026-09-21, ~13:50Z)

The container restarted while RING-2's implementer was between its
mesh commit and the geom-brep leg. Its worktree survived: five commits
past the spec (the newtype, the census and the collapsed allowlist,
three corpora re-pinned, geom's and mesh's dominance rows) were pushed
by the orchestrator as they stood, its 71-line uncommitted diff saved
beside the briefs, and a fresh lane on the same arm resumed from that
branch with instructions to re-verify everything before trusting it.
LANE-1's implementer had already reported (its file) and its frozen
head was already green; its dual dispatched on schedule. The restart
also killed every watcher; re-armed.

## LANE-1 dual concluded (2026-09-21)

Both arms APPROVE WITH FIXES on frozen `e6e66d99f3`: R1 (OPUS) 1 MAJOR
/ 5 MINOR / 6 NOTE, R2 (FABLE) 1 MAJOR / 4 MINOR / 6 NOTE; both ran
the two corpus dumps base vs head byte for byte at `f64` and `Dual64`
(318 rows, diff empty), the `compile_fail` for the right reason, the
wiring mutants, the red-first. **The one MAJOR is the same finding on
both arms, by execution** — the fold gives plain-name `f64` callers of
`validate_pseudomanifold`/`contact_marks` the certified door, so on a
corrupt M7-8 body the verdict changes from `VolumeUncomputable` to four
`EdgeCertification` findings, observable through `pncad-py`'s door and
two tour scenes; the spec's table chose the mechanism and its §1
denied the consequence — the spec's sentence did not survive ⇒
bilateral, no tally candidate. The orchestrator's ruling: the fold
stands (under ruling 3 the plain name IS the certified door; the base's
hybrid is what the cut removes) and owes a Python pin and a docstring.
Unique R1: DL3's re-wording inserted the wrong door into ratified text
(the gather calls the gates, not `_structural`) — restored to the
mechanism shape; the unguarded "ONE `lo` call" disclosure, guarded by
R1's own census row; the `_structural` suffix's two semantics
(`validate_geometric_structural` drops check 7, the tier-3′ twins run
it closed-form), a pre-existing ATREST shape now uniform — filed, not
changed. Unique R2: the evalscalar allowlist header written to the
spec's supertrait list, not the tree's; the unearned `Bounds` on the
props `_structural` doors. Fix pass dispatched on the FABLE arm.

## LANE-2 specified; block SCALAR-B6 drawn (2026-09-21)

LANE-2 (`docs/LANE-2-SPEC.md`, item `lane-2-chart-region-lane-deleted`;
dispatched after LANE-1 lands, on its shape): `ChartRegionLane` and its
five forwarding impls go; a two-pointer `RegionLane<T>` value with one
constructor at `Decide + CertifiedBounds` is threaded through the twelve
census signatures and `pseudomanifold_certificate_via`, the certified
twin supplying `Some` and the `_structural` twin `None`; `None` keeps
today's `CensusLaneUnsupported` and the `false` fold exactly, and the
unit adds the first rows that observe them produced. The survey
corrected the plan's ground: `census.rs` is CONTACT's, `chart_region.rs`
CHART's, the TOPO-shaped file reached is ATREST's `validate.rs`. Fences
announced here: CONTACT, CHART, ATREST, PROPS (`real.rs` prose, DL3),
GUARD (a selftest fixture spelling the `Sym` impl, two counts, one
header line), WIRE (prose), TCOST/TINT (`perf12_census_bvh_diff.rs`,
the census test files), the unowned `props.rs`/`lib.rs`. Block
SCALAR-B6's pre-draw fields (LANE-2 M; LANE-3 E; RING-3 M, Fable spec,
Ev-gated) were recorded before the byte: **153** ⇒ fable position 0
(slot 0 LANE-2 = FABLE, slot 1 LANE-3 = OPUS, slot 2 RING-3 = OPUS).
Ruling 3 supersedes the 2026-09-05 DEFER on this trait; the spec says so.

## RING-2's first hosted run: exact-zero bounds reach the meters (2026-09-21)

PR 3032's first head ran red on three rows the lane's local set could
not see: the mesh budget meter and `tools/tess-meter`'s rows (both
under the k-lint rows' features and roots) record an EMPTY certificate
for a flat NURBS face — its second-derivative bounds are now exactly
zero where the ring's one-ulp pad made them tiny and positive, so the
patch steps are infinite and no cell is sampled — and an interval-lane
`Display` row at eps 1e-12 (the interval arm was run at the default
eps only). The first two are a consumer that cannot digest a tighter
bound, the class the spec names for consumer changes: fixed at the
meter's source, never by widening the bound; the third is a message
re-pin or a chain fix, the lane's call with the cause named. Recorded
here because RING-3 dissolves this newtype into `Interval` and every
such consumer is on its path.

# The decisions each consumer makes on a turn's lever

The table lists every decision the consumers below make on a lever: a
`geom_core::Lever<T>` (`reach`, `lever`, `arm`) read straight, multiplied into a
dimensionless reading (`Margin::levered(x, arm)` = `x·arm`, `Margin::sagitta(k, arm)`
= `k·arm²/2`), or min'd into `folded_lever_arm` (`min(curvature arms, reach)`,
dihedral.rs:1173; `LeverBounds::min` keeps each bound's direction, extent.rs:734).
A longer arm leaves Zero sooner. The bound owed follows `crates/geom-core/README.md`,
"Float rounding", the lever bullet: a decision that acts only on a definite sign (its
Zero refuses or escalates) is owed the bound **from below**; one that acts on its Zero
is owed the bound **from above**; one that acts on every sign is owed **both** (decide
at each, act only where they agree). *Gate* marks a decision on the arm's own length
(acts on Positive only: from below). *Naming* marks a reading that only picks which
refusal an already-refused decision reports. *Indifferent* marks a decision whose
decided signs all lead to one action, so the lever moves only whether it escalates.
"CERT" is `EdgeCurve::certify`'s `run_checks`, which reads from below (rows C1–C5).

State of the code read: the doors now return `LeverBounds` (wedge_reach.rs:144-170,
`WedgeReach::reach`; decision.rs:297 `edge_reach`), and every production caller named
here takes `.below()` (e.g. mod.rs:8415, join.rs:1243, reduce.rs:3945, slit.rs:2030,
ops.rs:3122, finish.rs:2003, edge_wedge.rs:112/171, edge_join.rs:731, rules.rs:548,
adopt.rs:351-357, surgery.rs:6324-6331, decision.rs:1170/1186).

Paths: `dihedral.rs`, `edge_wedge.rs`, `certify.rs`, `enters.rs` are in
`crates/geom-brep/src/`; `topo/…` is `crates/topo/src/…`.

## Shared decisions (referenced by id from the consumer rows)

| Id | Decision (predicate) | Margin | Positive → | Negative → | Zero → | Escalation → | Bound owed | Cited at |
|---|---|---|---|---|---|---|---|---|
| S1 | `"dihedral_arm"` | `Margin::of(arm.length())`, `decide_positive` | proceed to S4 | gate refusal (`LeverEscalation::arm`) | gate refusal | escalation (arm rung) | below (gate) | `wedge_of`, dihedral.rs:1044 |
| S2 | `"dihedral_arm_wedge"` | `Margin::of(|sinθ·arm|)`, `decide_positive` | keep arm's margin | quote wedge | quote wedge | quote wedge | naming | `at_wedge`, dihedral.rs:1115 |
| S3 | `"dihedral_wedge_at_arm"` | `levered(sinθ, band escalate edge)` (unsourced, not the reach) | — | — | arm keeps margin | — | naming; lever is not the reach | `wedge_reads_zero_at_the_arm`, dihedral.rs:1131 |
| S4 | `DIHEDRAL_WEDGE` = `"dihedral_wedge"` | `levered(sinθ, folded arm)`, `decide_reported` | `Transverse` (Corner) | `Transverse` (unreachable for a magnitude) | `Smooth` (Tangent) | `NoTangentPlane` or `Lever(reading)` | per consumer | `wedge_of`, dihedral.rs:1062-1078 |
| S5 | `DIHEDRAL_SIGN` = `"dihedral_sign"` | `levered((n₊×n₋)·τ̂, arm)`, `decide_nonzero_reported` | `Convex` | `Concave` | escalation (`WedgeRung::Sign`) | escalation | below where the sign is used; indifferent where dropped | `Corner::convexity`, dihedral.rs:559-579 |
| S6 | `"material_wedge_side"` | `levered(cosθ, arm)`, `decide_reported` | `Aligned` | `Opposed` | S7, then `Perpendicular`, which `pairing()` refuses | escalation (`WedgeRung::Pairing`) | below | `normals_alignment` dihedral.rs:2244; `pairing` 2176; `Tangency::pairing` 602 |
| S7 | `GRADIENT_ALIGNMENT_LEVER` = `"gradient_alignment_lever"` | `Margin::of(lever.length())`, `decide_positive`, only on a Zero cosine | `Perpendicular` (refused by `pairing()` where read as a pairing) | refusal | refusal | refusal | gate / naming where the Zero is refused anyway | `normals_alignment`, dihedral.rs:2256 |
| S8 | `"material_pairing_wedge"`; `"material_pairing_offer_wedge"` | `levered(sinθ, arm)` `decide_positive`; `levered(tanθ, band edge)` | which refusal is quoted | — | — | — | naming | `pairing_at_wedge` dihedral.rs:2297-2315; 2322 |
| S9 | `TANGENT_SECOND_ORDER` = `"tangent_second_order"` | `sagitta(|κ_rel|, folded arm)`, `decide_reported` | in the walk: next station, then hook `after_positive` | `UnderDetermined` (unreachable for a magnitude) | `UnderDetermined` | `InBand` | per consumer | `second_order_verdict` dihedral.rs:1258; `second_order_walk` 1379-1386; `tangent_second_order` 1242 |
| S10 | `MATERIAL_CUSP_SIDE` = `"material_cusp_side"` | `sagitta(signed κ_rel, station.arm)`, `decide_nonzero_reported` | `Cusp` | `Slit` | stop: escalation (`WedgeRung::End`) | stop: escalation | below where the end is used; indifferent where dropped | `MaterialReads::after_positive`, edge_wedge.rs:482-502 |
| S11 | `"dihedral_arm"` (seam's lower end) | `levered(sinθ, fold(lo arms, reach))`, `decide_positive` | `Transverse` | escalation (arm) | escalation (arm) | escalation | below | `classify_seam`, dihedral.rs:928-932 |

The hooks of `second_order_walk`: `()` reads nothing (dihedral.rs:1328); `MaterialReads`
reads S6 before each station's S9 (edge_wedge.rs:465) and S10 after a Positive S9.

## Consumer rows

| Consumer | Decision (predicate) | Margin | Positive → | Negative → | Zero → | Escalation → | Bound owed | Cited at |
|---|---|---|---|---|---|---|---|---|
| MC `must_carry_over_edge` | S1, S4 at each interior station (`class_pass`) | S4 | any station Corner: `Transverse` (all) or `Mixed` | as Positive | all stations Smooth: go to S9 walk | `InBand` | per caller (below) | dihedral.rs:1529-1550; `class_pass` edge_wedge.rs:411 |
| MC `must_carry_over_edge` | S9 walk, hook `()` | S9 | all Positive: `JetDeterminate` if in lane (`Settled::tangency` 1596), else `UnderDetermined` | `UnderDetermined` | `UnderDetermined` | `InBand(Bend)` | per caller | dihedral.rs:1552-1567 |
| `must_carry_description` (chord_join `chord_spec`, `split_edge_onto`, sweep surgery) | S4 via MC | S4 | `Transverse` → `Refuted` (refusal); `Mixed` refused | as Positive | proceed (S9) | refusal (`escalated`) | above | dihedral.rs:1884-1903; `description` 2026-2041 |
| `must_carry_description` | S9 via MC | S9 | `TangentIntersection` stored, re-certified by CERT C3 from below | conventional | conventional stored (`OnPair`/chart image; no lever re-check in CERT) | refusal | both: MC read owes above, its Positive path gets below from CERT (refusal, not escalation, where they disagree) | dihedral.rs:2032-2037 |
| `reask_pair` | S4 via MC | S4 | `Corner` → `Intersection` (CERT C1 from below) | as Positive | proceed | `PairRefusal::InBand` | both (Positive path checked by CERT; Zero path not) | dihedral.rs:1938-1944 |
| `reask_pair` | S9 via MC | S9 | `TangentIntersection` (CERT C3) | `OnPair{declared}` | `OnPair{declared}` | `PairRefusal::InBand` | both (as above) | dihedral.rs:1940-1952 |
| `reask_chart` | as `reask_pair` | — | `Some(intrinsic)` | `None`: image kept | `None`: image kept | refusal | both | dihedral.rs:1971-1990 |
| WV `edge_wedge_over_as` | S1, S4 per station | S4 | Corner → S5 each | as Positive | Tangent → S6/S9/S10 | `InBand` | both | edge_wedge.rs:326-331, 350 (`Mixed` class) |
| WV | S5 per corner station | S5 | `Crease{Convex}` (or `Mixed` sign) | `Crease{Concave}` | `InBand` | `InBand` | below | edge_wedge.rs:336, 360-365 |
| WV | S6 per tangent station | S6 | `Seam{bend}` | S9's bend → `End`/`Lamina` (`material_outcome`) | `InBand` | `InBand` | below | edge_wedge.rs:465-475, 517-535 |
| WV | S9 walk, hook `MaterialReads` | S9 | `Bend::Determinate`; reads S10 | `UnderDetermined` | `UnderDetermined` (Opposed: `Lamina`) | `InBand(Bend)` | both | edge_wedge.rs:374-389 |
| WV | S10 after each Positive S9 | S10 | `End{Cusp}` | `End{Slit}` | `InBand(End)` | `InBand(End)` | below | edge_wedge.rs:489-502 |
| `edge_wedge_over` | = WV with `Role::Choice` | — | — | — | — | — | as WV | edge_wedge.rs:298-304 |
| `classify_seam` | S1, S4 over `fold(hi arms, reach)` (hi = reach for a spline face) | S4 | analytic pair: `Transverse`; bracketed: go to S11 | as Positive | `Smooth` | `SeamEscalation::Lever` | above on the hi read (its Zero is returned) | dihedral.rs:922-927; `seam_face_reading` 946-986 |
| `classify_seam` | S11 over `fold(lo arms, reach)`; `chart_arm(…, reach.length())` | S11 | `Transverse` | escalation | escalation | escalation | below on the lo read | dihedral.rs:928-932, 980 |
| `rim_wedge::classify_shared_rim` → `boolean::route_rim` | S4 via WV (`Role::Check`) | S4 | `Crease` → `RimRouting::Transverse` → `ClassificationInvariant` refusal; `Mixed` → `TangencyUnsupported` | as Positive | tangent path | `in_band` coincidence refusal | above | rim_wedge.rs:1163-1170; mod.rs:8418-8457 |
| route_rim | S5 via WV | S5 | `Transverse` refusal | `Transverse` refusal | refusal (in band) | refusal | indifferent | mod.rs:8452 |
| route_rim | S6 via WV | S6 | `Seam` → `departures`; `Opposite` glues (`Ok`), others `Unrouted` | `Cusp`/`Lamina` (by S9) | refusal | refusal | below | mod.rs:8421-8440 |
| route_rim | S9 via WV | S9 | Aligned: `Seam` (bend unused); Opposed: S10 → `Cusp` → `Unrouted` | — | Aligned: `Seam`; Opposed: `Lamina` → `ClassificationInvariant` refusal | refusal | Aligned: indifferent; Opposed: below | mod.rs:8421, 8440, 8452 |
| route_rim | S10 via WV | S10 | `Cusp(Cusp)` → `Unrouted` | `Cusp(Slit)` → `Unrouted` | refusal | refusal | indifferent | mod.rs:8440 |
| `pcurves::decision::singular_turn` | `"pcurve_singular_turn"`, `Period::angle().decide` (geom-core period.rs:237): one decision per half-period mark | `levered(gap − mark, lever.below())`, lever = `JointSides::lever` = min of two edges' reaches | mark cleared / not cleared: walk on, or `Ok(k)` → element `(k, twin)` | as Positive (sign picks side) | `OnMark`: `mates`-free slit → `Ok((step,false))` only where `rest.deck == wraps` and not twin; else `EdgesCoincide` refusal | `Undecided` → `PinMiss::Escalated` | both | decision.rs:1183-1205; period.rs:265-300; `lever` decision.rs:263 |
| `half_turn_off` | `"pcurve_singular_sheet"` | `levered(acrosŝ·along, lever.below())` | `false` (not twin) | `true` (twin) | `NoLever` refusal | escalation | below | decision.rs:1220-1245; read at 1170-1171 |
| `pcurves::seam_reading` | `classify_seam` (rows above), `chart_arm` = `T::spline_chart_arm` | — | — | — | — | `SeamUnread::Lever`/`Crease` | as `classify_seam` | topo/pcurves/mint.rs:527-548 |
| `boolean::join::stands_on` | `"bool_rim_face_opposed"`, `decide`, returned `== Sign::Negative` | `levered(±n·n_f, wall's reach)` | `false` | `true` | `false` | coincidence refusal | below: `false` makes `closes_at_its_site` false, the record is skipped (join.rs:997), counted (1402-1406) and refused `SingleSiteSectionLoop` (1918) | join.rs:1209-1252; caller 1140-1143 |
| `boolean::reduce::sheets_part` | `"bool_touch_sheets_part"`, `decide`, returned `== Sign::Negative` | `levered(into_x·into_y, max of two reaches)` | `false` → `Interior::Unseen` | `true` → `Pierce` (both edges split at `p`) | `false` → `Interior::Unseen` | coincidence refusal | both: `Unseen` is not a refusal here; `Placement::declared(…, false)` records, defers (`Deferred`) or keeps the frontier (reduce.rs:5524-5539); what a deferred edge becomes downstream was not traced | reduce.rs:3882-3953; caller `interior` 3820-3833 |
| `boolean::slit::walls_inside` | `"bool_rider_wall_side"` per wall | `levered(into·floor, wall's reach)` | `inside = false` | `inside = true` | bend rung below | coincidence refusal | both | slit.rs:2036-2048 |
| walls_inside | `RIDER_WALL_BEND_ACROSS` = `"bool_rider_wall_bend_across"` (Zero only) | `parting(Δbend, floor, arm)` = `sagitta(Δbend·floor, arm)` (enters.rs:615) | curved floor: `CurvedFace` refusal; flat: `inside`, `flat` | `inside = false` | curved: refusal; flat: `coplanar_wall_inside` | refusal | curved floor: below; flat floor: both | slit.rs:2074-2092 |
| walls_inside | `"bool_rider_wall_coplanar"`, `decide_nonzero` | `levered(n·floor, arm)` | `Tangent` refusal | `false` | escalation | coincidence refusal | below | `coplanar_wall_inside` slit.rs:2193-2210 |
| walls_inside (result) | — | — | `[true,true]` → `Some(flat)`: parted (slit.rs:437) or `unplaced` refusal (1116); `[false,false]` → `None`: not parted / "does not lie in the face"; mixed → `Crossed` | — | — | — | (as the decisions) | slit.rs:2097-2101 |
| `boolean::ops::describe_edges` | `seam_reading` (classify_seam) | S4/S11 | `Transverse` → `describe_intersection` (CERT C1) | — | `Smooth` → MC (`must_carry_reading`) | `DescribeRefusal::undecided` | both (Transverse path checked by CERT) | ops.rs:3115-3164 |
| describe_edges | MC via `must_carry_reading` | S4, S9 | S4 Pos: `Mixed(None)` refusal; S9 Pos: restate `TangentIntersection` (CERT C3) | — | S4 Zero: proceed; S9 Zero: `Conventional`, restated only if `stale` | refusal | S4: above; S9: both (Positive via CERT) | ops.rs:3195-3253, 3369-3383 |
| `splitting::finish::describe_section_boundary` | `seam_reading` | S4/S11 | `describe_intersection` (CERT C1) | — | `Smooth` → `edge_wedge_read` | `Undescribed`/escalation | both | finish.rs:2003-2026 |
| describe_section_boundary | WV via `edge_wedge_read` | S4, S5, S6, S9, S10 | S4: `Crease` → `SmoothJoinRefuted`; S6 Aligned: `Seam`, store; S9 Pos: `TangentIntersection` | S6 Opposed: `End`/`Lamina` → `KnifeEdge` refusal on a curved wall, stored on a plane | S4: accepted; S9: conventional | `WedgeEscalated` | S4: above; S5, S10: indifferent; S6: below; S9: both | finish.rs:2037-2070 |
| `boolean::edge_join::joinable` | S1, S4 via `classify_dihedral` | S4 | `Locus::Transverse`: join; `Locus::Tangent`: not joined | — | `Locus::Tangent`: S9; `Locus::Transverse`: not joined | `regularity` undecided | both: "not joined" keeps the vertex, and check 11 asks the same predicate (`joinable_at_rest`), so it is an answer, not a refusal | edge_join.rs:728-751 |
| joinable | S9 via `tangent_second_order` | S9 | join | not joined | not joined | `regularity` | both (same reading) | edge_join.rs:745-747 |
| `splitting::rules::edge_wedge` | S1, S4 via `classify_dihedral` | S4 | proceed | — | `None` → `wall_graze` | `SliverSector` | both | rules.rs:560-564 |
| rules::edge_wedge | `"enters_material_arm"` | `Margin::of(arm.length())`, `decide_positive` | proceed | escalation | escalation | `SliverSector` | below (gate) | enters.rs:529-531 |
| rules::edge_wedge | `"enters_material"` | `levered(into_mate·n_own, folded arm)` (`rise`) | `Some(false)` (`Exits`) → `side.opposite()` | `Some(true)` (`Enters`) → `side` | `None` (`Tangent`) → `wall_graze` | `SliverSector` | both | rules.rs:570-578; enters.rs:384-401; caller rules.rs:483-494 |
| `topo::edge_wedge::edge_wedge` | WV, `Role::Choice`, full verdict to caller (blend `battery.rs:1117`) | — | — | — | — | — | as WV (S5, S10 below; S4, S9 both) | topo/edge_wedge.rs:106-118 |
| `topo::edge_wedge::edge_wedge_read` | WV, `Role::Check`, `WedgeClass` + `WedgeWalk::description` | — | — | — | — | — | S4, S9 both; S5, S10 indifferent (class drops sign and end); S6 indifferent to the description (both pairings store by S9's bend, edge_wedge.rs:258-276), used by callers as class | topo/edge_wedge.rs:159-178 |
| step-import `adopt.rs` (`Description::Tangent`) | MC via `reask_pair` in `must_carry_rung` | S4, S9 | S4 Pos (`Intersection`) and S9 Pos: `None` → candidate `TangentIntersection` → CERT | — | S9 Zero (`OnPair`): `NotYetSupported` refusal | `RestJoinUndecided`/`RestJoinMixed` | below: every non-refusal path ends in CERT | adopt.rs:351-394, 470-496 |
| sweep `blend/surgery.rs` contact edge | `must_carry_description` | S4, S9 | S4 Pos: `SurgeryInvariant`; S9 Pos: `TangentIntersection` via `set_edge_curve` (certifying) | — | S9 Zero: conventional `.spec(rest)` stored | `BlendError::Escalated` | S4: above; S9: both (Positive via CERT) | surgery.rs:6319-6360 |
| C1 CERT `run_checks` Intersection | `wedge_decided` (S1, S4) | S4 | pass | pass | `NotTransverse` refusal | `ArmCollapsed`/`Escalated` | below | certify.rs:5088-5118 |
| C2 CERT Tangent | `"tangent_gap_orientation"` (`gradient_alignment` + `pairing`, S7) | `levered(cosθ, reach)` | σ = +1 | σ = −1 | refusal (named by C4, else `Escalated(TangentGap)`) | as Zero | below | `gap_orientation` certify.rs:3912-3965 |
| C3 CERT Tangent | `tangent_second_order` (S9), `Refused::of` | S9 | pass; arm folded into `tangent_arm_min` | `NotSecondOrderSeparated` | `NotSecondOrderSeparated` | `Escalated(TangentSecondOrder)` | below | certify.rs:5209-5254; recourse.rs:399 |
| C4 CERT | `"tangent_normal_parallel"` (`named_by_parallelism`, `folded_parallelism`) | `levered(sinθ, folded arm)` | renames a refusal | renames | keeps refusal | keeps refusal | naming | certify.rs:3985-4006; uses 3951, 5227 |
| C5 CERT Tangent | `"tangent_tube_margin"` (`tube_separation`) | `sagitta(κ_min − drift, tangent_arm_min)` | pass | `TubeNotSeparated` | `TubeNotSeparated` | `Escalated(TangentTube)` | below | certify.rs:1856-1872, 5414-5422 |

`tangent_arm_min` is read only by C5 (certify.rs:5057, 5254, 5414); it is not stored in
the certified record. The `tangent_normal_parallel` residual at certify.rs:5256 is
`levered_inv(sinθ, |κ_rel|)`, not the reach. Certification acts on definite signs only:
every Zero is a refusal and every reading that might act on a Zero is naming. That holds.

## Bounds needed, per consumer

- **MC (`must_carry_over_edge`)**: none of its own; each caller's mapping decides. S4 and S9 share one arm, so they cannot take different bounds unless the reach is passed twice.
- **`must_carry_description`**: above on S4 and S9 (their Zeros are stored unchecked); S9's Positive is certified from below afterwards. So: above in the door, below in CERT.
- **`reask_pair` / `reask_chart`**: both on S4 and S9; the Positive paths are re-certified from below, the Zero path (`OnPair` / image kept) is not, so above in the door is the part missing.
- **WV (`edge_wedge_over_as`)**: both (S4, S9); S5, S6, S10 below. Separable by decision but not by argument: one `reach` feeds `folded_lever_arm` for all.
- **`classify_seam`**: already split: its Smooth is read at the hi arm, its Transverse confirmed at the lo arm; with respect to `reach` the hi fold owes above and the lo fold below. Today one `reach` (below) feeds both.
- **`route_rim`**: above on S4 (a seam is glued on its Zero); below on S6 and on S9 for opposed faces; S5, S10 indifferent. Mixed, separable by decision.
- **`singular_turn`**: both (`OnMark` acted on). Caveat: it acts on `OnMark` only where the loop's other joints already wind the face's whole turn, a structural check; whether that check alone protects the Zero was not established. `site_rows`' bound from above is poison (decision.rs:312), so a read from above escalates there.
- **`half_turn_off`**: below; already reads `lever.below()`.
- **`stands_on`**: below (only Negative acts; Zero and Positive refuse).
- **`sheets_part`**: both (Negative splits; Zero and Positive take `Unseen`, which goes on).
- **`walls_inside`**: both for `bool_rider_wall_side` and for the flat-floor bend rung; below for the curved-floor bend rung and for `bool_rider_wall_coplanar`. One `arm` per wall feeds all three.
- **`describe_edges`**: both on the seam class (Transverse certified from below afterwards), above on MC's S4, both on S9.
- **`describe_section_boundary`**: both on the seam class; above on the fold's S4; below on S6; both on S9.
- **`joinable`**: both on S4 and S9, because "not joinable" is an answer tier 3 shares.
- **`splitting::rules::edge_wedge`**: both on S4 and `enters_material`; the arm gate below.
- **`topo::edge_wedge`**: as WV. **`edge_wedge_read`**: both on S4, S9; S5, S10 indifferent.
- **step-import `adopt.rs`**: below suffices: it refuses or hands the edge to CERT.
- **sweep `surgery.rs`**: as `must_carry_description`: above in the door, below in CERT.
- **CERT**: below throughout; confirmed.

## Corrections to work/oneround/a-turns-lever-is-read-from-below-for-every-caller.md

1. Its opening describes a door that reads only from below. The doors now return
   `LeverBounds::Between{below, above}` (wedge_reach.rs:144-170, commit 5aa4c79b;
   decision.rs:297), with the faces' covers and the edge's own cover above. Every
   production caller still takes `.below()`.
2. `stands_on`: listed as a Zero site. Its `false` (Zero or Positive) is not acted on as
   an answer: `closes_at_its_site` returns false, `one_site_segments` skips the record
   (join.rs:997), and the leftover record is refused `SingleSiteSectionLoop`
   (join.rs:1402-1406, 1918). Only Negative acts: definite sign, from below, which is
   what it reads.
3. `sheets_part`: listed as Zero. Its Negative is a definite claim acted on (a split at
   `p`), and its Zero/Positive take `Interior::Unseen`, which is not a refusal
   (reduce.rs:3832, 5524-5539). So it owes both, not above alone.
4. `route_rim`: listed as Zero. S4's Zero is acted on (above), but the same fold's
   pairing S6 and, for opposed faces, the bend S9 act only on definite signs (below);
   S5 and S10 are indifferent. Mixed. Also its "Transverse refuses" is a
   `ClassificationInvariant` refusal, and `Unrouted` is read by the glue door as "no
   tangency", not as a refusal of the operation (mod.rs:8182-8183).
5. `must_carry_description` (`chord_spec`, `split_edge_onto`): listed "Zero, unsure".
   S4: Zero acted, Positive refused: above. S9: Positive stores `TangentIntersection`,
   Zero stores the conventional description: every sign. The Positive path is
   re-certified from below by CERT; the Zero path is not. So the door owes above, and
   with CERT the pair is "both".
6. The MC re-asks (`reask_pair`, `reask_chart`) are every sign, as listed. Add: the
   Positive paths (`Intersection`, `TangentIntersection`) pass CERT from below; the
   conventional path is the only one with no second reading.
7. `edge_join::joinable`: listed "every sign, then definite". S9 there is every sign too:
   Positive joins, Zero/Negative return `Ok(None)` (edge_join.rs:745-750), and check 11
   reads the same predicate at rest, so it is not a refusal. Both.
8. `edge_wedge_read`: listed "every sign (its finish caller serves Smooth only, a Zero
   use)". Within it, S5 and S10 are indifferent (`WedgeClass` drops the sign and the
   end), and S6 does not change the stored description. In `describe_section_boundary`
   the fold's S4 Positive is refused (`SmoothJoinRefuted`), so S4 owes above there; S9
   still owes both.
9. `splitting::rules::edge_wedge`: confirmed every sign; it also makes the
   `enters_material_arm` gate and an `enters_material` whose `Tangent` (Zero) is acted
   on as smooth (`wall_graze`).
10. `walls_inside`: "every sign" holds for `bool_rider_wall_side`; its two rungs differ:
    `bool_rider_wall_bend_across` is below on a curved floor and both on a flat one;
    `bool_rider_wall_coplanar` is below.
11. `describe_edges`: "seam class, every sign" holds; add that its Smooth branch runs MC
    with Transverse refused (`must_carry_reading`, ops.rs:3369-3383), so S4 there owes
    above, and that `classify_seam` itself already brackets a spline face's arm between
    a lo and a hi fold of the reach.
12. JT: the issue's "`pcurve_singular_turn` serves on its mark (a slit's whole turn)"
    holds; the mark is acted on only where `rest.deck == wraps` and the edges are not
    on twin sheets (decision.rs:1192-1201). `pcurve_singular_sheet` and the turn already
    name `lever.below()` explicitly (decision.rs:1168-1171, 1186). `site_rows`' bound
    from above is poison.
13. The two callers outside topo, left unclassified: step-import `adopt.rs` needs only
    below (every non-refusal path is certified); sweep `surgery.rs` is a
    `must_carry_description` caller (above in the door, below in CERT).
14. CERT: "definite sign throughout" is confirmed, with `named_by_parallelism`,
    `GRADIENT_ALIGNMENT_LEVER` and the `*_wedge` readings in naming roles only, and
    `tangent_arm_min` read only by the tube check, not stored.

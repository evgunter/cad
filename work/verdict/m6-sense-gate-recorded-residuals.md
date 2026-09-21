---
id: m6-sense-gate-recorded-residuals
kind: issue
title: M6-6 sense-gate residuals — the recorded classes outside the gate, with their flip conditions
status: open
opened: 2026-08-07
github: 226
refs: [223, 250]
priority: P1
cost: D
---

## From GitHub issue 226

Opened 2026-08-07; 0 comments.

Tracking issue for #223's recorded residuals (Ev's ask — recorded AND scheduled). Each is pinned in-tree (the pin's doc names its flip condition; the pin fails the day the condition lands, forcing the honest update):

1. **Conic-trimmed cylinder walls slip both gates** (residual 4, the review's executed counterexample): cut_cylinder's ellipse-trimmed wall — single-wall flip green, whole-body inversion green with positive volume, export-flip-reimport green. Pin: `cut_cylinder_conic_trim_residual_stays_green`. **Flip condition: the ellipse-rim material-side encoding** (extend boundary_material_sign's rim vocabulary past circles). Sequencing: a rider on whichever unit next touches the rim-classification family, or its own S unit if the M6 exit walk wants it sooner.
2. **Rimless-band half-flip invisible** (V=0 Zero-exempt; the ball's single-encoding limit). Pinned as residual at #223. Flip condition: a shell-level orientation check or a second encoding channel — genuinely open design, sized with the next props unit.
3. **NURBS faces bit-free** (winding-derived quadrature — outside the four-kind scope by the ratified unit shape). Flip condition: a NURBS material-side encoding, naturally riding the NURBS-vocabulary growth (stage-1 recognition era).
4. **Arc-bounded planar caps check-6-exempt** (the pre-existing deferral; whole-body cases now caught via circle-rimmed curved walls — but a body with ONLY arc-bounded planar faces + conic-trimmed walls remains uncovered, per residual 1's counterexample). Flip condition: check 6's arc-bounded planar arm. **NEW FACT (VERBS-1031B, 2026-09-03): the class now has a PRODUCER, and that turns this residual from a coverage hole into an assigner/checker divergence.** `merge_faces::loop_winding` learned the arc-bounded winding arm, so `merge_coplanar_faces` now MINTS merged planar faces whose outer loop and rings ride circles (four per teapot cup) and ASSIGNS their outer/ring roles from the `bool_ring_run_winding` predicate — while check 6, the same predicate's third site, still skips exactly those loops. Roles are therefore assigned by a functional the validator cannot check. Measured rather than argued: under VERBS-1031B's MUT-2 (the bulge correction applied backwards) the cup's merge SUCCEEDS with every annulus inside out and `validate_geometric` stays `Ok(())`. The flip condition is unchanged and is now owned, with its refusal-surface cost, by `work/curved/verbs-1031b-assigner-checker-divergence.md` (adopted by CURVED 2026-09-04; its plan carries the lane).

These are walk material for M6's exit (carried items with named owners per the walk discipline); none blocks the walk itself.

## Re-read (PROPS orchestrator, 2026-09-05 — a read-only census at `fa31d6187`)

This item is a TRACKER, not a unit. What the census found, per residual:

1. **Conic-trimmed walls**: the pin was re-cut as
   `cut_cylinder_conic_trim_wall_flip_is_caught_and_the_inversion_is_the_residue`
   (`crates/step-export/tests/m6_6_sense_gate.rs:365`): each single-wall
   flip now refuses with `LaminaWedge` (the material-wedge arm, one
   dimension down); what remains is the WHOLE-BODY inversion staying green
   with positive volume. The flip condition is unchanged — an ellipse-rim
   material-side encoding — and the obstacle is the premise, not the
   vocabulary: a tilted section is not an iso-`v` rim, so
   `props_rim_level` itself has to be restated for it
   (`geom-brep/src/props/curved.rs:1173-1201`). PROPS ground after the
   inheritance; a D→H unit of its own when the sphere/quad lanes are done.
2. **Rimless-band half-flip**: `ball_half_flip_is_caught_at_the_shared_edge`
   (`m6_6_sense_gate.rs:250`) — the half-flip refuses `LaminaWedge` at the
   bands' shared edges; the face-level `Unencoded` exemption
   (`curved.rs:196-197`, `validate.rs:4004`) is untouched by design, and
   the fully inverted ball is caught by `NegativeVolume` only. The
   "shell-level orientation check" is partly what the edge-local wedge
   arm now is; the pair-level question is a design conversation, not a
   patch.
3. **NURBS faces bit-free**: the change detector
   `loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below`
   (`crates/sweep/tests/m5_s11_concave_sense.rs:644`) fires the day the
   `spline_chart().is_some()` skip (`validate.rs:3985`) is removed. Rides
   the NURBS vocabulary; not schedulable here.
4. **Arc-bounded planar caps**: CURVED's lane (`work/curved/plan.md`),
   on TOPO's territory (`validate.rs` and `merge_faces.rs` are
   `work/topo/program.md`'s literal paths; no open PR touches either
   today). Not PROPS'.

Doc drift fixed here: the divergence item's path (it lives under
`work/curved/`); `validate.rs:3826` cites a `work/verbs/` path that does
not exist — TOPO's line to fix. Residual 1's pin name above is the old
one.

## Residual 2, extended: what the MEASURING door answers (SENSE-DOORS, 2026-09-15)

Both of SENSE-DOORS' reviewers exercised the one-band-reversed ball
through the public API and found the same two things, which the residual
above records only half of. On a revolved unit ball with ONE band's
`Face::sense` flipped (`Body::set_face_sense`):

1. **`topo::props::mass_properties` answers volume exactly `0.0`, with
   no error.** The two hemispheres' radial terms cancel term for term,
   so the door a user reaches for first returns a plausible-looking
   number for a body that encloses nothing. The residual above is
   written about what the GATE sees; this is what the MEASUREMENT says,
   and a caller who never runs tier 3 gets it silently.
2. **Tier 3 refuses, and the refusal names the wrong thing.**
   `validate_geometric` raises exactly one `LaminaWedge { edge }` — the
   seam meridians reading as conformal contact — on a body whose defect
   is an inverted face, and it reports one edge although both seam
   meridians are lamina. There is no `CurvedSenseInverted`: the lamina
   reading wins first. That is MATE3's known `LaminaWedge`-message
   problem on the exact shape this residual is about.

Pinned as of SENSE-DOORS (PR 2649):
`crates/mesh/tests/r2_sense_e2e.rs::a_reversed_band_measures_zero_and_is_caught_only_by_tier_three`
asserts all three of the ball's volumes and both refusals, so the day
either half changes the row says so. Flip condition for the residual is
unchanged (a shell-level orientation check or a second encoding
channel); what is added here is that the measuring door's silence is
part of what that design conversation has to answer.

## Residual 2's misnomer, a second instance: a rod with one wall reversed (SENSE-FOLD, 2026-09-15)

A rod (two half-arc profile segments extruded: two half-cylinder walls
on one chart, two planar caps) with ONE wall's `Face::sense` flipped
raises three errors for the one defect through `validate_geometric`:
check 4 names the wall (`CurvedSenseInverted { face }`, the bit read
directly), and the material-wedge arm raises `LaminaWedge { edge }`
on each of the two SEAMS the reversed wall shares with its honest
twin — the same cylinder, osculating jets, opposed material sides,
read as conformal contact. Unlike the ball's case the bit IS named
here, so the misnomer is a duplicate rather than a substitute; but a
user still reads two edges as broken for a face that is. Pinned as
`crates/sweep/tests/r2_sense_fold_probes.rs::a_reversed_rod_wall_is_named_by_check_4_and_read_as_lamina_at_both_seams`
(exact error set: the wall once, the two seams, nothing else), so
the day the wedge arm stops reading a reversed wall as lamina the row
says so.

## Home

`work/issues/`: the four residuals span `validate` check 6, the props sense gate and the NURBS vocabulary, and no open program's charter claims the set — VERBS cites only residual 1, as VERBS-CONE's known trap.

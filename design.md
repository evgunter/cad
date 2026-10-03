# NURBS and spiric operand edges in the boolean — design report (revised after reconciliation round 1)

## For Ev

**Recommendation (likely).** Three decisions, in build order:

1. **Delete `gate_operand_edges`, once the three sites it silently protects are typed** (the planar lane's "no row means a line" arm, the join's on-edge germ frame, the section ring lane). A refusal then names the pair that lacks a rung, which is how faces already work (C12.1, "retire per arm, never wholesale"), and no second roster of admitted carriers sits beside the lanes. This retires no shape today and is still right: a refusal that outlives its reason is a second source of truth.
2. **Build the NURBS edge rung with frontier (d), not now and not inside the NURBS-face row.** Its first producer is (d)'s join window, which will mint fitted NURBS seams on cylinder and sphere faces; from that day DESIGN's "every boolean output is a legal boolean operand" (the maximal-faces paragraph) is owed, and an output the boolean cannot re-consume breaks it. Before (d) lands the rung has no caller, which is the dead-code pattern frontier (f) names. The rung is the ring composite below: one lane keyed on the carrier, no root lane per face kind.
3. **Build nothing for the spiric.** No public door produces a spiric-bearing operand that reaches the boolean (the shelled partial revolve stops at the props door, `VolumeUncomputable`; teapot wall 1 measures it). Keep the typed refusal; the kind re-opens with its producer, through the section reduction below.

**Terms.** A *carrier* is an edge's underlying infinite curve; a *spiric* is a torus cut by a plane parallel to its axis, which the shell of a partial revolve mints as a cap rim. The *operand gate* refuses before any work; the *crossing layer* is the sweep that finds where an operand edge meets the other operand's faces; a *rung* is one certified case of it; the *extent test* certifies that an untouched face lies on one side of the other operand; a *germ chord* is the join's section of two face carriers between crossing vertices (frontier (d)); the *ring composite* is the polynomial `f ∘ C` of a surface's implicit residual `f` along a NURBS curve `C`, in Bernstein form.

### The premise, corrected (sure)

1. **The gate's own reason names the wrong producer.** Its doc says rung-3 edges "are what the curved zip mints, not what it consumes". Nothing in the boolean mints a 3-D NURBS carrier today; (d)'s rung 3 is a 2-D chart image, unbuilt. The producers are the loft and path sweep, whose seams are the boundary iso-curves of their NURBS walls, the shelled partial revolve's spiric rim (`offset_axial::mint_carrier`), and STEP import, which keeps a spline carrier that certifies as no analytic kind (`recognize_curve`: "stays NURBS silently").

2. **For every producer that exists today, the edge is the last of four doors, and lifting it changes no outcome.** Measured by lifting the gate locally (not committed), ε default:

   | shape | refuses next at |
   |---|---|
   | loft prism, box through a wall | `CurvedBooleanUnsupported { kind: Nurbs }` — the crossing layer has no side test on a NURBS surface (`implicit_residual` is poison there) |
   | loft prism, box far away, disjoint | `NurbsExtentUnsupported` — frontier (e) |
   | loft prism, peg pressed into its planar cap only | `Containment(KindUnsupported { Nurbs })` — point-in-solid has no ray row through a NURBS face |
   | loft prism, flush plate under its cap, declared | `CurvedBooleanUnsupported { kind: Nurbs }` |
   | teapot wall 3 (spout ∪ pot), lily wall 8 (sheath ∪ blade) | `CurvedBooleanUnsupported { kind: Nurbs }` |
   | klein wall 3 (loop ∪ bulb) | `CurvedPairUnsupported { Cone, Plane }` at the gate |
   | every STEP fixture in the corpus (60 files) | no imported body carries a NURBS edge at all; the recognizer promotes what certifies, and the files with true splines refuse at adoption |

   Behind those three doors stands a fourth, the join's section frame, which has no arm for any NURBS face pair. The three tour walls are therefore the NURBS-face row's (crossing side test, extent, point-in-solid, section frame), and klein's is the cone's. The edge-only lane's consumers are bodies whose faces are all analytic and whose edges are not: (d)'s fitted seams, and STEP-adopted splines on analytic walls, of which the corpus has none.

3. **The spiric kind is unreachable at the gate** (above). Its "drilled away from the rim" consumer cannot be built until the props lane integrates a spiric-bounded cap.

4. **"Re-entry through the germ-chord lanes" locates nothing.** (d) is the join's section *window*; it mints an edge, it does not find where an existing edge pierces a face. For the spiric, a reduction to the sections table is the lane when a producer exists: a spiric edge on `O = T ∩ Π` meets a carrier `S` exactly where the planar section `Π ∩ S` (a C5 line, circle or ellipse) meets the torus, so `line_torus_roots` and `circle_torus_roots` answer plane and sphere faces with no new root lane, the minor angle of each root is the edge parameter exactly, and cylinder and torus faces refuse typed where `ellipse × torus` and `spiric × torus` do.

5. **The gate is load-bearing for soundness, not merely conservative (sure).** In the sweep's planar lane, `conic_plane_crossing_roots` answers "no row" for a line, a spiric and a NURBS carrier alike, and the lane reads that as *"a line: the M3 lane below owns it"*: the crossing parameter is interpolated from the endpoint distances and same-side endpoints mean no crossing. For a curved carrier both are wrong and silent. The join's on-edge germ frame answers `JoinDesync` for the same carriers, and the ring lane `SectionInvariant`, both citing the gate. Decision 1's precondition is that all three refuse typed on their own.

### The rung, when (d) brings its first consumer (likely)

The substrate exists. `geom_core::spline::compose` (M5 PR 4) builds, for a NURBS curve against a plane, sphere, cylinder, cone or torus, the rational Bernstein form of `f ∘ C` in certification arithmetic with certified coefficient hulls per span; today it serves only the on-locus certificate of fitted section curves (`ssi::certify`). As a crossing rung:

- **Clearance**: the numerator hull one-signed over the span certifies a clear pair (the denominator is positive by the weight invariant). No sampling, no chord-dip charge, no per-carrier speed or curvature bound: the hull *is* the enclosure, as `FaceBoxRule::ControlNet` is a box.
- **Roots**: subdivide the same coefficients by knot insertion in certification arithmetic (`insert_knot_plan`, `apply_certified`) until a piece is sign-definite (clear), or its ends differ in sign and its derivative hull (`derivative_coeffs`) is sign-definite (one root, bisected on the residual to the scalar's resolution and confirmed ON the surface), or its arc length is inside the band (a tangency, `Uncertain`). That is the clear / monotone / split ladder `circle_roots::certified_subdivision` runs on the conic's trigonometric polynomial, with hulls in place of Taylor bounds.
- **Coincidence is read honestly.** A seam lying on the other operand's carrier has every coefficient within rounding of zero, so the hull straddles zero and the ladder ends `Uncertain` at the typed door. A sampled enclosure reads the same residual as definitely negative by its own charge, which is why the conic rung needed a carrier-identity rung in front of it.
- **One lane, keyed on the carrier.** Per face kind only the composite differs, and all five are written, the cone included. The plane case is the linear composite and repairs the planar arm in (5). The conic rung becomes the special case where the composite is a trigonometric polynomial. The NURBS edge box becomes the control hull (`nurbs_curve_aabb`), the obligation `EdgeBoxRule::NoSoundBox`'s docs defer to "the day the gate admits the kind".

**The sampled alternative** (a hull of residual samples widened by a chord-dip charge, with bounds `|C′|, |C″|, |C‴|` per carrier and `|F″|` per face kind) is the conic rung generalized. For a NURBS carrier it is weaker on every axis: it needs derivative bounds the tree has only for non-rational carriers and only to second order (`nonrational_second_derivative_sup`; no `|C‴|` door, no rational door), it has no cone arm (`conic_arc_residual_range` returns `None` there), and it misreads coincidence. It is the right shape for the **spiric**, whose carrier is not rational and has no composite, with `spiric_rate_bounds` and its trigonometric form supplying the bounds. The two methods are not rivals for one carrier: the hull for NURBS, the sampled enclosure or the section reduction for the spiric, and neither comes first because the spiric has no producer.

### Alternatives weighed

- **Pair-scoped gate** (refuse a spiric or NURBS edge whose box may meet any face of the other operand). Cheap and symmetric with the split's PR 3843. Against: it re-states at a second site the box test the sweep's BVH runs per pair once the edge has a sound box, retires nothing in the table, and is unsound until (5) is fixed. The split could narrow safely because its other operand is one plane and nothing runs behind its gate. Not recommended.
- **Edge rung now, ahead of (d).** Its substrate is independent and unit tests can run on hand-built carriers. Against: no caller until (d) lands; building it first is machinery with no consumer. Recommended only as (d)'s consumer-side half, scheduled with it.
- **Edge rung inside the NURBS-face row** (my earlier position). Against: (d) comes first and already owes it by the legal-operand clause; the face row is larger and later.

### Ratified text

Nothing to change. The body-scoped edge gate is stated only in code docs (M5 PR 9); DESIGN.md and the topo README do not mention it, and C12.1's per-arm clause and the maximal-faces paragraph's legal-operand clause already decide the shape and the schedule. Frontier (e) names the extent half of the face row. The work row's "neither has a root lane" and "a definite crossing would still refuse" overlook `spline::compose`.

**Confidence.** Decisions 1 and 3: likely; decision 2's schedule: likely, resting on (d) being a P1 open row (`work/join/cylinder-sphere-germ-pair-has-no-section-frame.md`). The measured table, the corpus census and the planar-lane defect: sure. The spiric being unreachable: sure. The composite serving as root lane with the ladder written as described: likely (the hull and knot-insertion doors exist; the ladder is new code).

## For the orchestrator

**What changed in round 1, and why.** Two things moved. (a) The gate: I had retired it as a line item of the NURBS-face row; it now goes as soon as its three protected sites are typed, on the other report's final-state argument that a refusal must earn its place and a gate beside the lanes is a second roster. (b) The schedule: I had tied the rung to the face row; the other report's (d) consumer, which my table missed, plus the legal-operand clause (confirmed at DESIGN's maximal-faces paragraph) move it to (d). I did not move to "now": with the spiric unreachable and the corpus free of spline edges, there is no caller before (d). On the method I did not move; the comparison is now spelled out, and the sampled enclosure is credited as the spiric's shape. On the spiric I did not move: their "drilled away from its rim" payoff is argued, and the props door makes it unbuildable, measured.

- **Measurement.** `gate_operand_edges` made conditional on an environment variable locally; a unit test in `crates/sweep` built the four prism cases on `test_support::loft_prism`; the tour's `lily::wall_probes`, `klein::wall_probes` and the teapot's in-crate probe test ran with the gate lifted; a scratch `step-import` example imported every `.step`/`.stp` under `tests/fixtures` and counted NURBS edges and the kinds of the faces they bound. All edits were reverted; nothing but this file is on the branch.
- **Defects found off the question**, not filed here because this branch carries only the report:
  1. The planar lane reads a spiric or NURBS carrier as a line (`boolean/reduce.rs` sweep, the `Err(()) => {}` arm after `conic_plane_crossing_roots`). CLEAVE, P2: latent while the gate holds.
  2. `join.rs` on-edge germ frame (`JoinDesync`, "the operand gates refuse") and the ring lane (`SectionInvariant`) cite the gate as their invariant; both become reachable the day it goes.
  3. `CurvedEdgeUnsupported`'s doc and Display name the zip as producer and say "spline" though the variant fires for spirics; `gate_operand_edges`'s doc the same.
  4. `work/reach/boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule.md` should split: the NURBS-face operand row (side test, extent (e), point-in-solid, section frame; the three demo walls belong there) and the edge rung as (d)'s consumer-side item; its "no root lane" paragraph should cite `geom_core::spline::compose`.
- **Rows that flip when the gate goes**: `review_cleave_nurbs_lane` (disjoint union with the M7-8 cube), `s16_box_soundness`, `offc_r1_probes`, `spiric_rim`, and the three scene walls; each re-pins on the face door it then reaches.
- **Could not check.** The checkout is shallow, so the gate text's provenance rests on the "M5 PR 9" citation; the other report found it on REACH/CURVED commits of 2026-09-30/10-01, neither an Ev ruling. Whether (d)'s fitted 3-D carrier will be rational: `curves::fit` interpolates with unit weights, so likely not, but the composite takes either.

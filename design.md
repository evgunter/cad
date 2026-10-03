# NURBS and spiric operand edges in the boolean — design report

## For Ev

**Recommendation (likely).** Do not build an edge lane as a unit of its own, and do not build a pair-scoped edge gate. Retire `gate_operand_edges` as one line item of the row that makes a **NURBS face** a boolean operand, and when that row reaches the edge, give it one rung keyed on the carrier (the ring composite below) rather than a root lane per face kind. For the **spiric**, build nothing now: no public door produces a spiric-bearing operand that reaches the boolean, so the typed refusal stays and the kind re-opens with its first producer, which is frontier (f)'s posture.

**Terms.** A *carrier* is an edge's underlying infinite curve (line, circle, ellipse, spiric, NURBS); a *spiric* is a torus cut by a plane parallel to its axis, which the shell of a partial revolve mints as a cap rim. The *operand gate* refuses before any work; the *crossing layer* is the sweep that finds where an operand edge meets the other operand's faces; a *rung* is one certified case of it; the *extent test* certifies that an untouched face lies wholly on one side of the other operand; the *germ-chord lane* is the join's section of two face carriers between crossing vertices (frontier (d)).

### The premise, corrected (sure)

1. **The gate's own reason is wrong about who produces these edges.** Its doc says rung-3 edges "are what the curved zip mints, not what it consumes". Nothing in the boolean mints a 3-D NURBS carrier today: the zip's rung 3 is a 2-D chart image (`Pcurve::Fitted`, frontier (d), unbuilt). The producers are the loft and path sweep, whose seams are the boundary iso-curves of their NURBS walls (`sweep::loft` phase 6), and the shelled partial revolve, whose cap rim is a spiric (`offset_axial::mint_carrier`). So the claim is about the zip, not about the operand.

2. **The edge is the last of four doors, and lifting it alone changes no outcome.** Every producer puts a NURBS edge on a NURBS face of the same body, and that face refuses later whichever way the other operand meets it. Measured by lifting the gate locally (not committed), ε default:

   | shape | refuses next at |
   |---|---|
   | loft prism, box through a wall | `CurvedBooleanUnsupported { kind: Nurbs }` — the crossing layer has no side test on a NURBS surface (`implicit_residual` is poison there) |
   | loft prism, box far away, disjoint | `NurbsExtentUnsupported` — frontier (e) |
   | loft prism, peg pressed into its planar cap only | `Containment(KindUnsupported { Nurbs })` — point-in-solid has no ray row through a NURBS face |
   | loft prism, flush plate under its cap, declared | `CurvedBooleanUnsupported { kind: Nurbs }` |
   | teapot wall 3 (spout ∪ pot), lily wall 8 (sheath ∪ blade) | `CurvedBooleanUnsupported { kind: Nurbs }` |
   | klein wall 3 (loop ∪ bulb) | `CurvedPairUnsupported { Cone, Plane }` at the gate |

   The three tour walls are therefore not this row's: they are the NURBS-face row's, and klein's is the cone's. A row scoped to the edge has no consumer it can retire.

3. **The spiric kind is unreachable at the gate.** The shelled partial revolve, its only producer, stops at tier 3's props door (`VolumeUncomputable`: a spiric-bounded cap's area is an elliptic integral; teapot wall 1 measures it). Two source comments say the same (`census::edge_reach`, `boxes::EdgeBoxRule::Spiric`). Building a spiric crossing lane now is reviewed machinery with no caller.

4. **"Re-entry through the germ-chord lanes" is not an option for a NURBS edge.** That lane sections two face carriers, and a seam's parents are NURBS walls, for which only the plane×NURBS section route exists. For the spiric it *would* be the natural lane: a spiric edge on oval `O = T ∩ Π` meets a face carrier `S` exactly where the planar section `Π ∩ S` (a C5 line, circle or ellipse) meets the torus, so the existing `line_torus_roots` and `circle_torus_roots` answer plane and sphere faces with no new root lane, the minor angle of each root is the edge parameter exactly, and the cylinder and torus faces refuse typed where `ellipse × torus` and `spiric × torus` do. Kept here for the day a producer exists.

5. **A latent defect the gate hides (sure).** In the sweep's planar lane, the conic root door (`splitting::conic_plane_crossing_roots`) returns the same "no row" answer for a line, a spiric and a NURBS carrier, and the lane's arm reads that answer as *"a line: the M3 lane below owns it"*: the crossing parameter is interpolated from the endpoint distances and same-side endpoints mean no crossing. For a curved carrier both are wrong, and silent. The body-scoped gate is the only thing keeping that arm sound, which is the unstated nesting invariant `curved_face_arm`'s own comment warns against. Whatever is built, this arm becomes typed first.

### The edge rung, when its turn comes (likely)

The substrate exists. `geom_core::spline::compose` (M5 PR 4) builds, for a NURBS curve against a plane, sphere, cylinder, cone or torus, the rational Bernstein form of the composite `f ∘ C` in certification arithmetic, with certified coefficient hulls per span. Today it serves only the on-locus certificate of fitted section curves (`ssi::certify`). It is also the crossing rung:

- **Clearance**: the composite's numerator hull one-signed over the span certifies that the edge clears the face (the denominator is positive by the weight invariant). No sampled chord-dip enclosure and no per-carrier `|C′|, |C″|` bounds are needed: the hull *is* the enclosure, exactly as `FaceBoxRule::ControlNet` is a box.
- **Roots**: subdivide the same coefficients by knot insertion in certification arithmetic (`insert_knot_plan`, `apply_certified`) until each piece is sign-definite (clear), or its ends differ in sign and its derivative hull (`derivative_coeffs`) is sign-definite (one root; bisect on the residual to the scalar's resolution and confirm it reads ON the surface), or its arc length is inside the band (a tangency, `Uncertain`). That is the clear / monotone / split ladder `circle_roots::certified_subdivision` already runs on the conic's trigonometric polynomial, with hulls in place of Taylor bounds.
- **One lane, keyed on the carrier.** Per face kind only the composite differs, and all five are written. A plane face is the degree-`p` linear composite, which repairs the planar-lane arm in (5) at the same time. A NURBS face has no composite, which is the face row's problem, not this rung's. The conic rung becomes the special case where the composite is a trigonometric polynomial; nothing is represented twice.
- **The edge box** becomes the control hull (`nurbs_curve_aabb`), the obligation `EdgeBoxRule::NoSoundBox`'s docs say is owed "once a rung-3 operand gate admits the kind". Sound by the convex-hull property; today the NURBS edge box is poison, which overlaps everything.

With the rung in place the gate has nothing left to say about a NURBS edge: the refusal moves to the pair that lacks a rung, which is C12.1's shape ("retire per arm, never wholesale") and already how faces work. A *pair-scoped gate* that admits the edge "wherever it provably meets nothing" would re-state, at a second site, the box test the sweep's BVH already runs per pair once the edge has a sound box; it retires no shape in the table above; and it is unsound until (5) is fixed. The split's narrowing (PR 3843) was right for the split because its other operand is one plane and nothing runs behind the gate; neither holds here. Reversible: every piece is a rung or a box rule.

### Alternatives weighed

- **Pair-scoped gate now.** Cheap and symmetric with the split. Against: measured to retire nothing, duplicates the BVH's pruning, and leans on the defect in (5). Not recommended.
- **Edge rung as a standalone unit, now.** Its substrate is independent and its unit tests can run on hand-built bodies, so it could be built early. Against: it has no end-to-end consumer until the face row's three doors land, and ordering it first invites the dead-code pattern. Recommended only as a line item inside the face row.
- **A root lane per face kind, derived from `|C′|, |C″|` bounds** (the work row's sketch). It would work, but it re-derives per carrier what the composite already carries exactly, and leaves the spiric with "a clear pair would clear" and no roots. Not recommended.

### Ratified text

Nothing to change. The body-scoped edge gate is stated only in code docs (M5 PR 9); DESIGN.md and the topo README do not mention it, and C12.1's per-arm clause already decides the shape. Frontier (e) names the extent half of the face row. The work row's statement that "neither has a root lane" and that "a definite crossing would still refuse" overlooks `spline::compose`; it should cite it.

**Confidence.** Recommendation: likely. The measured table and the planar-lane defect: sure. The spiric being unreachable: sure. The composite serving as root lane with the subdivision written as described: likely (the hull and knot-insertion doors exist; the ladder itself is new code).

## For the orchestrator

- **Measurement method.** `gate_operand_edges`'s refusal was made conditional on an environment variable locally, a unit test in `crates/sweep` built the four prism cases on `test_support::loft_prism`, and the tour's `lily::wall_probes`, `klein::wall_probes` and the teapot's in-crate probe test were run with the gate lifted. All edits were reverted; nothing but this file is on the branch.
- **Defects found off the question**, not filed here because this branch carries only the report; each belongs in the program named:
  1. The planar lane reads a spiric or NURBS carrier as a line (`boolean/reduce.rs` sweep, the `Err(()) => {}` arm after `conic_plane_crossing_roots`), masked by the gate. CLEAVE (reduce.rs is in its paths), P2: latent today, the first narrowing makes it live.
  2. `BooleanError::CurvedEdgeUnsupported`'s doc and `gate_operand_edges`'s doc name the zip as the producer of rung-3 edges; the producers are the loft's seams and the shell's spiric rim. Doc-only, lands with whatever touches the gate.
  3. `work/reach/boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule.md` should be re-scoped as the NURBS-face operand row (crossing side test, extent (e), point-in-solid, then the edge rung) or closed into one, and its "no root lane" paragraph corrected to cite `geom_core::spline::compose`. The three demo walls it lists should be cited there, not here.
  4. The brief's "the carriers the curved zip, loft and sweep MINT": the zip mints none; `swept.rs` mints lines and circles only; the NURBS seams come from `loft.rs` (the path sweep `sweep_body` skins through it).
- **Could not check.** The checkout is shallow, so `git log -S` on the gate's text finds nothing; its provenance is the "M5 PR 9" citation in the code only. I did not measure a spiric operand because none can be built past tier 3.
- **Assumed.** The composite door accepts a `NurbsCurve3<T>` through `certified_coords` for both scalar lanes (`ssi::certify` does exactly this). I did not build the subdivision ladder; its cost is a lane's unit, not a design question.

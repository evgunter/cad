## For Ev

**Recommendation.** There are two changes, in this order.

1. **Every sweep door builds its carrier data from the construction's own inputs and rigid motions, never from the vertices it placed.** The profile hands each door the unit directions its construction made. A lane can build this now; it needs no ruling. `sure` that it is right; `likely` that it closes the slab and the derived-frame cube, together with the aimed-ray row.
2. **Check 1 stops reading a frame's unit length back from its numbers.** I lean towards the carrier storing the witness (a type that proves the direction is unit) over the alternative of asking through the decision funnel. Change 2 is its own `[ev]` question. `likely`

I changed my mind on point 2. My first report said that check 1's reading of a direction that truly turns over the box is off only by about δ²/2, so bisection cures it. That was wrong except near the coordinate axes.

**Terms.**
- *Box scalar*: `Interval` or `Sym`, a number held as a bracket over the parameter box.
- *Carrier*: a stored surface or curve.
- *Door*: an operation that returns a body.
- *Witness*: `UnitVec3` or `RigidMotion`, types whose only constructors establish that a direction is unit or that a frame is a rotation.
- *Check 1*: tier 3's datum check. Among other things it computes `‖u_ref‖ − 1` from the bracket's ends and never through the decision funnel.

### Premise check
- Both widths are minted, not inherent (`sure`, read). `cap_plane` and the extrude's line walls fit Newell planes to corners that carry the box. The cylinder's `u_ref` and every rim circle's are `rim.normalize()` of two placed points. Struts are `w.normalize()` and line rims are `(q_to − q_from).normalize()`.
- The revolve only partly follows the rule. Its walls do: they take `AxisFrame`'s axis witness and `u3`. Its latitude circles do not: `revolve/surfaces.rs`, `full.rs` and `turn.rs` each mint `rim.normalize()`. Neither do its partial caps or the loft's caps, which go through `cap_plane`. I was wrong to call the revolve and tube already compliant.
- Two widths are tangled together:
  - The slab's is width in a *value*. The cap's normal is (0, 0, 1) at every depth, and the stored bracket is not.
  - The tour's is a *claim read back from numbers*. Check 1 measures the length of a bracketed unit vector.
- Change 1 removes the first. It removes the second only where the direction does not truly vary.

### Change 1 — the doors keep the frame they are handed (buildable now)
- **Caps.**
  - The normal is ± the sketch frame's normal, `frame.turn(±ẑ)`. The sign is the profile's winding, which is already decided.
  - The origin is the frame's origin, plus the sweep for the end cap.
  - `u_ref` is the frame's first axis.
  - `cap_plane_orientation` and the `EdgeOn`/`Orientation` refusals lose their cause and go. `newell_plane` keeps no sweep caller and stays for the importer.
- **Line walls.**
  - The normal is `frame.turn` applied to the segment's in-plane unit normal, a rotation of a unit vector, so it is exact.
  - Line rims run along `frame.turn(direction)`. Struts run along `±frame.turn(ẑ)`.
  - On the `Extrusion::Vector` arm the wall is ruled along `w`, which may be oblique within ε. That arm keeps one decided mint, the levered `UnitVec3` of (rim direction × `w`), until `work/strut/extrude-arc-walls-are-ruled-in-n-not-w.md` is settled. No box study uses that arm.
- **Arc walls and rim circles.** `u_ref` is `frame.turn(the arc's unit start direction)`.
- **Revolve latitude circles.** The frame's seam direction `u3`. This is inferred: a vertex sits on the seam meridian.
- **Partial-revolve and loft caps.** That station's frame.
- **The profile carries the directions.** `ValidatedSegment` gains a line's unit direction and an arc's unit start direction. These come from the path layer's `Dir`, the circle verb's +x (or its phase), and the unit that `Arc2::from_chord` already computes and then drops.
  - Without them the tour does not close. The hole's start is `(cx + r, cy)`, so a 2-D re-derivation `(start − centre)/‖·‖` still divides a bracket by its own norm (`([0.9, 1.1], 0)` becomes `([0.82, 1.22], 0)`), and check 1 still refuses. This is inferred.
  - The redundancy is of the kind the profile already has: an arc's fields repeat its endpoints, and those are certified against it (`arc_reads_back`, `carrier_endpoint_*`).
- **What it makes true.** Every stored direction in the slab, the cube and the tour's holes is exact, at `f64` as well as over a box. The only width left is the geometry's own.
- **Still possible.** A new door can mint a direction from points through a decided normalize. A reviewer sees that, but the type does not stop it.
- **Reversible.** Yes; the change stays inside `sweep` and `profile`.

### Change 2 — check 1 stops asking a frame for its length (the fork)
**Worked example**, checked by hand. Take a 5 mm hole on a body turned by θ = 30° ± 1°.
- The stored `u_ref` is `(cos θ, sin θ, 0)`, held as two independent brackets: [0.857, 0.875] and [0.485, 0.515].
- The norm of that bracket reads [0.985, 1.015]. The deviation is first order, about δ·|sin 2θ|, not δ²/2.
- Check 1's margin is therefore ±7.5e-5 m against ε = 1e-9. It decides only once a leaf is about 1e-5° wide, roughly 17 halvings along that one axis.
- Every member of the family is exactly unit.
- This is a real case. Fork 259's case (a) measured it on a boxed motion and left it reported as undecided.

**Option 2: a carrier stores its frame as the witness.** Check 1's six frame margins go at every scalar. The importer mints through the decided constructors.
- For:
  - A non-unit frame cannot be represented.
  - It resolves a conflict in spirit. Since PR 5068, `RigidMotion`'s own docs say "at a box scalar no door re-derives a motion's rigidity from its entries", and check 1 does exactly that to the same numbers once they sit in a carrier. The later text is yours; check 1's margins came from PR 3238, an agent unit.
- Against:
  - Nothing measures a wrong mint at `f64` any more.
  - Tier 3 reads an `f64` body at a box scalar by a structural lift. That lift would carry the witness upward, which 5068's "never up" forbids.

**Option 2′: bare fields stay, and check 1 asks through the funnel.**
- For: small and reversible. At `Sym` the rule `sin² + cos² = 1` discharges the example by theorem.
- Against:
  - Plain `Interval` never certifies a frame that is wide over the box.
  - Fork 266 measured identities over chained frames freezing at `Sym`.

I lean to 2. Both options change text: `geom`'s at-rest paragraph, and under 2 also `UnitVec3`'s and `RigidMotion`'s "What is NOT this type" and "Crossing scalars".

**Ratified text.**
- Change 1 needs none. It is your `[scalars]` sentence (PR 6912): "a claim … is built at the lane scalar, from its own inputs".
- It reverses an agent-written module doc in `newell.rs` ("Plane equations are derived caches … the sweeps call this").
- Add one clause to `crates/sweep/README.md`: carrier data comes from the inputs and rigid motions. That clause binds future work, so it is yours to ratify.

**Confidence.**
- Change 1 is right: `sure`.
- It needs the profile directions for the tour: `likely`.
- Change 2 over 2′: `likely`.
- The tour needs change 2 at all once change 1 lands: `unsure`. Its holes do not turn over its box.

## For the orchestrator

**(1) What moved me, and what I now recommend.**
- The worked example. I did the arithmetic: the bracket norm's deviation is first order in δ wherever the direction is off the axes. My δ²/2 claim, and "bisection cures it", were wrong. I now agree a second change is warranted, and that it is a separate `[ev]` fork. I lean to the stored witness, as the other report does. Its `RigidMotion`-docs argument is sound: I read `rigid_motion.rs` "re-derives" at lines 44, 185 and 1239.
- The site list. The revolve's latitude circles and partial caps re-derive too. I withdraw "revolve and tube already follow the rule".
- I drop the tight unit map from the recommendation. It is unneeded where the profile carries directions, and it does not touch the bracket-norm question.

**(2) My arguments the other report touches.**
- *The profile must carry directions.* Not answered: it names the profile as "a candidate, not recommended unmeasured". It still stands. Change 1's arc arm needs a sketch start direction, and the 2-D re-derivation `(cx + r − cx, cy − cy)/‖·‖` is the same quotient its own measurement 2 warns about. The circle verb already knows +x, and `from_chord` computes the unit and drops it.
- *Option B, `Sym` narrowing values by expression.* Answered: we both reject it. I keep it only as a possible backstop for a direction between two authored points that share a parameter.
- *Tier 3 is not at fault.* Partly answered against me: right for the cap's tilt, wrong for check 1's frame-length read.
- *The `Vector` arm.* We agree it needs a decided mint, or the strut row settled.

**(3) Where we still differ, and the question beneath it.**
- **Sequencing and need.** I put change 2 off the critical path; the other report pairs it with this row as what holds the tour's stop.
- **The question:** with change 1 built, profile directions included, does the tour's certified stop still refuse at check 1? Its holes vary in spacing and radius, not in angle. If it certifies, change 2 is a separate `[ev]` fork driven by rotating frames (fork 259 (a), the 30° example). If it still refuses, change 2 belongs to this row.
- **How to measure it, on the probe:** mint `u_ref = frame.turn(+x)` on the hole walls and on the rim circles, then rerun `demo-tour certified`. The stand-in for change 2 (skip the `Length`/`Tilt` margins at the box scalars) is the other report's measurement 2. Run both.

**Scope a lane can build now (change 1):**
- `sweep::extrude`: caps, line walls, arc walls, line rims, rim circles, struts, from `profile.plane().frame()`. The `Distance` arm is exact; the `Vector` arm keeps a levered decided mint for its walls and strut.
- `swept::cap_plane`: rewritten to take a station frame, used by the extrude, `revolve/partial` and the loft.
- The revolve's latitude circles: `surfaces.rs`, `full.rs`, `turn.rs`. That `u3` is the right `u_ref` for each is inferred; check per site.
- `profile`: `ValidatedSegment` carries `Line { dir }` and `Arc { start_dir }`, fed from `Dir`, the circle verbs, and `from_chord`'s unit. The lift carries them verbatim. `crates/profile/README.md` has no clause on segment representation (grep), so no ratification is needed.
- The `register_rigidity` and `register_rim_identity` call sites may lose their reason to exist (inferred). Leave them for a follow-up.
- Expect re-baselined pcurve rows on the caps: the cap's origin moves from Newell's centroid to the frame origin.

**Off-question:**
- `Frame::of_motion` is called only by its own test (`geom/src/frame.rs:273`).
- Blends mint many `u_ref`s and normals by `normalize` of differences. That is unmeasured; it is a P1 inventory.
- I could not check the tour's curve-datum findings beside the face findings.

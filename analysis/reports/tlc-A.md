## For Ev

**Recommendation (likely).** No caller should choose a bound. Each levered decision reads its margin at both bounds through one door, and acts only where the two readings lead to the same action. Where they don't, the door refuses with a "straddle": the `Indeterminate` that the existing straddle door (`k_stats::straddle`) records under that decision's own straddle row. No composite verdict needs a new "bounds disagree" arm. The ~41-caller classification goes away, and so does the class of mistake that four reviewed units made with it.

**Terms.**
- *Lever*: the length a dimensionless reading (a tilt, a cosine) is multiplied by to get a margin.
- *Bound from below / above*: the edge's ends and the faces' vertices; the faces' covers.
- *Straddle*: one bound reads Zero and the other reads a definite sign.
- *Pivot*: the point a lever is measured from.
- *Datum*: the reading the lever multiplies, at the point where it is read.

**Premise check (sure).** The brief asks where each decision's bound is chosen. For a margin of the levered form (x·L, or κL²/2), the size of the margin grows with L. So reading both bounds gives exactly: a definite sign when the bound from below decides it, Zero when the bound from above decides it, and a straddle otherwise. The README's three clauses ("definite → below, Zero → above, every sign → both, act where they agree") are that one rule seen from three kinds of consumer. A caller that "needs below" loses nothing by reading both; only the wording of its refusal changes.

The brief also says the composite verdicts (`MustCarryVerdict`, `PairRefusal`, the wedge fold, `RimRouting`, `SeamUnread`, `PinMiss`, `SingularMiss`) have nowhere to say "the bounds disagree". That is the wrong thing to want. A disagreement belongs to one scalar reading, not to a verdict. Every sub-reading inside a composite is already a decide that returns `Indeterminate`. Each composite already routes that into an in-band arm for that rung (for example `MustCarryVerdict::InBand(at_station(i))`, `PinMiss::Escalated`, `SeamUnread::Lever`). A straddle travels the same path, so it is refused once, in one spelling, where it arises.

Mixing bounds across one composite is sound (sure). Say the first-order Smooth is claimed at the bound from above and the second order's Positive at the bound from below. Each claim holds for every true lever between the two bounds, so their conjunction does too.

**The final state.**
1. **Composite consumers.** No consumer of a turn's lever takes a `Lever<T>`. `must_carry_over_edge`, `must_carry_description`, `class_pass`, `second_order_walk`, `dihedral_wedge`, `tangent_second_order`, `edge_wedge_over_as`, `classify_shared_rim`, `classify_seam`, the joint turn, `stands_on`, `sheets_part` and `walls_inside` take the region (item 3) and pass it, unopened, to each leaf decision.
2. **Where the bound is chosen: nowhere outside one door.** A lever reaches a margin only through the levered-decision door (today `topo::validate::decide_at_lever` / `decide_levered`). It moves to `geom-core` beside `decide`, with one entry per margin form (levered, sagitta, levered unit direction, and the `Period` turn, which already does this through `TurnRows`). `LeverBounds::below()` and `above()` stop being public. A decision cannot pick a bound because it cannot open the pair.
3. **One value per row pair.** A levered predicate is named by its predicate row and straddle row together, as `TurnRows::bounded` already pairs them. A levered decision with no straddle row then fails to compile. Today it is a convention: one straddle row per predicate, written by hand.
4. **Agreement is on the action, not the sign (Ev's call, see below).** The door takes the consumer's own map from sign to action; the default is the identity. `stands_on` and `sheets_part` compare `== Sign::Negative`; that comparison becomes their map. So a reading that is Zero at the bound from below and Positive at the bound from above gives "not opposed" either way and is not refused. A reading that is Zero at one bound and Negative at the other is refused. The decision is classified by the code that acts on it, so it cannot be mis-filed.
   - Worked example (likely): `Settled::tangency` demands the intrinsic tangency only if the bend is Determinate and the pair is in the certificate lane. Outside the lane, Determinate and UnderDetermined store the same thing. So a straddle on the bend there should not refuse. The fix is to read the lane first and ask the bend only where it matters, not to pass an action map down the walk.
5. **No bound from above.** This replaces the poison lever with a typed variant, `LeverBounds::AtLeast(below)`. A definite verdict at the bound from below stands, because the margin only grows with the lever. Zero is refused on the straddle row, with an enclosure that has no upper end. Certification is a definite-sign claim, so it keeps every verdict it has today; only Zero-acting decisions there refuse.
6. **Pivot (item 3): the doors return the region, not a lever.** The doors (`edge_reach`, `stored_reach`, `wedge_reach`, `WedgeReach`) return the turn's region:
   - points known to lie on it, for the bound from below;
   - the covers that hold it, for the bound from above.

   Each reading takes its bounds from its own datum: each interior station for the wedge, the joint vertex for the turn. Today the lever is measured from the span's middle while the datum is read at a station. From an end station the far distance can be up to twice the far distance from the middle, so the middle's lever is neither bound there. This is the same rule as "any point is sound provided the datum and its lever are read at the same one", and as "a record that verdicts read from different points holds its region". It matches the session box's "from outside as its cover, from inside off points". (sure on soundness)
7. **What the bound from above reads.** It reads the covers of everything the region is made of, as the door holds it: each face's cover and each boundary edge's own cover over its span. The vertices fall inside the edge covers, so the patch that folded the points into the bound from above (item 5) goes.
   - The face cover becomes a function of the face as the door holds it (its surface and its loops), not of a `FaceKey` in a body.
   - The site mint (item 4) then has an honest bound from above for the face it is minting.
   - A cover measured inside its own vertices (ratio 0.29) is not a cover of that face. Either the door's face record is not the face it governs, or `face_cover` misses its own boundary. The session box reads face covers too, so this is a defect to fix at its source, not something to fold away. (likely)
8. **Bound from below (item 1).** It may grow by sampled points of each boundary edge's span; `edge_reach_bounds` already samples a carrier this way. That is sound for any point of the region. A too-short bound from below only ever causes a straddle, never an unsound claim, so this is a completeness question.
   - It does not help the two cases measured at ratio 9.5e3: spheres whose faces reach far past a small boundary circle. Only a point inside the face would help there, and that needs a containment decision.
   - Recommend edge samples if measurement shows they cure straddles; no interior witnesses yet. (likely)
9. **The cycle (item 6).** The lift reads its faces' covers through the looser unlifted cover. That stays: looser is sound and costs only straddles.

**What is a faithful elaboration, and what is yours.**
- *Faithful* (agents can land them): points 1–3 and 5–9. They are the lever bullet and the same-point clause read literally.
- *Ratified text.* The lever bullet's "which bound" sentence and the records clause were agent-written. You approved them in PR 7141 with "sounds great! i am less sure about the records thing but it seems fine unless there's some kind of weird information flow". Point 6 rests on the records clause.
  - What now flows where: the doors hand a region to each verdict, and every lever is derived at a verdict's own datum. Nothing stores a lever.
- *Yours, 1: action or sign agreement* (point 4). "Act only where they agree" does not say on what. I lean to action. Sign agreement refuses honest verdicts whose two readings do the same thing.
  - Second, separable: rewrite the three clauses to state the rule they come from. Suggested wording: "a levered decision reads both bounds and acts only where both readings lead to the same action; for a margin that grows with its lever, that is a definite sign from below and Zero from above".
- *Yours, 2: newly refused classes* (your PR 7140 condition). Each of these is a claim that used to be made from below with no sound basis:
  - smooth edges between faces much larger than their boundary (the sphere-sphere circle; one-vertex circle faces);
  - Zero-acting turns at a site mint, if its cover cannot be built;
  - Zero-acting decisions on spline pairs that have no cover.

  The measurements below would name them; they come back to you before landing.
- *Sequencing, not a fork.* The open row `work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md` asks whether the dihedral's lever should be distance from the edge rather than from a point. Point 6's region makes that a change of reader, not of door. Decide it after this.

**Measurements before this goes to you.**
- M1. Straddles per row, on the same suites. Judge each against the bodies tier 3 accepts at rest: a straddle on a valid body is a completeness loss, to be reported as a class; one on an invalid or ill-posed body is the intended refusal.
- M2. Middle pivot versus datum pivot: count the verdicts that change. Any definite verdict that becomes Zero or a straddle was an unsound claim before.
- M3. Straddles where both signs lead to the same action. If there are none, point 4's map is not needed; take sign agreement and drop it.
- M4. Bound from below with edge samples: how many straddles it cures, and what the symbolic tier pays for it (`wedge_reach`'s docs say interior points are costly there).
- M5. Which faces really read no cover, and why. `Surface::cover` gives a control-net cover for `Nurbs`, so "NURBS has no cover" may really be a placeholder or poisoned net mid-construction. A face of a finished body with no cover is a defect; the target is zero.
- M6. Faces whose cover reaches less far than one of their own boundary edges' covers; the target is zero (item 5). Also re-check the ratio figures once the pivot moves, since all were measured from the span's middle.
- M7. Readings whose bound from below is definite but whose bound from above reads differently. This must be zero for levered margins; any hit is a margin that is not of the levered form.

Confidence:
- Premise correction: sure.
- Points 1–3, 5 and 6: likely to sure.
- Point 4 (action rather than sign): likely.
- Point 7: likely.
- Point 8: unsure until M4.

## For the orchestrator

- **The brief may be wrong on item 2.** `face_cover_in` hulls `Surface::Nurbs` by `surface.cover()`, which is `net(...)` over the control points (`crates/geom/src/cover/readings.rs`). So the 1.24M poisoned reads at `euler::certify_edge_spec` are probably placeholder surfaces (`Unknown::PoisonedNet`'s doc names the placeholder) or faces not yet attached. I did not confirm which.
  - Certification reads only from below there (`.below()` in `euler.rs`), so that path is unaffected either way.
- **Unchecked.** Whether any caller matches on a certification refusal's kind and acts on it (for example, retrying as a tangent). If one does, it acts on Zero, and changing that refusal from Zero to a straddle changes its flow. Grep the `CertifyError` matches before converting.
- **Unchecked.** That `RimRouting` and `SingularMiss` callers carry an `Indeterminate` for in-band readings. Neither enum has such an arm, so the in-band case presumably travels in the surrounding `Result`. I checked `MustCarryVerdict` / `PairRefusal` (`InBand`), `PinMiss` (`Escalated`) and `SeamUnread` (`Lever`).
- **Possible defect, not filed (I was read-only).** The fold in `wedge_reach.rs` that takes the points into the bound from above ("mid-door a face's vertex can stand off its carrier") is evidence that either `face_cover` can miss its own boundary or a door's face record is not the face it governs. The session box relies on face covers, so this belongs as a row on whichever slate owns `topo::cover`, citing `WedgeReach::reach` and the 0.29 ratio.
- **Possible defect, not filed.** The doors' pivot (`carrier.mid_point` in `edge_reach` and `WedgeReach::reach`) versus the datum at `interior_stations` and at the joint vertex. This is a same-point violation today, independent of the bounds. It overlaps the honeone row.
- **Provenance.** The lever-bullet sentence and the records clause come from commit `f2a9024a45` (PR 7141, `[ev]`, agent-written). Ev's only comment was "sounds great! i am less sure about the records thing but it seems fine unless there's some kind of weird information flow that results". Read through the REST API.
- **Assumption, unchecked against `Interval` rounding.** Every turn-lever margin is of the levered or sagitta form, so its size grows with L. M7 tests this.

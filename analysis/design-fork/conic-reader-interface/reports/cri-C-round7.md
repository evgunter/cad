## For Ev

**Decision asked.** Three things, and one sentence in the README.
1. Code outside `conic.rs` stops taking a conic's centre or frame. A conic states where it is as one opaque **position**.
2. The kernel names one **polynomial along a conic**, in `geom`, built from the position. `geom` states its numbers; `geom-brep` makes every decision on them.
3. The centre becomes a wrapped datum, `Centre`, which no decision reads.

**Terms.**
- *Placement*: which point a conic stores, its centre or a point of its arc (PR 7258). Private to `conic.rs`.
- *Position*: `P(θ) = stored + U·ξ(θ) + V·η(θ)`. `stored` is the one stored point; `U`, `V` are two vectors. At a centre, `ξ, η` are `cos θ, sin θ`. On the arc they are `cos θ − cos θ₀, sin θ − sin θ₀`, formed without cancellation, with `θ₀` the stored point's parameter. No reader sees which.
- *Polynomial along a conic*: a reading composed with the position. Height over a plane is degree 1; the residual against a sphere, wall or cone is degree 2; against a torus, degree 4. It is a polynomial in `ξ, η` whose constant is the reading at the stored point.
- *Datum*: a number about the whole conic (its centre, its axis line), not a point of it.

**What was measured.** Scratch branch, f64. The carriers are cone sections minted by the real `plane_cone_section`, semi-axis `a` from 1e4 to 1e9 m, nothing axis-aligned. The truth is the curve the stored numbers state, in 60-digit arithmetic, with no kernel code in it.
- **A first-degree root through the plane door**, within 3 m of the stored point, `a` = 1e6:
  - 4e-9 m off as the tree stands;
  - 1e-9 m with the constant taken from the stored point and the solve unchanged;
  - 2e-15 m posed about the stored point (at most 1e-14 at every size).

  So the right constant is not enough; the solve belongs with the value. Far from the stored point (θ = 0.5 and 2) the three agree, at the parameter's last bit times the speed. That is the angle parameter's own limit, which this decision does not touch.
- **A sphere beside the stored point, the existing subdivision unchanged.** 480 poses, each a true double crossing. At ε 1e-9, 261 certify today and 456 with the residual formed about the stored point; at ε 1e-12, 125 and 216. No wrong answer in 2,880 runs. Today's refusals are digits lost in harmonics about the centre.
- **Wrong on main today.** Three public curve-level doors answer wrongly for a section held on its arc, each deciding on its derived centre with no account of its rounding:
  - `curve_pair::meet` certifies no meeting for a line through a point of the arc (from `a` ≈ 1e8 at ε 1e-9, from 1e5 at ε 1e-12);
  - the plane door answers Miss for the ellipse's own plane;
  - `Curve3::on_carrier` answers Off inside the band's gap.

  The tree before PR 7203 gives the same verdicts. No body was found built wrongly: `split` builds such a section only to `a` ≈ 6e5 at ε 1e-9, because the edge certificate's size limit refuses the carrier first, and every vertex it built stood within 0.06 ε of the truth. A fix for these three is being built now, in this direction.
- **By reading, not run:** 22 of the 63 remaining ellipse readers decide on the derived centre unmetered.

**Not measured.**
- `Interval`, except where the suite runs it (below).
- A wall, a cone or a torus in place of the sphere.
- Whether any moved f64 bit is nearer the truth.
- How many pieces the subdivision walks (its cost).
- The 22 suspects beyond the three that were run.

**Who does what.**
- **`geom`, the polynomial:**
  - sums and products;
  - its value and derivatives at a parameter, each with a rounding bound read at that parameter (in the probe that bound held at every point checked, worst error 0.36 of it, where today's one whole-turn noise is what refuses);
  - a whole-turn bound on its top derivative (enough in the sphere run);
  - its range over a window;
  - at degree 1, its extremes and roots.
- **`geom`, the conic:** its speed over a window (`speed_over`, which exists). Re-reading the logs, every root-slack refusal left at ε 1e-9 passes at the span's own speed; the meter reads the whole turn's top speed today.
- **`geom-brep`:** which polynomial a surface asks for; every band, sign and meter; the subdivision; the quartic ladder; every refusal. The ladder forms its five coefficients from two of the polynomial's jets (measured: right to 4e-16 m in the median, against 9e-5 m from centre harmonics). It asks the polynomial nothing else.

**First-degree roots: one closed form per placement.**
- **At a centre:** the crest and half-chord main uses, unchanged.
- **On the arc:** measured from the stored point, a quadratic in the tangent of half the angle from it.
- **Why not one form:** the suite run that forced the arc's form onto centre-held conics made two `Interval` rows that built escalate (a touch's two roots enclosed as one wide interval).
- **On `Interval`:** a conic on its arc may state both forms and keep the tighter, since each is a sound enclosure. That is by argument, not measured.
- **No first-degree door certifies its root today.** The plane door returns a closed-form root unchecked and its callers split the edge there. That is `geom-brep`'s to fix; this decision gives it a root good to the parameter's last bit to certify.

**The centre.** `centre()` returns a `Centre`: stored or derived, never panics, and cannot be added to a vector.
- **Two doors only:** `axis_line()` and one coordinate door, both on a gated list with a reason per caller.
- **Callers, a closed list:** reporters into a format that is centre-form by its own definition (the STEP writer, a sketch arc written back) and mints of a new entity's datum (a blend surface about a rim's axis).
- **A circle's own edge description is not on the list.** It is built from the conic, or from the axis of the surface the edge lies on.
- **Decisions read the position.** Of the 79 circle readers, 39 decide something (coaxial, a parallel of, on this surface, one circle). Each is a test that a polynomial is constant or zero along the conic, which is today's arithmetic on a centre-held circle.

**Also in the final state.**
- **Off `Conic`:** `frame`, `along`, `semi_axis_vectors`, `unit_in_plane`, `u_ref`, `v_ref`. `axis` stays. The principal frame is asked as `major_frame()` / `section_frame()`, which state the parameter at their vertex.
- **A point against a conic** is three readings from the stored point: its parameter, its height over the conic's plane, its level (inside or outside).
- **A conic made from a conic** is a map on the conic: the existing ones, plus a homothety, a concentric offset and a re-seam at a point. Seven records that carry a circle as centre, axis and radius hold the `Circle`.
- **Two conics compared** is three doors: one stored value (`same_content`); one stated locus exactly, either sense; one conic's level along the other's position, whose sign pattern is apart, nested, tangent or crossing.

**What changes for a centre-built conic.** PR 7258 told you nothing about it changes.
- **Still true of the curve:** its stored numbers and its evaluator are untouched, by construction.
- **Still true of first-degree readers:** they keep main's form.
- **Not true of readers of degree 2 and up.** They read a bound at the point where they read one whole-turn noise, and verdicts move. Measured by forcing every conic through the scratch polynomial over the `geom-brep` and `topo` suites (6,514 tests, one fuzz seed), 57 of 4,252 sphere and wall door calls change verdict:
  - 49 from a refusal to an answer the suite's own exact oracle accepts;
  - 4 the other way (2 of them the scratch's double charge on `Interval`);
  - 4 one refusal for another.
- **Three more failing rows are untraced:** one `Interval` escalation on a sliver, and two pinned query counts.
- None of the moved numbers was judged against an independent truth.

**README, replacing "Every reading of where a conic is goes through the conic."** (agent-written, approved in passing on PR 7258):
> A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

**Order.** The fix under way builds plain readings on `Conic`, each from the stored point; under this decision they are sugar over the position. The remaining readers are converted before the edge certificate's size limit is lifted for a conic held on its arc; otherwise the 22 suspects become reachable through `split`.

**Pushback I expect.**
1. **The machinery against the need:** only sections far from their centre gain, and none that large is built today. My answer: three wrong answers at public doors; 261 against 456; and the type replaces about eleven private harmonic structs.
2. **Centre-built conics' verdicts move**, where PR 7258 said nothing changes.

**Confidence.** The position, the polynomial and `Centre`: likely. The cut: likely for a sphere, unsure for a cone or torus. First-degree form per placement: likely at f64, unsure on `Interval`.

## For the orchestrator

1. **What the measurements change.** Nothing I signed is withdrawn; the polynomial's list shrinks.
   - **The half-angle question goes.** M3 shows the ladder's five coefficients are two jets, which `geom-brep` can form itself.
   - **"The magnitude its rounding is metered against" becomes a bound read at the parameter.** M1 door D: the uniform noise "does not see where the constant was read".
   - **Windowed ranges of derivatives leave the must-list.** M2 certifies with a whole-turn `|F⁗|`; the cost is unmeasured.
   - **First-degree roots stay with the value, one closed form per placement** (For Ev). My earlier "posed about the stored point" was ambiguous for a centre. M4 built the reading I did not intend, tan(θ/2) about θ = 0 on a centre-held conic, and showed it is wrong there.
   - **"No first-degree door certifies" is a `geom-brep` defect and does not move the cut.** The polynomial states the roots and a bound at each; the door owes the slack decision. M1 door D shows it can: (iii) certifies 3 of 6 at 1e4 where (i) and (ii) refuse.
2. **The windowed speed is the conic's, not the polynomial's, and the cut does not change.** `Conic::speed_over(span)` is already on the trait. The slack, monotone and width rows should ask it where they ask `speed_hi`.
3. **Centre-held bits.** Final state as in For Ev: one reading for both placements at degree 2 and up, per-placement closed forms at degree 1. I withdraw "the opaque type may hold Fourier coefficients to keep bits": M4's moves come from the local bound, and they are mostly refusals becoming answers, so keeping the old numbers would keep the worse reading. Ev must be told that verdicts on centre-built conics move.
4. **The class changes urgency and order, not the design.** It is the measured reason no decision reads `Centre`. The fix unit should not build:
   - a public accessor for the stored point or phase (the probe's `probe_stored`);
   - a first harmonic returned as constant and amplitudes for callers to solve (M1's variant (ii)). Return an opaque degree-1 value with range, extremes and roots, so it becomes the polynomial's first degree and not a second type;
   - the arc's root form applied to centre-held conics;
   - a rounding charge for the derived centre added to the suspect decisions. That is charging where the README says re-pose, and it would be undone;
   - any height or level reading that subtracts a derived centre inside.
5. **Measured or judged in ways to weigh.**
   - **M4's first-degree mode** measures a form none of us signed for a centre. The plane door's and `meet`'s 606 moved calls and the two `Interval` escalations belong to it.
   - **M2's jet bound** is a hand `rounding_charge` checked at sampled points. The final one is running arithmetic, as `running_point` is.
   - **M2's "all 22 slack refusals pass"** is a re-read of logged numbers at the far end's speed, not a re-run. That speed is the span's maximum only where it is monotone; `speed_over` is the sound number.
   - **M1's truth** takes the stored frame entries with an exact cross product. Fork 296 left "one stated truth for a stored frame" open. Near the stored point the choice cannot matter (frame error times the offset); it could in the "far" columns.
   - **The wrong answers at ε 1e-9** begin at `a` ≈ 1e8, well past what `split` builds. Sweep 2 is a reading.
   - **The ladder.** In M2 and M3 it contributes only spurious escalations: the subdivision refuses every graze itself. Whether it is needed at all belongs on its filed row.

**Off the question, to check or file.**
- **`centred_at` may have a production caller.** Sweep 2 row 19 (`pcurve_cache::anchored`) calls it. Two reports said only tests do.
- **`Centre`'s caller list is classified for the 79 circle functions only.** On the ellipse side:
  - rows 46 to 48 take an ellipse's `axis_line()` straight off the mint and are untraced;
  - rows 58 and 59 are reported values;
  - row 61, the test oracle `conic_oracle/exact.rs`, states another curve than the evaluator for an arc-held ellipse;
  - row 22 supports the exact-locus door reading the placement inside `conic.rs`.
- **The README's "consumed extent" clause** ("a datum fixed by a carrier's form (an apex, a centre) is read there") was flagged by another designer. It is a separate question and I have not folded it in.

Nothing was built or run by me this round.
## For Ev

**Short answers**
- **How ugly?** Small. A circle and an ellipse each hold their directions and sizes once, plus a *placement* with two variants:
  - **centre-held**: the centre;
  - **arc-held**: a point of the arc the conic serves, with its parameter zero there, and for an ellipse where on the ellipse that point sits.

  Each variant holds only its own numbers. Only `conic.rs` ever asks which one a conic has. The ugly work is elsewhere: about a hundred readers rebuild the curve from its centre, and they have to be fixed under any form.
- **Is it the family pattern?** Yes, as above.
  - The tree has the pattern at two layers. In the recipe, a sketch arc is written as `Bulge`, `Via`, `Center`, `Sweep` or `ArcLen`, each keeping only its numbers and all lowering to one `Segment::Arc`. In the kernel, `Curve3`'s kinds sit behind shared readings, and circle and ellipse sit behind `Conic`.
  - What is built is not the pattern. It is one struct with a hidden tag, and an ellipse field documented "unread" under one value of that tag.
- **No cutoff.** Each construction makes the same variant every time, at every size:
  - an authored centre is held as a centre;
  - a cone section's near vertex, a swept arc's start point and an imported edge's vertex are held on the arc.

  Nothing ever converts one variant to the other.
- **The panic.**
  - **Yes, where the kernel chooses.** Some kernel cuts have a centre that runs off to infinity at a pose a person can reach: a cylinder cut almost along its axis, two equal cylinders almost parallel. Each of those mints must hold a point of the arc. A debug assertion in each one makes holding a centre there a kernel bug, never something a person meets.
  - **No, as a size limit on centre-held conics.** A person reaches any size legitimately: a gently curved wall whose axis is kilometres away, a 3 km `Center` arc, a STEP file, a scaled body. A panic there would fire on real models.

**What happens past the length where rounding stops fitting** (likely)
- A centre-held conic is read near its arc by short chords from its seam vertex, `centre + u·a`. The size table measured this to certify exactly what arc-held storage does, from 10 m to 10 km, at ε = 1e-9.
- What remains is that one sum, which rounds at u·|centre|. It exceeds the run's headroom `(K − 1)·ε` once the centre is more than about `(K − 1)·ε/u` away. At K = 10 that is about 80,000 km at ε = 1e-9 and about 80 km at ε = 1e-12.
- So the length depends on the run's ε. It is the run's admission rule, not a property of the curve.
- What happens there: a check that cannot read its difference refuses with a typed error and the recourse "bring the magnitude down or the tolerance up". The tree already does exactly this for sketch arcs (`ArcBelowSceneResolution` in `profile/src/validate.rs`).
- It is the person who meets it, on that model at that ε, with a stated recourse.

**The other proposals, and what is wrong with each**
- **A typed refusal at every door a person sizes, once a semi-axis passes the rounding reach R, plus an assertion behind it.**
  - Size is the right thing to bound if anything is bounded: a move cannot change it.
  - The objections:
    - it refuses geometry that reads correctly: R is about 16 times shorter than where the one sum stops fitting;
    - it adds a gate on stored terms, which "Float rounding" rules out ("never a carrier's stored terms");
    - it makes a document's validity depend on ε at its doors, which the admission rule already handles at the check.
- **An assertion at the session box's diagonal.** I proposed this last round and withdraw it. Far-axis walls and STEP files legitimately make centre-held conics far larger than the box, so it would panic on person input.

**At `Interval` and the symbolic scalar** (likely; arithmetic, not measured)
- The chord reading `(c + u·a) + u·a·(cos θ − 1)` holds the semi-axis twice. Every enclosure it gives is at least as wide as the plain centre form `c + u·a·cos θ`, and up to three times as wide on the far side.
- So the centre-held conic reads by chords at f64, and about its centre at the enclosure scalars. The tree already reads a frame differently by scalar kind in the same way (`Ellipse::quarter_turned_where`).
- That gives back main's enclosures and the symbolic identity (the sweep names the centre's own node) for authored circles. Arc-held conics read by chords at every scalar.
- The lane should confirm this on the ten `Interval` rows the one-form build moved. It does not block the sentence.

**Moves, reversals, re-seams and scalings**
- A **move** maps the stored point and the directions.
- A **reversal** flips the axis. An arc-held ellipse negates the sine of its point's anomaly.
- A **re-seam** turns a centre-held conic exactly about its centre. An arc-held conic is never re-seamed, because its parameter is zero at its stored point.
- A **scaling** maps the stored point and multiplies the sizes.
- In every case the variant is kept. `replace_face::homothety` today rebuilds through the centre door; it must map the stored point instead.

**Contracts:**
- a cover holds its arc (through the conic);
- a carried certificate equals a fresh one;
- an exact centre at the box scalars for a centre-held conic;
- the STEP round trip's fixed point, which is import's to provide.

Last bits and digests are habits.

**If you need to rule on the remaining split, one line each:**
- **(i)** No numeric limit. The kernel asserts only in the runaway mints, and past the rounding length a check refuses typed with its recourse. (I lean this way.)
- **(ii)** A typed refusal at each door a person sizes, once a semi-axis passes R, with a kernel assertion behind it.

**Measurement that decides (i) against (ii):** build a centre-held conic with its centre 1e5 to 1e7 m out and its arc in the box, through a real door, and certify it at ε 1e-12 and 1e-9.
- Only right answers or typed refusals: (i).
- Any wrong definite verdict: (ii), at the length the probe finds.

**Sentence for "Float rounding"** (`crates/geom-core/README.md`):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there; its construction fixes which, the same at every size, and a construction whose centre runs off at a pose it can reach holds the arc. At `f64` a point is summed by short chords from a point of the curve, a centre-held conic's seam vertex; at the enclosure scalars a centre-held conic reads about its centre. Every reading of where a conic is goes through the conic."

**Confidence:**
- the family of placements, fixed by construction: likely;
- panics only in the runaway mints: likely;
- the scalar split at the enclosure scalars: likely;
- that past the length the outcome is a typed refusal rather than a wrong answer: likely by reading, and the probe above settles it.

## For the orchestrator

**What moved me, and to what**
- **The shape: from one layout with a pair `k` to an enum of placements inside each kind** (the first report's shape).
  - The second report is right that `k = (0, 0)` is an in-band value standing for a kind.
  - The two kinds now differ in their parameter convention (zero at the stored point against the angle from the seam) and in their enclosure-scalar evaluator. So my single formula no longer describes the code.
  - I prefer the placement enum to two types per kind. DESIGN `[one-way]` asks that the shared part (directions, sizes) be one type each kind holds. Two types per kind repeats it four times.
  - Either is fine to the lane, under these constraints: no unread field; only `conic.rs` matches on the variant; re-seam only on the centre-held variant.
- **The panic: from a box-diagonal assertion to no numeric limit** (the second report's position).
  - Its door table shows that far-axis surfaces, STEP and authored `Center` arcs legitimately produce large centre-held conics. My diagonal would panic on them.
  - I add what that report lacks: the explicit length where the single seam-vertex sum leaves the headroom, about `(K − 1)·ε/u`, and what happens there. The answer is the admission rule's typed refusal, which `ArcBelowSceneResolution` already implements for sketch arcs.
- **The enclosure scalars.** The second report raised the doubled semi-axis. I resolve it by arithmetic rather than by measurement: every chord-form width contains the centre form's widths plus more. Lane confirmation is owed, not blocking.

**Errors I see in the others' reports**
- **Second report:** "no door today checks a carrier's size or centre". `profile/src/validate.rs`'s `ArcBelowSceneResolution` refuses an arc whose stored magnitude the tolerance cannot read past. It fires conditionally, on an unreadable check, and is not a blanket gate, but it is the precedent for my answer.
- **First report, part 2:** R as the bound.
  - The chord reading's one sum costs u·|c|. That leaves the headroom only at |c| ≈ 16·R.
  - So R is a short-chain constant applied to a single sum.
  - A door refusal at R refuses models that certify.
- **First report, part 1:** "every point is summed by chords from a point of the curve" at every scalar. That is wider at `Interval` for centre-held conics, as above.

**Still differing, and why**
- **Against the first report's part 2 (door refusals at R).** As above. It also needs every outside-fed door to carry an ε-dependent gate: revolve, STEP, homothety, `ConstructedProfile`, possibly `junction::scaled`. Five gates to maintain, against one existing pattern at the check. If the probe finds wrong verdicts, I would move to it, at the probed length.

**Runaway-pose mints (the assertion sites)**
- The reports' union: the plane–cylinder tilted ellipse, `bisector_ellipses`, the plane–cone circle across the axis, blend's oblique end face, STEP's recognised circle, offset's sphere and torus seam circles.
- They overlap, and the lane should settle the list. The plane–cone axis-normal case is "far apex": that may belong to the far-surface-datum row (`a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`) rather than here.

**Owed**
- The probe above.
- The `Interval` confirmation on the ten moved rows.
- A `profile` row: `Segment::Arc` erases the spec at lowering, so a bulge arc's 2-D centre is itself far. This is the second report's finding, not filed.

**Housekeeping:** I read `round4/message.md`, the two reports, and `validate.rs` on `origin/oneround/anchored-conic`. Nothing was edited or run.

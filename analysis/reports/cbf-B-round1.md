## Reconciliation answers (round 1)

**1. The parameter. I was wrong; A's parameterisation is the one to forward.**
- **My θ₀ form (ISO parameter kept).** Take a = 1e4 and an anchor 0.45 rad from the nearest vertex.
  - Near the major vertex: θ₀ ≈ 0.45, ulp ≈ 5.6e-17, speed ≥ a·sin 0.45 ≈ 4.4e3 m/rad. Representable points along the arc are about 2.4e-13 m apart.
  - Near the minor vertex: θ₀ ≈ 1.12, ulp ≈ 2.2e-16, speed ≈ 9e3 m/rad. Spacing is about 2e-12 m.
  - Both are of order u·a, wherever the anchor sits.
- **A's form (parameter zero at the anchor).** The spacing at a parameter s is about speed·2u·|s|, so about 2u times the arc distance from the anchor. That is about 2e-18 m at 1 cm, and the anchor itself sits at s = 0 exactly.
- **What mine gives up.** Mine restores exactly the `ulp(θ)·A` parameter term that PR 4861's re-pose (seam at the nearest vertex) was built to remove. An edge's end parameter would land up to about u·a away from its vertex along the tangent, so the endpoint residual goes back to u·a.
- **The "1e-18 m" in my example.** It held only across the arc: a point at a representable parameter lies on the curve to that. Along the arc it did not hold.
- **Cost to the roughly 60 sites that pair the parameter with `u_ref`, `v_ref` or the semi-axes.** Each must read the principal angle θ₀ + s instead of s.
  - Give them one `Conic` reading: the principal (cos θ, sin θ), computed from the stored phase pair by the angle-sum products, with no angle addition and no centre. Each site then keeps its shape.
  - Direction, curvature and support readings see only relative-u error, which is harmless.
  - Principal-vertex parameters (−θ₀ + kπ/2) stop being exact constants. At a vertex the error is second order in a support, but each such site needs a look.
  - Sites that rebuild a point from `c + a·cos θ·u` are the defect sites themselves, and must go through the evaluator.
  - `step-export`'s "the two parameterizations agree" becomes "agree up to the stored phase". B-rep edges export by vertices; I did not check whether any exported entity carries conic parameters.
- **Small residue on A's phase pair.** (cos θ₀, sin θ₀) is not exactly unit. Say it is read as given (exact), like the frame question in PR 7131.

**2. The derived exported centre: a consequence to state, not its own question.**
- `[edges]` makes the carrier "a derived cache … never a peer of the description", and what the user wrote is kept in the document, which persists and replays bit for bit.
- What the user sees: in an exported STEP file, a circle authored centred at 0.1 can print its centre as 0.09999999999999998. The same holds in any inspector that shows a carrier's centre. Their document still says 0.1.
- Today an untransformed authored circle prints 0.1; any placed or moved one already prints a rounded centre.
- So it belongs in the consequences Ev is told; no separate fork. A was right to name it, and I had understated it.

**3. What moved me, and what stands.**
- Moved:
  - the parameter (above);
  - naming the form in Float rounding: a clarification, not a decision change, so I now include it;
  - PR 7074 (the `[import]` rewrite, after PR 4812) strengthens the import premise;
  - A's two branch defects: `plus_zero_curve` drops the anchor, and the re-seam goes through the derived centre;
  - A's point against storing conjugate semi-diameters.
- Stands, consistent with A:
  - the fixed point is import's idempotence to provide;
  - the carried-certificate pair is to be re-derived;
  - the verdicts are gated on an oracle.
- Not answered either way: why the carried-certificate rows moved. My guess was vertices placed outside the conic's evaluator. Neither of us traced it.

---

## For Ev

**Recommendation: one stored form for every conic, anchored on the arc its edge uses, with the parameter zero there and the centre never stored.** (likely)
- A circle or ellipse stores:
  - its *anchor*, a point of the curve on the arc the edge uses (in the box), where the parameter is zero;
  - the principal directions and the semi-axes;
  - for an ellipse, its *phase*: where on the ellipse the anchor sits, stored as the pair (cos θ₀, sin θ₀), with θ₀ measured from the major axis.
- A point is summed from the anchor by its chord, `P(s) = P₀ + 2·sin(s/2)·[−a·sin(θ₀+s/2)·u + b·cos(θ₀+s/2)·v]`, with nothing to cancel.
- The centre is one derived reading, good to about u·a (u is f64's unit roundoff, about 1.1e-16; a the larger semi-axis). Export writes it.
- A construction handed a centre computes the anchor once; from then on the anchor is the curve. A construction that can put the centre far away (a shallow cylinder or cone cut, a very large circle) computes the anchor in the arc's own local frame instead.
- Nothing records how a conic was born, and nothing is cached.

**Why parameter zero must sit at the anchor.** An edge stores its ends as parameters. A parameter of size θ is representable only every ulp(θ), which moves the point by ulp(θ) times the curve's speed along the arc.
- Example: a = 1e4, anchor 0.45 rad from a vertex.
  - If the parameter stays measured from the major axis, representable points near the anchor are 0.2–2 picometres apart (2e-13 to 2e-12 m), of order u·a. An edge's end can then miss its vertex by that much along the curve, which brings back the term your first-step re-pose (PR 4861) removed.
  - With zero at the anchor, the spacing is about 2u times the distance from the anchor: about 2e-18 m at 1 cm.

**Premise check**
1. **No source supplies a carrier, so there are no source bits to keep.** (sure on the texts; likely that this settles it)
   - `[edges]`: the carrier is "a derived cache … never a peer of the description".
   - `[import]` (rewritten on PR 7074, after PR 4812): "no value the file printed becomes a cache". `step-import` builds every edge's carrier with the kernel's own section construction.
   - So "an imported ellipse keeps the file's centre form exactly as read, with an anchored cache" describes a state the import pipeline does not produce. An imported conic is a kernel-minted section.
2. **The clause in question is agent text you approved in passing.** On PR 4812 your words were "import should re-pose in a sense, but ideally it would round trip nicely … a cached internal representation". "Evaluators read the anchored form either way" was the agent's summary; you answered "sounds good!".
3. **The measured "only about 5×" assumes the centre is still the truth.** (likely)
   - It compares the original centre-form curve with an anchor recomputed from that centre on every read. Against that curve, nothing at f64 does much better than u·far.
   - A stored anchor moves the curve once by about u·far, the same order as any construction's rounding. After that, points near the arc hold to u times their distance from the anchor, at f64 and at `Interval` alike.

**Text to change**
- The PR 4812 record (GERM row "Decided", and the unit's seam note):
  - replace "stored in the form it was born in", the imported-ellipse sentence and "evaluators read the anchored form either way" with the recommendation above;
  - "parameter zero there" stays.
- The geom-core README, "Float rounding": name the form, so that "re-minted in a well-conditioned form" reads "a conic is stored anchored on the arc it serves". This is a clarification; `[simple.conditioned]` already requires it.

**The answers as final states**
1. **One anchored form (recommended).**
   - Every conic reads well near its arc.
   - A far-centred centre form cannot be written down.
   - A frame's rounding multiplies only short chords, so it no longer depends on PR 7131 and the at-rest frame check stops scaling with the semi-axis.
   - The bare-rounding gate's "far-centre" sites, the fitted resolution constant and the re-seaming doors lose their reason.
   - Costs:
     - every conic's last bits move;
     - the roughly 60 sites that pair the parameter with the axes read the principal angle through one `Conic` reading;
     - a principal vertex's parameter is no longer an exact constant;
     - an exported centre is derived, so a circle you typed centred at 0.1 can print in STEP as 0.09999999999999998 (your document still says 0.1);
     - `step-export`'s "the parameterizations agree" becomes "agree up to the stored phase".
   - Reversible: the form sits behind the `Conic` readings.
2. **Two forms, each read as born (what is built now).**
   - Keeps every existing bit.
   - Its type means two things (an optional anchor beside a stored centre), with two evaluators.
   - Every far-centred circle and every shallow cylinder cut stays at u·far, against Float rounding.
   - Its only reason is bit stability, which may choose between spellings but may not justify keeping a second implementation.
3. **The clause as written: anchor recomputed from a stored centre on each read.**
   - Pays every bit move for a constant factor at f64 and nothing at `Interval`.
   - Dominated.
4. **Centre form with exact (compensated) sums.**
   - Needs a new scalar operation, gives nothing at `Interval`, and is not exact under PR 7131's reading of a frame.
   - Dominated.

**Contracts and habits**
- **Contracts, kept:**
  - same inputs give the same bits (D2/D9), mesh bytes at one and four threads included;
  - reversal stays exact;
  - new and stronger: the point at parameter zero is the stored anchor, bit for bit.
- **To re-derive, not re-baseline:**
  - "a carried certificate equals a fresh one" (2 rows);
  - our own dialect's STEP fixed point. It rests on "evaluate, invert, evaluate" returning the same bits, which no form guarantees; import should provide it.
- **Habits, to regenerate:** quarter-turn points equal to `c ± r` exactly, the 3 bit pins, the 6 mesh digests, the 2 query counts.
- **Owed before landing:** the 4 changed verdicts, each checked against an exact oracle. This gates landing; it does not rank the answers.

**Confidence.** Recommendation: likely. Parameter zero at the anchor: sure (from the representable spacing above). Import never keeps a printed curve: likely (docs plus `build.rs`, not traced end to end). Options 3 and 4 dominated: sure.

## For the orchestrator

- **Provenance.** PR 4812's comments were read on GitHub: the clause is from comment 6132205976, Ev's own words are in 6132196517, Ev's approval is 6132885636. Float rounding's "re-minted" sentence comes from commit 31576a6c6 (the companion table says ratified on PR 6329). I did not read PR 7074 myself; it is cited from the round's other report.
- **Where I still differ from A:** nowhere of substance. I conceded the parameter, and now include naming the form in Float rounding and the derived-centre consequence. A slight emphasis remains: I would have mints that can put the centre far compute the anchor in their local frame, not through a centre door. A's text implies the same.
- **Defects on `origin/oneround/anchored-conic`, to file.**
  - (a) The anchored constructors store both the anchor and the derived centre (as `frame.origin`).
  - (b) The anchor is fixed at the u_ref vertex.
  - (c) `plane_cylinder_section` is left centre-born, although the decision covers it.
  - (d) `step-import` `signed_zero.rs` `plus_zero_curve` rebuilds via `Ellipse::new` and drops the anchor (A's finding; I did not re-check it).
  - (e) `quarter_turned_where`/`seamed_toward` re-anchor through the derived centre (A's finding; I did not re-check it).
- **Off-question.**
  - The `freecad::cross_dialect_fixed_point` two-cycle belongs to import's vertex-foot idempotence.
  - The carried-certificate rows (`revert_plane_charts`, `snowman`) are untraced.
  - Quadrics whose axis, centre or apex is far from the face they bound are the same disease one level up, and have no row.
  - `tangent_circle` builds far-centred circles through the centre door.
- **Not checked.**
  - Whether any exported STEP entity carries conic parameters, which would make the phase shift visible.
  - The admission floor of the chord form at a general phase (modelled only).
  - Whether `section_near` reaches the cone's anchored arm.
  - No builds were run.

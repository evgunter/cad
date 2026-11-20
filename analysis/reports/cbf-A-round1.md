**Reconciliation answers (round 1)**

Your read is right, with one nuance: I class the STEP fixed point as import's to provide under any form, and only the carried-certificate identity as "re-derive the gate".

1. **The parameter: forward zero at the anchor.** Take a = 1e4, the anchor 0.45 rad from the nearest vertex, b ≪ a. The curve moves about a·sin 0.45 ≈ 4350 m per radian there.
   - ISO parameter (B): values near 0.45 are 2⁻⁵⁴ ≈ 5.6e-17 apart, so representable points are 2.4e-13 m apart along the arc, however close to the anchor. It scales with a: 2.4e-11 m at a = 1e6, past ε = 1e-12. Every vertex, root and cut on the arc lands on that grid.
   - Zero at the anchor (mine): spacing is u times the distance from the anchor: 1.2e-16 m at 1 m, exact at the anchor.
   - B's evaluation is fine for a given θ, because θ − θ₀ is an exact subtraction. What it gives up is what an edge can store. That is the parameter term PR 4861 was built to remove, about 0.2·u·a here, back on exactly the conics that need a general anchor (the cylinder's shallow cuts).
   - The two forms differ only on an ellipse anchored off a principal vertex. A circle turns its seam to the anchor, and a vertex-anchored ellipse has phase zero, so the forms differ only where B loses resolution.
   - "Edge parameter intervals and export are untouched" is a cost of the change. Every conic span is re-minted anyway, and an exported edge curve carries vertices, not parameters.
   - Cost to readers: an ellipse's parameter is no longer its eccentric angle. The sites that take `sin_cos` of the parameter against `u_ref`, `v_ref` or the semi-axes read the angle's sine and cosine from the conic (one door, angle addition with the stored phase). Linear readers (a harmonic along a direction) can take conjugate semi-diameter vectors and need no phase. Circle sites pay nothing. I did not split the lane's 60 by kind.
2. **The exported centre is a consequence, not a question.** `[edges]` makes the carrier a derived cache, and what the user wrote lives in the recipe. The only place a carrier's centre shows is the text of a STEP file, where a `CIRCLE` or `ELLIPSE` centre can differ in the last digit from the axis point printed for its surface. Ev accepted that for minted ellipses on PR 4812. I keep one line of it for Ev and drop the "typed 0.1" wording, which wrongly suggested the user's value is touched.
3. **What moved me.** B is right that "Float rounding" needs no change; I withdraw that edit. B's guess at why the carried certificates moved (vertex points made by centre arithmetic outside the evaluator) is a better lead than my none. Nothing in B answers the resolution argument in 1.

## For Ev

**Recommendation (likely): one stored form. Every circle and ellipse carrier is stored anchored, its parameter zero at the anchor, and the centre is never a field.**

- A carrier stores a point of the curve inside the session box (the *anchor*), the frame's directions, the sizes and, for an ellipse, where on the ellipse the anchor sits (its *phase*).
- The parameter is measured from the anchor, as the PR 4812 record already says ("parameter zero there").
- The centre is one derived reading, good to about u·a (u is f64's unit roundoff, a the larger semi-axis).
- A door that is handed a centre computes the anchor once. From then on the anchor is the curve.
- "Born with a centre" stops being a stored distinction, and the "anchored cache" disappears: with one form there is nothing to cache.

**Premise check: three corrections**

1. **Nothing gives a carrier, so there are no given bits to keep.** (sure on the texts and the tree; likely that this settles it)
   - `[edges]`: the 3-D carrier is "a derived cache", "never a peer of the description".
   - `[import]`, rewritten on PR 7074 (2026-11-17, five weeks after PR 4812): "no value the file printed becomes a cache". Your words there: "nothing imported should be able to reach a state that can't be reached by native things".
   - The tree does this: `step-import/src/build.rs` builds each edge's carrier by the kernel's own section construction.
   - So "an imported ellipse keeps the file's centre form exactly as read, with the anchored form as an internal cache" describes a state `[import]` forbids and the tree does not have. An imported conic is a kernel-minted section, which you already decided is anchored.
2. **The clause in question is not your sentence.** "Evaluators read the anchored form either way" was the agent's, and you answered "sounds good!". Your words were "import should re-pose in a sense, but ideally it would round trip nicely". PR 7074 has since settled the file's round trip differently.
3. **The "only 5×" comes from keeping the centre as the truth.** (likely; reasoned from the evaluation chains, not run) The measured variant re-sums the zero point from the centre on every read and charges that rounding. With the anchor stored, the error is u times the distance from the anchor: the unbounded gain.

**What the evidence points at**

The built type means two things: an optional anchor beside a frame whose origin is a centre, given on one kind and derived on the other. On the branch `oneround/anchored-conic` the two origins are kept in step by hand:
- import's `plus_zero_curve` rebuilds a minted ellipse through the centre door and drops its anchor (found by reading, not run);
- a re-seam of an anchored ellipse re-anchors through the derived centre, losing u·a;
- every reader of the frame's origin still gets a far-rounded centre.

**Why the parameter is zero at the anchor** (sure on the arithmetic)

A cylinder's shallow cut has no principal vertex near the arc it uses (measured 0.42 to 0.50 rad away), so its anchor is a general point. Take a = 1e4 and the anchor at 0.45 rad.
- If the parameter stayed the angle from the principal axis, its values near 0.45 are 5.6e-17 apart and the curve moves 4350 m per radian. Every vertex, root and cut would sit on a grid 2.4e-13 m wide along the arc; at a = 1e6, 2.4e-11 m, past ε = 1e-12.
- Measured from the anchor, the grid is u times the distance from it: 1.2e-16 m at 1 m, exact at the anchor.
- This is the parameter term the first step (PR 4861, the re-pose at the nearest vertex) removed where a vertex is near. Only zero-at-the-anchor removes it for the cylinder.
- Cost: an ellipse's parameter differs from its eccentric angle (and from STEP's parameter) by the stored phase. Readers that take the parameter's sine and cosine against the principal directions (about 60 production sites by the lane's count, circles included) read them from the conic instead. A circle has no phase: its seam turns to the anchor.

**A worked case for the stored point**

A circle of radius 1500 m through the origin; the point 1 m along the arc.
- From the centre: two terms of size 1500 that cancel, each rounded at about 1e-13 m, fresh on every read.
- From an anchor at the origin: terms of 3e-4 m and 1 m, rounded at about 1e-16 m.
- Handed only a centre, the door rounds once: one rigid shift of the whole curve, the same for every reader, which certification can see.

**Each answer as a final state**

*A. One anchored form (recommended).*
- Makes true: any in-box point of any conic reads at about u times its distance from the anchor, within the run's admission floor (the rounding of a short chain on in-box coordinates). (likely; modelled, not measured)
- `[simple.conditioned]` ("then treated as real numbers") holds for every conic. The seven files the bare-rounding gate lists as `far-centre`, the fitted resolution constant (re-fitted 32 → 40 in this build) and the re-seaming doors lose their reason.
- A far-centred conic in centre form cannot be written down (`[unrepresentable]`).
- It does not depend on PR 7131: a frame's one-ulp width multiplies only short chords.
- Leaves possible:
  - one edge using both flanks of a thin ellipse (PR 4812 already gives one carrier per edge);
  - a door handed a far centre with nothing better known. In production that comes from a surface whose own axis or centre is far: the same disease one level up, with no row filed.
- Consequences:
  - Every conic's last bits move.
  - A STEP file prints a derived centre, as you accepted for minted ellipses on PR 4812. A `CIRCLE`'s centre can differ in the last digit from its cylinder's axis point. Nothing else shows a carrier's centre: what the user wrote is in the recipe, and the carrier is a derived cache (`[edges]`).
- Reversible: yes. The form sits behind the shared `Conic` readings.

*B. Two forms, each read as born (what the lane built).*
- Defensible: on PR 6329 you said another under-the-hood form is "not that bad" (as quoted in the far-centre row).
- Keeps main's bits for every conic made through a centre door.
- Leaves a far-centred centre form representable and silently lossy: every circle mint, the cylinder's cuts, any fixture. That is against "Float rounding" ("a stored form that loses digits … is re-minted").
- Keeps two evaluators, the rounding charges and an anchor a rebuild can drop.
- Its main reason is bit stability, which `memories/output-stability-as-justification.md` says may not justify keeping a second implementation.

*C. The clause as written, centre still the truth.*
- Pays every bit move for a 5× constant, and gains nothing at `Interval`.
- An exact-sum refinement also gives nothing at `Interval` or under PR 7131's reading of a frame.
- Dominated by A.

**Contracts and habits**

- Contracts A keeps:
  - same build and inputs give the same bits (`[determinism]`), mesh bytes at one and four threads included;
  - reversal is exact structure (the anchored sum is exactly even and odd in the parameter);
  - new: the point at parameter zero is the stored anchor bit for bit, and its inverse is exactly zero.
- A contract to re-derive, not re-baseline: "a carried certificate equals a fresh one" (2 rows). `[one-way.places]` carries a certificate only on a bit-identical support. The cause is not traced.
- Owed by import under any form: the one-hop STEP fixed point. It needs "evaluate, invert, evaluate" to return the same bits, which no stored form guarantees. (likely)
- Habits, to regenerate: the 3 bit pins, the 6 mesh digests, the 2 query counts, the quarter-turn identity (it fails on main too when the centre is zero).
- The 4 moved verdicts need an oracle before landing. That gates the landing, not the choice.

**Text I would change**

The GERM row's "Decided (Ev, PR 4812)", first bullet, and the unit's copy of it: replace "stored in the form it was born in", the imported-ellipse sentence and "evaluators read the anchored form either way" with "every conic is stored anchored on the arc it serves, parameter zero there, the centre derived". "Float rounding" and `[simple.conditioned]` already say this and need no change.

## For the orchestrator

**Where I still differ from B**
- The parameter only (answer 1 above). B's "the anchor's parameter θ₀ is stored, the parameter stays ISO" retires "parameter zero there", on which the record's own result ("resolution is u times the arc distance from the anchor") depends. That text is agent-written but appears three times on PR 4812 (the body's recommendation, the agent's first reply, the row) and Ev approved it.
- Not a difference of substance: B stores the phase as an angle, I as the pair (cos θ₀, sin θ₀). I prefer the pair because the mint knows the cosine directly and a quarter or half turn is an exact permutation; unsure, the lane's to settle.

**Provenance**
- PR 4812 and PR 7074 comments read on GitHub; the D7 rewrite is commit `edde229fe`. Ev's PR 6329 remark is quoted from the far-centre row, not checked on GitHub.

**Errors in the brief and the evidence file**
- "An imported ellipse" is not born with a centre as a carrier: `entities.rs` parses the printed record as evidence, and `build.rs` builds the carrier with `topo::section_near`. I did not verify that `section_near` reaches `plane_cone_section`'s anchored arm.

**Defects found off the question (by reading, nothing run)**
- `origin/oneround/anchored-conic`, `crates/step-import/src/signed_zero.rs`: `plus_zero_curve` destructures an ellipse with `..` and rebuilds it with `Ellipse::new`; it is called on the built carrier in `build.rs`.
- Same branch, `conic.rs`: `quarter_turned_where` and `seamed_toward` re-anchor from the derived centre.
- Rows worth filing: far-datum surfaces (axis, centre or apex far from the face); the fixed point's reliance on evaluate-invert-evaluate idempotence (IMPORTGATE's); `tangent_circle` building far-centred circles through the centre door.

**Not checked**
- That the chord form holds the admission floor for a general-phase anchor (modelled only).
- Why the carried-certificate rows moved, the 4 verdicts, whether the fixed-point row holds under A.
- The kind split of the lane's 60 sites, and whether any production circle mint is handed a far centre with no in-box data.
- That nothing public reads a carrier's centre: taken from the agent's statement on PR 4812.
- No cargo run. No `analysis/*` branch opened.

**The evaluation I have in mind (unmeasured)**
- `P(s) = P0 + 2·sin(s/2)·[−a·sin(θ₀+s/2)·u + b·cos(θ₀+s/2)·v]`, the two components orthogonal and each at most the chord; `sin(θ₀+s/2)` by angle addition from the stored pair.
- Not conjugate semi-diameter vectors as the stored datum: the minor semi-axis read back from their cross product loses about u·a/b. They are fine as a derived reading for the linear readers.

**Seams**
- PR 7131: under A a conic frame's origin is the anchor, so the conic reads of `frame.origin()` and `.center()` are renamed, not silently re-meant.
- `step-export/src/lib.rs`'s "Identity is meant literally … the two parameterizations agree" and the test `emitted_conic_carriers_equal_the_kernel_carriers_bitwise` need re-wording: the centre is derived, and a general-phase ellipse's parameter differs from the file's by its phase. Not a ratified page.
- The run-4 list of 22 is roughly the expected re-baseline, minus the rows that need an oracle or a re-derivation.
## For Ev

**Recommendation: one stored form for every conic, the anchored one. No conic is "born with a centre" any more, and nothing keeps a file's centre form.** (likely)

Every circle and ellipse would store a point on the arc its edge uses (the *anchor*) and that point's parameter θ₀. The *placement* would be a frame whose origin is the anchor, plus the principal directions and the semi-axes. A point is the anchor plus offsets, `P(θ) = P₀ + a·u·(cos θ − cos θ₀) + b·v·(sin θ − sin θ₀)`, with each difference written as a product of sines, so nothing cancels near θ₀. The centre is derived on demand: for export, and for the few readings that genuinely want it. Each *mint* (each kernel construction that produces a conic) computes its anchor in a well-conditioned way. Where the centre is near the arc (almost every circle), `c + r·u` is already fine. Where the centre can be far (shallow cylinder and cone cuts, large tangent circles), the mint works in the arc's own local frame, as the cone mint on the branch already does. There is one evaluator and no flag recording where a conic came from.

**Premise check: the "imported ellipse" half of the PR 4812 decision has nothing to apply to.** (likely)
- DESIGN `[import]`, step 2 (edge adoption), and `step-import`'s own pipeline doc say every cache is rebuilt from the surfaces by the kernel's own constructions, and "no curve or point the file printed is a cache". An imported ellipse edge's carrier is therefore minted by the same section construction as a native one. The file's `ELLIPSE` record is only evidence for picking the description.
- So "keeps the file's centre form exactly as read … round-trips bit-exactly" describes a state the import pipeline never produces. The round trip the tree actually promises is our own dialect's: the second export of a re-import is byte-identical to the first. That is weaker than "the file's bytes come back", and it was ratified with that difference stated.
- Your own words on PR 4812 were "import should re-pose in a sense, but ideally it would round trip nicely … a cached internal representation". The "cache", the per-origin split and "evaluators read the anchored form either way" were agent text, which you approved with "sounds good!". I read the PR's comments directly.

**Premise check: "born with a centre" is a property of a mint, not of a curve.** (sure)
In a body, every conic is a kernel mint: native, or rebuilt by adoption. So the question is not what to preserve from a source. It is whether each construction is well-posed where it puts its result. The ratified Float-rounding clause (geom-core README, ratified by you on PR 6329) already answers that: "a stored form that loses digits (a conic whose centre lies far from the arc it serves) is re-minted in a well-conditioned form". `[simple.conditioned]` says the same. The born-form build leaves every far-centred circle and every shallow cylinder cut in violation of that clause.

**Why the measured "only about 5×" undersells the fix.** (likely)
The ~5× compares the *original* centre-form curve with an anchor recomputed from it at every evaluation. Measured against that curve, no f64 computation can do much better than about u·far (u is f64's unit roundoff, about 1.1e-16; "far" is the centre's distance). And under PR 7131's reading of a frame as its polar factor, even an exactly computed sum is not exact. A stored anchor changes which curve the stored numbers describe: the minted curve moves once, by about u·far, which is the same order as any mint's own rounding. From then on its points near the arc hold to u·|P|, at f64 and at `Interval` alike. That is the unbounded gain, and only a stored datum gives it.

**Ratified text to change**
- The PR 4812 record (GERM row "Decided", and the unit's seam note). Retire "an imported ellipse keeps the file's centre form … as an internal cache … round-trips bit-exactly". Rewrite "evaluators read the anchored form either way" as "every conic is stored anchored on its arc; the centre is derived".
- "Parameter zero there" becomes "the anchor's parameter θ₀ is stored". The conic's parameter then stays the ISO one measured from the principal axis, so edge parameter intervals and export are untouched. The anchor may sit anywhere on the arc. A shallow cylinder cut has no principal vertex near the arc it uses (measured 0.42–0.50 rad away), so an anchor fixed at a vertex does not serve it.
- Float rounding and `[simple.conditioned]` need no change. This option is what they already say.

**The options, each as a final state**

1. **One anchored form for every conic (recommended).**
   - One evaluator, one stored point, and the centre never stored.
   - The frame's rounding multiplies only the offsets, which are small near the arc. That removes the at-rest frame check's amplification by the semi-axis, and the dependency on PR 7131.
   - Closes, at their source, the rows on far-centred conics: decisions against in-box points, chords, nested curves, the arc cover, and the missing re-seam door for circles.
   - Cost: every conic's points change in the last bits.
   - Reversible: internal; no public API changes.

2. **Evaluate each conic in the form it was born in (the branch as built).**
   - Every existing bit is kept.
   - The conic type stores an optional anchor and has two evaluators, chosen by where the curve came from, and the anchored case keeps the derived centre stored in its frame as well. So it is a type that means two things, with two points kept in step.
   - Every far centre-born mint stays at u·far, against the ratified Float-rounding clause. Choosing this means narrowing that clause.

3. **The clause as written: one evaluator, anchor recomputed from the stored centre on every read.**
   - Gains a constant factor at f64 and nothing at `Interval` (the enclosure is still u·far wide).
   - Moves every bit, as option 1 does, without option 1's gain.
   - Dominated by option 1.

4. **Keep the centre form, but evaluate it with exact (compensated) sums.**
   - Needs a product-with-residual operation on the scalar trait.
   - Gives nothing at `Interval`, and under PR 7131's polar-factor reading the sum is not exact anyway.
   - Dominated.

A defensible middle: option 1 only where the centre is far. This keeps the bits of near circles, but it brings back two forms and adds a threshold that some rule has to set. I lean against it.

**Which bit identities are contracts and which are habits**
- **Contracts (all kept under option 1):**
  - D2/D9 determinism: the same input gives the same bytes.
  - Our own dialect's STEP fixed point (second round trip byte-identical). Under one chain it became a 1-ulp two-cycle. The cause is the import step that places a free vertex at the foot of its printed point on the carrier: that step is not idempotent at the bit. The centre form hid this only at quarter turns. It should be fixed there, not by keeping centre forms. (likely)
  - "A carried certificate equals a fresh recertification." This is an identity a gate is pinned with. Its failure suggests vertices are being placed by centre arithmetic outside the conic's own evaluator: two readings of one point. Re-derive the gate; do not re-baseline it. (unsure)
- **Habits (regenerate):**
  - quarter-turn points equal to `c ± r` exactly;
  - the sphere support reading exactly 0.5;
  - mesh digests;
  - query counts;
  - bounds pinned against evaluated points.
  Per the standing rule, bit preservation is not the bar.
- **Owed before landing:**
  - The four verdicts that changed, each checked against an exact oracle. Two of them look like improvements. In phase 1, the analogous pair turned out to be true certifications.
  - The `KernelLimit` verdict is attributed to the re-fitted constant, not to the form.

**Worked example.** A cylinder of radius 1 cm, cut almost along its axis, gives an ellipse whose semi-major axis is 10 km.
- Centre form: every point near the model is good to about 2e-12 m, whichever way it is evaluated.
- Option 1: the mint places the anchor where the plane meets the cylinder inside the box, good to about 1e-18 m. Points within a few centimetres of the anchor are good to about 1e-18 m. Export derives the centre to about 2e-12 m, which is all a centre form can hold anyway.

**Confidence.** Recommendation: likely. Import never keeps a file's curve: likely (read in the docs, not traced end to end). Option 3 is dominated: sure. The θ₀-anchor spelling versus storing conjugate semi-diameters: unsure, since both are complete forms. I prefer θ₀ because it keeps the ISO parameter and the principal readings direct.

## For the orchestrator

- **Provenance.** The PR 4812 comments were read via GitHub. The clause is from agent comment 6132205976; Ev's own words are in 6132196517; Ev's approval is 6132885636. Float rounding's "re-minted" sentence was written in commit 31576a6c6 ("[ev] margin scale per Ev"); the companion table says Ratified (Ev, PR 6329). The checkout is shallow, but no conclusion here rests on an empty `git log`.
- **The import premise, worth a check before Ev reads it.** DESIGN `[import]` edge adoption ("no value the file printed becomes a cache") and the `crates/step-import/src/lib.rs` pipeline doc (step 4). `entities.rs` still builds `Curve3::ellipse`/`Circle::new` from `ELLIPSE`/`CIRCLE` records. I did not trace whether any of these survive into a body's carrier: for example a free edge, or a description chosen from the printed curve. If one does, that path contradicts `[import]` and is a defect in its own right. The evidence file's "Whether any import produces a far-centre ellipse was not surveyed" is the same gap.
- **Defects in the build relative to the decision.** File where they belong, probably as seam notes on the unit.
  - (a) `Circle::anchored`/`Ellipse::anchored` store the anchor *and* the derived centre as `frame.origin`: two points kept in step, against "centre derived, never stored".
  - (b) The anchor is fixed at the u_ref vertex (θ = 0). The decision says "on the arc its edge uses", and the phase-1 report says the cylinder cut needs a general point.
  - (c) `plane_cylinder_section` is a kernel-minted section, so the decision as ratified already covers it, yet it is left centre-born.
- **Off-question.**
  - The STEP fixed-point two-cycle points at the non-idempotent foot of a printed point on the carrier (`step-import` build/adopt).
  - The carried-versus-fresh certificate break (`crates/sweep/tests/common/certificates.rs`, `revert_plane_charts`, `snowman`) suggests vertex points minted outside the conic's evaluator.
  - Both need diagnosis before re-baselining. Neither is a reason to keep centre forms.
- **A wider class, not surveyed.** Far-centred *surfaces* (a sphere, cylinder or torus with a large radius, read near the model) have the same stored-form problem. The conic is one instance. A row for it may be worthwhile.
- **Interaction with PR 7131.** Under option 1 the frame's origin is the anchor, so 7131's polar-factor reading multiplies only the small offsets. The phase-1 report notes this too.
- **Assumptions.**
  - Representing tangent circles at ρ = 1500 m as user-reachable mints (not only fixtures) was not checked.
  - The 4 verdicts were taken from the evidence file as given.
  - No code was built or run.

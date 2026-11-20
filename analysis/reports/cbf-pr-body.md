**The decision: one stored form for every conic. Each circle and ellipse is stored anchored on the arc it serves, with its parameter zero at the anchor and its centre derived, never stored.** Both designers agree after one round; nothing is split. This revises your PR 4812 decision, so it is yours.

**Why it is back with you.** PR 4812's record had a minted section conic stored anchored, an imported ellipse keeping its file's centre form with an "anchored cache", and "evaluators read the anchored form either way". The lane building it measured that reading a centre-born conic from a zero point computed off its centre moves that conic's bits everywhere (22 rows, four of them verdicts) for a constant factor of about 5, and built it the other way: each conic read in the form it was born in. That leaves every far-centred circle, the cylinder's shallow cuts and anything authored exactly where main has them.

**What both designers found.**
1. **Nothing supplies a carrier, so there is no given form to keep.** `[edges]` makes the carrier a derived cache, and `[import]` (your PR 7074, five weeks after PR 4812) says no value the file printed becomes a cache; import rebuilds every carrier with the kernel's own section construction. The imported-ellipse half of the PR 4812 record describes a state the tree does not have.
2. **The clause was not your sentence.** Both read PR 4812's comments: "evaluators read the anchored form either way" was the agent's summary, and you answered "sounds good!". Your words were that import should re-pose but ideally round-trip nicely.
3. **The "only 5×" comes from keeping the centre as the truth.** With the anchor stored when the conic is made, an unmoved conic has the same bits as that measured variant, but its error near the arc is u times the distance from the anchor. That is the unbounded gain.

**What I am asking.** Approve the one stored form? It retires three things in the PR 4812 record: "stored in the form it was born in", the imported-ellipse sentence, and "evaluators read the anchored form either way". The diff rewrites that record (GERM's row and the unit's copy) and names the form in the geom-core README's "Float rounding"; the README sentence is a clarification and can be declined alone.
- Rejected: two forms, each read as born (what the lane built; its only reason is bit stability, and it leaves a far-centred centre form representable and silently lossy); the clause as written (every bit moves for a constant factor, nothing at `Interval`); a centre form with exact sums (a new scalar operation, nothing at `Interval`).

**What it costs, so you can weigh it.**
- Every conic's last bits move. Mesh digests, bit pins and query counts are regenerated.
- A STEP file prints a derived centre: a circle's printed centre can differ in the last digit from what a centre form would hold (a circle authored centred at 0.1 can print 0.09999999999999998). Your document is untouched; it keeps what you wrote.
- Parameter zero sits at the anchor, so an ellipse anchored off a principal vertex has a parameter that differs from its eccentric angle by a stored phase. About 60 production sites that pair the parameter with the principal directions read the principal angle from the conic instead. Keeping the parameter measured from the axis was weighed and dropped: an edge's stored ends would sit on a grid of 0.2 to 2 picometres along the arc at a semi-axis of 10 km, the term the PR 4861 re-pose removed.
- Two identities are re-derived, not re-baselined: "a carried certificate equals a fresh one", and the STEP fixed point, which rests on import's vertex placement being idempotent at the bit.
- Four verdicts moved under the lane's one-chain variant; each is checked against an exact oracle before anything lands.

**How I would stage it, unless you object.** First the unit in flight, reshaped: one positional datum per conic, every conic anchored at its seam vertex (a door handed a centre computes the anchor once), the cone section minted at its near vertex. Then the general anchor (the phase, the 60 sites, the cylinder's shallow cuts) as its own unit.

Option letters and numbers inside each section below are that designer's own. The measurements are on `analysis/oneround/conic-vertex-probes`; the fork's row is `docs/DESIGN-FORK-LOG.md` 294.

---

## A

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

---

## B

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

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw

## For Ev

**Recommendation (likely): change the sentence. A conic stores one point that lies inside the session box, plus where that point sits on the conic. The point is the centre when the mint is handed a centre in the box, and a point of the arc when the centre is far.**

Terms:
- *Session box*: no model coordinate lies past 1e3 m (`SESSION_BOX_EXTENT`).
- *Admission floor*: the rounding a run already allows for a short sum of in-box numbers, about 1.8e-12 m.
- *Box scalars*: `Interval` and the symbolic scalar, which carry a range or an expression, not one number.
- *u*: f64's unit roundoff, 1.1e-16.

**Premise check: three corrections**

1. **"Anchored" is not the well-conditioned form; "stored point in the box" is.** (likely)
   - A point summed from a centre in the box rounds at a few u times the box, which is about the admission floor. "Float rounding" already allows that.
   - The far-centre row says the same: "a reading on a conic whose stored terms stay in the box is unaffected". Its probe finds zero wrong answers at far = 1e3.
   - An anchor on the wrong part of the curve loses the same digits a far centre does. That is the lane's third finding (6.5e-13 against 1e-14).
   - A full circle is nearer its centre (r) than any anchor (up to 2r).
2. **The unsound cover is main's, not the new form's.** (sure on the arithmetic; not run in the tree)
   - In that fixture (semi-axis 1e7, centre 1e7 off) the derived centre equals the given centre exactly.
   - The cover rebuilds the arc's end as `centre − 1e7·cos θ`, which is 4.2e-10 m off the true point. Main's evaluator makes the identical error, so on main the two agree bit for bit and the test passes.
   - The anchored evaluator returns the true point, and that exposes the cover. I reproduced it: centre form gives 1.0124999824911356, anchored gives 1.0124999829140626, and the true value matches the anchored one.
   - This is the filed row `the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding…`.
3. **What the evidence points at is the readers, not the storage.** (likely)
   - `Conic::frame()` hands out a frame whose origin is the centre. About 110 production reads take that origin and rebuild `centre + size·direction` themselves: covers, chart images, the inverse, extents.
   - Each is a second statement of the conic, in the lossy form. The tree names this already: `a-conics-points-are-restated-in-centre-form-outside-its-evaluator`.
   - The ratified sentence says what is stored and is silent on what a reader reads.

**Why the centre must be storable: the box scalars** (sure on the arithmetic; the lane's ten rows are the measured side)

An authored circle's independent data are its centre and radius. Storing `centre + radius·direction` bakes the radius into the position.

Worked case: a hole and a boss sharing one centre, each radius boxed ±0.1 mm.
- Anchored: each centre reads back as `(c + u·r) − u·r`. Interval arithmetic cannot cancel that, so each centre carries ±0.2 mm and "concentric?" is undecidable over the box.
- Centre stored: both read the same `c`, and the question is decided exactly.
- At the symbolic scalar the given centre is a node, and `c + u·r − u·r` is a different expression (measured: `None` where `Some(Zero)`).
- The sweep tests state the guarantee this breaks: "over a parameter box a datum the geometry holds fixed is stored fixed".

No amount of reader work removes this. It is the form.

**The answers as final states**

*A. One point in the box, and where it sits (recommended).*
- Stored:
  - the point;
  - the axis and seam directions;
  - the sizes;
  - a pair giving the point's place in the conic's own unit coordinates: (0, 0) for the centre, (cos θ₀, sin θ₀) for a point of the curve.
- This is the pair you approved for the ellipse on PR 7194 ("where on it the anchor sits"), now allowed to be the centre. A circle gets it too.
- One evaluator: `P(θ) = point + u·a·(cos θ − ξ₀) + v·b·(sin θ − η₀)`. No tag, and nothing records how the conic was made.
- A constructor handed a centre stores it as given, with no arithmetic. It refuses loudly if that centre is past the session box. (The refusal is separable; likely.)
- A mint that can put the centre far computes a point of the arc and stores that, with parameter zero there. The cone section does this today. The shallow cylinder cut and far circles are owed it.
- What a reader reads:
  - *Directions* (axis, seam): stored, exact at every scalar, read without deriving any position.
  - *Where the curve is*: only through the conic's own readings from its stored point. `Cover::arc` takes the conic, not a centre frame. The chart image and the inverse are stated from the same point.
  - *The centre*: for a centre-stored conic it is the stored value itself (the same f64, the same enclosure, the same node). For an on-arc conic it is derived, good to u·a. Such a conic exists only because its centre is far, so a decision resting on it is one of the nine filed far-centre rows.
- Makes true: all three of the lane's findings go for conics made from a centre. A far-centred centre form cannot be built.
- Leaves possible:
  - A conic with its centre in the box and a semi-axis of box size keeps main's near-graze behaviour (typed refusals). The fix there is at the reader, by summing chords from a nearby point, which phase 1 measured at 40 of 40 with no stored form.
  - An on-arc conic whose stored point is far from the arc actually used.
- Reversible: yes, behind `Conic`.

*B. The sentence as written, with every reader re-posed on the anchor.* Defensible.
- Makes true: literally one kind of stored point, and every conic good to u times the distance from its anchor.
- Leaves:
  - the box-scalar widening above, for every authored circle;
  - an anchor at whichever vertex the seam happened to name;
  - a far centre still summed silently, once, at the door.

*C. Two forms with a `Born` tag (the step before).*
- Same numbers as A today.
- Two evaluators kept in step by hand, and a far centre form that stays representable.
- Dominated by A.

*D. Store both centre and anchor.* Two descriptions of one curve. No.

**Contracts and accidents**

- *A cover holds its arc.* A contract as an enclosure at `Interval`. At f64 it is a contract as real numbers, to in-box rounding. Bit-exact containment is an accident, and for a far centre it is false against the true arc on main.
- *A carried certificate equals a fresh one.* A contract (`[one-way.places]`), given the image and the carrier are stated from the same stored point. The two moved rows are an image stated from one datum and a carrier from another.
- *The STEP fixed point.* An accident: evaluate-then-invert happens to be idempotent in centre form. It is import's to provide.
- *An exact centre for a circle authored at one.* A contract at the box scalars. At f64 it is free under A, and "not a hard requirement" in your words.
- *Last bits, digests, counts.* Only "same inputs, same bits".

**Ratified text I would change**

"Float rounding", `crates/geom-core/README.md`. The sentence was written by an agent and approved by you on PR 7194 ("ok i see, i think this plan works, then"), after you asked whether it keeps the given form and how a centre form is lossier. You were told two forms' only reason was bit stability, and none of the three measurements above existed then.

Proposed: "A stored form that loses digits is re-minted in a well-conditioned form: a conic is stored about one point inside the session box, its centre where that lies in the box and otherwise a point of the arc it serves with its parameter zero there, together with where that point sits on the conic; no reader states a conic's points except from that point."

This is close to the two-form state you were told both designers rejected. What differs is one evaluator, no tag, and the far centre refused at the door, so "how accurate a conic is depends on how it was made, and nothing says which" no longer holds.

**Confidence**
- Recommendation: likely.
- In-box centre form within the floor: likely (worst corner about 15 u·box against a floor of 16).
- The cover correction: sure.
- The box-scalar widening is structural: sure.
- One branch-free formula serving both stored points at every scalar: unsure (the symbolic scalar is not checked).

## For the orchestrator

**Errors in the brief**
- **Item 2's headline instance is misattributed.** In `a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size` the anchor sum and the derived centre are both exact (anchor z = 0, derived centre z = 1e7). The 4.229e-10 is `Cover::arc`'s own centre-form rounding of the arc's end. Main's evaluator rounds identically, which is the only reason main's row passes. I ran this in f64 Python against mpmath. I did not run cargo.
- **Item 2 is a mix, not one class.** `edge_extent_bounds` compares a real-number bound against evaluated points bitwise. The `−7` foot row decides "on the axis" by exact zero. Only some rows are the derived centre.
- **"About 90 sites."** A grep of production files on `e04a87227` (test, row, probe and fixture files excluded by name) finds:
  - 109 `frame().origin(` reads on conic-like receivers;
  - 112 direction-only reads (`axis`, `u_ref`, `v_ref`) through the same `frame()`, each of which now derives a centre it discards;
  - some `.center()` reads; that grep also catches balls, so I did not count it.
- **"Parameter zero at the anchor" is not what `e04a87227` builds for a re-seamed ellipse.** The parameter stays the anomaly from `u_ref` and the phase says where the anchor is. That is consistent with the staging in PR 7194's body, but the README sentence reads as if it were already true.

**What I assumed or could not check**
- Nothing was run in the tree. The step-B counts are taken from the evidence README as given.
- The symbolic scalar's folding of `0·x` and `x − 0` is not checked. If it does not fold, the (0, 0) case needs its own spelling inside the one evaluator (a private two-arm). That weakens "one evaluator" to "one type, one door per reading". `inari`'s interval product and difference with an exact zero are exact as far as I know; I did not test it.
- At (0, 0), step A's offset `−(2·sin²(θ/2) − 1)` is not bit-equal to `cos θ`. So A as I describe it still moves last bits for centre-stored conics, unless the lane spells that case as `cos θ`. I did not weigh this (bit stability), but it sets the size of the re-baseline.
- The centre door's refusal past the session box:
  - I did not survey which production mints trip it. Phase 1 (b) names `plane_cylinder_section` and `plane_cone_section`. `a-far-circle-has-no-re-seaming-door` suggests far circles too (a plane through a far-centred sphere, cone latitudes).
  - The exact threshold (the box, or a small multiple) and typed-or-panic are the lane's.
  - The worst-corner arithmetic (centre and arc both in the box give a ≤ 2√3·box) is mine, by hand.
- The near-gap families at ρ ≈ 1500 have their centre past the 1e3 box. Under A their mints go to the on-arc door and keep the 40 of 40. A centre in the box with a semi-axis near box size keeps main's typed refusals.
- I read PR 7131's body. It bears on this only in that a frame's one-ulp width at `Interval` multiplies the lever from the stored point. That is at most a box diameter under A, and it is the semi-axis only for a far centre, which A does not allow.
- A is close to `3193e7c97` in behaviour (the README says 17,465 of 17,466 pass there). The differences are the type (the pair in place of `Born` and its roughly twenty `match` arms), the door refusal, and the reader rule.

**Defects off the question**
- `Conic::frame()` conflates directions with the centre. Under any answer, a direction read should not derive a position. I would split it into a directions reading and named position readings. It belongs with `a-conics-points-are-restated-in-centre-form-outside-its-evaluator`, whose three sites are a subset of the ~110.
- `Cover::arc(frame, semis, window)` is itself a centre-form copy of the conic. The filed arc-cover row should be read as "the cover takes the conic", not as a rounding repair.
- `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face` is the same question one level up. Under A a latitude circle shares its surface's axis datum, so whatever that row decides for the surface carries to its circles through the same two doors.

**Housekeeping**
- The branch listing showed `analysis/design-fork/conic-centre-readers`. I did not open it.
- I made and removed one detached worktree (`/root/.local/share/cad-work/designer-59`). Nothing in `/home/user/cad` was edited or committed.
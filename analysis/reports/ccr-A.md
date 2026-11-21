## For Ev

**Recommendation (likely): change the sentence, and keep its purpose.** A conic keeps one numeric form and one evaluator, but its stored point depends on the conic's size, not on how it was made:
- **No semi-axis longer than the session box (1 km):** it stores its **centre**, bit for bit as handed.
- **Bigger than that:** it stores a **point on the arc it serves**, and its centre is derived.

The far-centre loss is removed exactly where it occurs. The cost the lane measured, which falls on every ordinary conic, goes away.

**Terms.**
- *Stored point* `O`: the one position the conic holds.
- *Placement* `k = (k_u, k_v)`: where `O` sits in the conic's unit coordinates. `(0, 0)` means `O` is the centre. `(cos θ₀, sin θ₀)` means `O` is the curve's point at anomaly θ₀. This is the "phase" pair step A already stores, with `(0, 0)` now allowed.
- *The evaluator, for every conic*: `P(θ) = O + u·a·(cos θ − k_u) + v·b·(sin θ − k_v)`. The centre reads as `O − u·a·k_u − v·b·k_v`, which is `O` itself, exactly, where `k = 0`.

**Premise check (three corrections)**
1. **Far-centre loss happens only when the conic is bigger than the box.** (likely)
   - Every point of a conic lies within its larger semi-axis `a` of its centre. So with `a` within the box, a centre-form point is a short chain of operations on in-box coordinates, and its rounding is inside the run's admission floor (about 1.8e-12 m). `[simple.conditioned]` accepts that floor, and no site charges it.
   - Every loss measured so far is on a conic bigger than the box: the near-gap families at radius 1500 m, the far sections at 1e5–1e7, and the 1e4 ellipse of the minor-vertex row.
   - The original row says it too: "a reading on a conic whose stored terms stay in the box is unaffected".
   - So "a stored form that loses digits" describes a large conic stored at its centre, not every conic stored at its centre.
2. **The 45 moved tests show two different things, and only one of them is about the form.**
   - (i) *Costs on conics that lost nothing.* Interval and symbolic widening, and the `f64` gap between the derived centre and the curve, on radii of 0.5–2 m (group 1, most of group 2). The anchored form buys nothing there and charges every reader that wants the centre.
   - (ii) *Code that writes the curve out again from its centre, outside the conic.*
     - The arc cover: `Cover::arc` keeps its own centre-form arc.
     - The edge's chart image (`Pcurve::Harmonic`'s `Free { c, a, b }`).
     - `param_near`, and the extent bounds.
     - These agree with the evaluator today only because both happen to sum from the centre. That is a defect under any stored form. The unsound 1e7 cover belongs here: it is a large conic, and its cover already misses on main in kind (filed as the far-arc-cover row).
3. **The sentence does not describe what was built either.** "Its parameter zero at the anchor": step A keeps the parameter as the angle from the principal axis (`u_ref`) and stores the anchor's angle. So after a seam turn, parameter zero is no longer at the anchor. For a large conic anchored away from a vertex (the cylinder's shallow cut), the parameter question the PR 7194 designers argued is still unbuilt. (sure)

**The answers as final states**

*B. Stored point chosen by size (recommended).*
- **What it makes true:**
  - An authored or in-box conic keeps its centre exactly at `f64`, at `Interval` (a point interval) and at the symbolic scalar. "Concentric" and "on the cylinder's axis" stay exact Zero verdicts.
  - STEP prints the centre you wrote (0.1 stays 0.1).
  - Main's bits come back for every ordinary conic.
  - A large conic is anchored on its arc, so it reads to u times the distance from its anchor.
  - Accuracy depends on the curve's size, never on its history. That was the reason two-forms-as-born was rejected, and it does not apply here.
- **What a constructor handed a centre does:**
  - Within the box, it stores that centre.
  - Past the box, it must also be told which part of the conic it serves (a parameter window or a point). It anchors there with that point's anomaly, so it never falls back to the `u_ref` vertex (group 3).
  - A whole conic bigger than the box cannot lie in the box, so asking for one is a typed refusal.
  - Result: a large conic stored at its centre cannot be built.
- **What readers read:**
  - Anything that writes out the curve itself (a cover, an extent, a chart image, the inverse): it reads `O` and `k` through the conic's one evaluator, never `center() + size·direction`. That makes it agree with the evaluator by construction at every size and scalar.
  - Anything that needs the centre as a datum (a relation to a sphere's centre or a cylinder's axis, the Reach rule's centre datum): it reads `center()`.
    - Within the box, that is exact at every scalar.
    - Past it: at `f64` the centre is rounded at u·a; at `Interval` it is an enclosure of the true centre, about ulp(a) wide; at the symbolic scalar it is the derived expression.
    - Such a verdict on a large conic is levered by `a` and should read a nearer datum. The Reach page already allows any point, provided the datum and its lever are read at the same one.
- **What it leaves possible:**
  - A threshold. Across it the stored form and the last bits jump, but both sides are sound.
  - Two value shapes for `k` inside one type: a family of representations behind the shared `Conic` interface, which you allowed on PR 6329.
- **Reversible:** yes. Setting the threshold to zero gives A.

*A. As ratified: every conic anchored, the centre always derived.*
- **What it makes true:** one value shape, and a large conic stored at its centre cannot exist without any check.
- **What it costs, on every conic:**
  - At `f64` an authored centre comes back as `fl(fl(c + u·a) − u·a)`, not `c`.
  - At `Interval` a centre given as a range gains the semi-axis twice. About ten certified builds become refusals.
  - The symbolic scalar loses `c + u·r − u·r = c`.
  - About 90 sites that read a conic's centre have to be re-posed or charged.
- **What it gains over B on in-box conics:** precision below the admission floor, which the project's own rule declines to charge. (likely)

*C. Anchored, plus the centre stored or cached, or the anchor's rounding kept as an exact residual.*
- The first is two descriptions of one curve kept in step by hand.
- The residual form gives nothing at `Interval`.
- Rejected.

*D. Two forms, each read as it was born.*
- Rejected again: a large conic stored at its centre stays representable.

**Which guarantees are contracts and which are accidents (under B)**
- **A cover holds its arc. Contract** (soundness). Today it holds by the two chains agreeing to the bit, which is an accident. The cover has to be built from the conic's own `O` and `k`.
- **A carried certificate equals a fresh one. Contract on the door**, under `[one-way.places]`: a door may carry a certificate only if it keeps the support bit-identical. The residual values themselves are accidents. Under B, ordinary conics behave as on main. On large conics, the chart image must read the conic's form, so that a reversal or a move is exact. (unsure; the mechanism is not traced)
- **The STEP round trip's fixed point. Contract**, from your "ideally it would round trip nicely". Its mechanism (`param_near` agreeing bit for bit with eval) is an accident. B keeps it for ordinary conics. For large ones `param_near` has to read from the anchor, as `Circle::azimuth` already does. (likely)
- **An exact centre for a circle authored at one. Contract** under B for every in-box conic, at every scalar. Under A it is lost.
- **Quarter-turn identities, mesh digests, query counts: accidents.** Regenerate them.

**Ratified text I would change** (geom-core README, "Float rounding"; agent text you approved on PR 7194 before these measurements existed). Replace "a conic is stored anchored on the arc it serves, its parameter zero at the anchor and its centre derived, never stored" with:

> "a conic stores one point of its plane: its centre where no semi-axis exceeds the session box, and otherwise a point of the arc it serves, where it is anchored; every other position is derived from that point, and nothing restates the curve outside its evaluator."

The parameter clause goes to the unit that builds the general anchor, as its own question.

**Worked example.**
- A circle you author at centre (0.1, 0, 0), radius 0.5: under B it stores (0.1, 0, 0). Every reader at every scalar gets 0.1. STEP prints 0.1. The symbolic scalar sees the same expression you wrote.
- A shallow cut of a cylinder giving an ellipse with a = 1e7: it stores a point in the middle of the cut arc, with that point's anomaly. Its cover is built from that point, and at the 1e7 row that cover holds the arc's own point.

**Confidence.**
- Recommendation: likely.
- Loss occurs only past the box: likely (from the floor's definition and every measurement; no in-box probe at ε 1e-12).
- The restating sites are a defect under any form: sure.
- A costs every ordinary conic: sure (measured).

## For the orchestrator

- **Owed probe before Ev answers:** run the near-gap families at radius 900 m, in centre form, at ε 1e-12. If they refuse there, the size threshold is wrong and the case for A strengthens. If they certify, B's premise holds.
- **Threshold choice:** I used `SESSION_BOX_EXTENT` because it is a constant and does not change with a move. The rounding reach `R` depends on ε, which would make a carrier's stored form depend on the run's tolerance; I rejected it for that.
  - A sharper criterion exists: the distance from the stored point to the arc served. The 1e4 minor-vertex ellipse is fine in centre form once re-seamed (1e-14), with its centre only 1 m from the arc. That criterion depends on the arc, though, and the conic carries none.
  - The geom-core README says the box is "never a carrier's stored terms". A test on size is not a test of the box against the carrier's terms, but a reviewer may read it that way.
- **Ratified-text contradiction:** "parameter zero at the anchor" (fork row 294) against step A's stored-phase design. The brief's "already settled" covers the phase, not this clause. Check whether Ev knows the parameter clause is unbuilt.
- **Brief check:** the 1e7 cover and the 1e4 minor-vertex rows are large conics. The extent row (r 2), the sphere row (0.5), the certificate rows (r 0.5 and 1), the STEP row (`box_fillet_corner`) and group 1 are in-box, so they are pure costs of deriving the centre. The brief's grouping hides that split.
- **Where class (ii) lives:**
  - `crates/geom/src/cover.rs`: `Cover::arc` stores a centre-form `Arc` piece (frame, semi-axes, window). That is a second statement of the conic and the root of the cover defect.
  - `crates/geom-brep/src/pcurve_cache.rs`: `Channel::Free { c, a, b, l }`.
  - `Curve3::param_near` in `crates/geom/src/curves.rs`.
  - The `Conic` trait's default `unit_in_plane`, `along` and `semi_axis_vectors` (in `crates/geom/src/curves/conic.rs`) are centre-form readings built into the trait. Each needs an anchored counterpart.
  - Filed already: `work/oneround/a-conics-points-are-restated-in-centre-form-outside-its-evaluator.md` and `the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it.md`.
- **Reach rule:** the geom-core README says "a datum fixed by a carrier's form (an apex, a centre) is read there". On a large conic that gives a lever of length `a`. That probably needs a revision (read the anchor) under either A or B. Not filed.
- **Not checked:**
  - Whether the symbolic scalar simplifies `x·0` to zero. If it does not, `center()` at `k = 0` should return `O` structurally.
  - Whether every door that mints a large conic knows the part it serves. The section tables and sketch arcs do; I did not check revolve rims or import.
  - The effect on the quadric issue (`a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`). B's rule (datum by size) carries over to surfaces naturally. That is reasoning, not measurement.
- **Rough count:** a grep of `e04a87227` finds 270 production hits for `.center()` or `frame().origin()`, all carrier kinds together. I did not verify the lane's 90.
- **Read-only:** I did not run cargo, edit or commit. Fetches of `oneround/anchored-conic` and `onehome/carrier-frames` failed on a ref lock (other agents share this checkout); I read the existing refs and the commits named in the brief instead. PR 7194's comments were read through the REST API.

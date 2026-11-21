## For Ev

**Recommendation (likely): reword the ratified sentence. Every coordinate a conic stores lies in the session box (1 km, no coordinate larger).**
- **Centre in the box:** the conic stores its centre, exactly as it was handed, and is seamed at the principal vertex nearest the arc it serves.
- **Centre outside the box:** the conic is stored at a point of the arc it serves, its *anchor*. Its parameter is zero there, and the centre is derived.
- **Readers:** none states the curve any other way than from that one stored point.

**Terms.**
- *Admission floor*: the rounding a run already allows for a short sum of in-box numbers, about 1.8e-12 m.
- *Box scalars*: `Interval`, which carries a range, and the symbolic scalar, which carries an expression.
- *u*: the unit roundoff of `f64`, 1.1e-16.

**Premise check (four corrections)**
1. **Lossy only far: the centre form loses digits only when its centre lies outside the box.** (likely)
   - Take a centre in the box, an arc in the box, and the seam at the vertex nearest the arc. Then every term of `centre + u·a·cos θ + v·b·sin θ` is at most box-sized, even on a very long ellipse, because the arc must then sit near the minor vertex.
   - The rounding of that sum is a short in-box chain, which is what the admission floor already allows. A corner-case estimate gives about 15 u·box against a floor of 16.
   - Every measured loss has its centre far outside the box: the near-gap families at 1500 m, the sections at 1e5–1e7. The original row says "a conic whose stored terms stay in the box is unaffected".
   - The rule also follows from a clause you already ratified: "the session box is 1 km, no coordinate larger" (`[tolerance.units]`). A far centre is a stored coordinate outside the box.
2. **Storing the centre is needed, not merely harmless, at the box scalars.** (sure)
   - A circle authored by centre and radius has those two as its independent data. If the radius is folded into a stored anchor, the centre comes back as `(c + u·r) − u·r`, and interval arithmetic cannot cancel the two `u·r` terms.
   - Example: a hole and a boss share one centre, and each radius is a ±0.1 mm range.
     - Stored at their centres, "concentric?" is decided exactly.
     - Anchored, each centre carries ±0.2 mm and the question cannot be decided.
   - That is the lane's ten `Interval` and symbolic refusals. No change to the readers removes it.
3. **The unsound cover on the 1e7 ellipse is main's defect, not the new form's.** (likely; computed outside the tree, not run in it)
   - In that fixture the derived centre equals the given one exactly.
   - The cover rebuilds the arc from the centre (`Cover::arc` holds its own centre-form copy of the conic) and misses by 4.2e-10 m.
   - Main's evaluator makes the same rounding error, so the test passes there by bit agreement. The anchored evaluator is right and exposes the miss.
   - The fix is that the cover takes the conic itself.
4. **"Parameter zero at the anchor" is not what was built.** (sure)
   - The step built before this one keeps the parameter as the angle from the seam axis and stores where the anchor sits. After a quarter-turn re-seam the anchor sits at about π/2. There, representable parameter values are about u·a apart along the curve.
   - For a conic stored at its arc, the parameter must start at the anchor, and such a conic is never re-seamed. That keeps the clause, now true of exactly the conics that are anchored.

**What is stored, what a door does, what a reader reads**
- **Stored:** the axis and seam directions, the sizes, one point, and where that point sits in the conic's unit coordinates: (0, 0) for the centre, (cos θ₀, sin θ₀) for the anchor. One evaluator serves both: `P(θ) = point + u·a·(cos θ − k_u) + v·b·(sin θ − k_v)`. Nothing records how the conic was made.
- **A door handed a centre in the box** stores it with no arithmetic.
- **A door handed a centre outside the box refuses loudly.** Summing `c + u·a` from a far centre loses the digits once, silently: the measured sections miss their own surfaces by 1–3.6 u·a. So a conic whose centre is far is made only by an *on-arc door*, which computes the anchor from local data at the arc. The cone section does this today; the shallow cylinder cut, far circles and large sketch arcs are owed one.
- **A reader that needs directions** reads them stored, exact at every scalar, and derives no position. Today `Conic::frame()` makes about 110 direction-only reads derive a centre and discard it.
- **A reader that needs where the curve is** (a cover, an extent, a chart image, the inverse) asks the conic, which computes from its stored point. It never computes `centre + size·direction` itself.
- **A reader that needs the centre:**
  - On a centre-stored conic, it gets the stored value: the same `f64`, the same enclosure, the same symbolic node.
  - On an anchored conic, it gets a derived centre, good to about u·a. Such a conic exists only because its centre is far, so a verdict resting on that centre is far-levered and should read a nearer datum.

**Contracts and accidents**
- **A cover holds its arc. Contract.** At `Interval` as an enclosure; at `f64` in real numbers, to in-box rounding. Containment to the bit is an accident; for a far centre it is false on main.
- **A carried certificate equals a fresh one. Contract** (`[one-way.places]`). It holds when the chart image and the carrier are both stated from the one stored point.
- **The STEP round trip's fixed point.** A contract of import; that it holds today is an accident of the centre form. In-box conics keep it, because the printed centre is the stored one.
- **An exact centre for a circle authored at one.** A contract at the box scalars, where certified builds rest on it. At `f64` it comes free.
- **Mesh digests, query counts, last bits.** Habits. Only "same inputs, same bits" binds.

**The other answers**
- *As ratified, every conic anchored, every reader re-posed* (defensible, not preferred).
  - For: one kind of stored point.
  - Against: it loses exact centres and widens the box scalars for every authored conic. For in-box conics it gains only accuracy below the floor, which no admitted verdict reads.
- *Two forms tagged by how the conic was made* (the earlier step). It has two evaluators, and a far centre form stays representable.
- *Store both centre and anchor.* Two descriptions of one curve, kept in step by hand.

**Sentence I would write** ("Float rounding", `crates/geom-core/README.md`; text an agent wrote, which you approved on PR 7194 before any of these measurements existed):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores only in-box coordinates, its centre where that lies in the box and otherwise a point of the arc it serves, its parameter zero there; nothing states a conic's points except from that stored point."

This is close to the "two forms" you were told was rejected. What differs:
- one evaluator, and no record of how a conic was made;
- the form is chosen by where the centre lies;
- a far centre form cannot be built.

So "accuracy depends on how it was made, and nothing says which" no longer holds.

**Confidence.**
- Recommendation: likely.
- The in-box centre form stays within the floor: likely. A size measurement is pending.
- The box-scalar cost is structural: sure.
- The cover is main's own defect: likely.
- Reversible: yes; everything sits behind the `Conic` interface.

## For the orchestrator

**What moved me, and to what**
- **(1) The criterion: I moved from size to where the centre lies.**
  - My earlier rule was no semi-axis longer than the box. The other rule (centre in the box, plus `seamed_near`) also covers centre-in-box long ellipses, like the 1e4 minor-vertex row, which hold 1e-14 in centre form once re-seamed. It is also a consequence of `[tolerance.units]` rather than a new threshold.
  - My motion-invariance argument for size is only partly answered. A map that carries a centre out of the box while the arc stays in must re-mint at the arc, or the refusal threshold needs slack (a small multiple of the box). I left that choice to the lane, as one of the other reports does.
  - The geom-core README's "the session box … never a carrier's stored terms" reads against this rule. It needs a word that the box rule here is about conditioning, not the region predicate.
- **(3) A door handed a far centre: I moved from "anchor at the served part" to "refuse; far conics come only from on-arc doors".**
  - Phase 1 (b) shows that the single far sum at the door is itself the loss: sections miss their surfaces by u·a.
  - My earlier "rigid shift once, harmless" was wrong.
- **(4) The cover: I moved to "main's own".**
  - One report's arithmetic (exact derived centre in that fixture, a 4.229e-10 miss in `Cover::arc`'s own centre-form chain) is specific and checked against mpmath. Mine only said "in kind".
- **(2) The parameter: I moved to "keep zero at the anchor" for anchored conics.**
  - I had deferred it. Under the box rule, anchored conics are exactly the far ones, where the grid argument binds, and they never re-seam.
- **Also adopted:** splitting `Conic::frame()` into a directions reading and named position readings; and the concentric hole-and-boss case as the structural argument for storing centres.

**My earlier arguments**
- The admission-floor argument and the "restating sites" class are shared by all three reports.
- My generalised placement pair, (0, 0) or (cos θ₀, sin θ₀), was proposed by one other report too.
- My size threshold is answered by the position rule. My "the constructor anchors at the served part" is answered by the far-sum-at-the-door evidence.

**Where differences remain**
- **(5) Why the symbolic row lost its zero.** The brief's "`c + u·r − u·r` is not recognised as `c`" conflicts with `geom-core/src/sym.rs`, whose polynomial normal form cancels linear terms. The payload shows `frozen: 8`, which suggests forms froze on coefficients, or that the opaque `sin(θ/2)` atom did not match the rim's `cos θ`.
  - I lean to freezing (unsure).
  - It does not change the recommendation: under the box rule the rim conic stores its centre and the identity returns either way.
  - It does matter for one detail: at (0, 0), the step-A offset `−(2·sin²(θ/2) − 1)` is a different atom from `cos θ`. The centre-stored case should be spelled as `cos θ` inside the one evaluator, or symbolic identities on centre-stored rims may still freeze. One probe settles it.
- **(6) How sure the floor claim is: likely, not sure.**
  - It is reasoned, plus one corner computation by another report (15 against 16 u·box), with no in-box measurement at ε 1e-12.
  - The pending size measurement (ρ from 10 m to 1e4 m, at ε 1e-9 and 1e-12) would change my answer if, with the centre in the box (ρ up to about 1e3), the centre form gives **wrong definite answers**, or refuses where an anchored form certifies, at either ε. Then the floor does not cover it, and the answer moves towards anchoring everything (accepting the box-scalar cost) or a threshold well inside the box.
  - Refusals only at ε 1e-12 near ρ ≈ 1e3 would not change it: that is the admission rule's accepted cost.
  - A clean result to 1e3 that breaks past it confirms the rule.

**Not checked**
- Which production mints the far-centre refusal trips: the sections, far sphere and cone latitudes, large sketch arcs.
- Whether `inari` products with an exact zero stay exact.
- The ~110 direction-only read count; I took it from another report.

**Off the question**
- The quadric far-datum row, `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`, is the same rule one level up. Latitude circles inherit the surface's datum, so that row should be weighed next with the same criterion.
- The Reach clause "a datum fixed by a carrier's form (an apex, a centre) is read there" needs, for anchored conics, "read at the stored point". Not filed.

**Housekeeping:** I worked read-only, ran no cargo, and edited nothing.

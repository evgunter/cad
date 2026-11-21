## For Ev

**Recommendation (likely): reword the ratified sentence so the rule is about where a conic's stored point lies, not about one form.** A conic stores only in-box coordinates.
- **Centre inside the 1 km session box:** the conic stores its centre, exactly as handed.
- **Centre outside the box:** the conic is stored anchored on the arc it serves, parameter zero at the anchor, with the centre derived.
- **Readers:** every reader states the arc from the stored point, through one `Conic` reading. No reader takes a derived far centre as a coordinate.

Proposed wording for "Float rounding": *"A stored form that loses digits is re-minted in a well-conditioned form. A conic stores only in-box coordinates (`[tolerance.units]`): one whose centre lies in the box stores it; one whose centre lies outside is stored anchored on the arc it serves, its parameter zero at the anchor and its centre derived, never stored. Every reading of where a conic is starts from its stored point."*

**Premise check: four corrections**
1. **What was measured is not the state the sentence describes.** (sure, by reading)
   - The sentence says "anchored on the arc it serves". The build anchors every conic handed a centre at its `u_ref` vertex, by summing `c + u·a` from the centre it was given.
   - In production, only one door anchors on an arc: the cone section in `geom_brep::intersect`. About 87 `Circle::new` calls, and many more `Ellipse::new` calls, hand a centre.
   - So the measurement shows the cost of moving the far sum from the evaluator into the anchor and into every reader of the centre. It shows almost none of the gain.
2. **Every measured far-centre loss has its centre outside the box.** (sure on the cases cited)
   - The near-gap family that motivated the change has ρ = 1500 m. The unsound cover is on an ellipse whose centre is 1e7 m out.
   - `[tolerance.units]` (D4 ¶4, ratified) says: "the session box is 1 km, no coordinate larger in magnitude; geometry outside the range is rejected at construction". A far centre is a stored coordinate that rule already excludes.
   - With the centre and the arc both in the box, every term of `c + u·a·cos θ + v·b·sin θ` is of box size. Its rounding is a short chain on in-box values, which is what the admission floor already covers (16·u·1 km ≈ 1.8e-12 m). (likely: this bound is reasoned, not measured)
   - On PR 7194 you asked how the centre form is lossier, and the answer recorded was "only where the centre is far from the arc". This rule makes that answer the code.
3. **Deriving the centre does not remove the far-centre loss. It moves the loss into every reader.** (sure)
   - `Cover::arc` holds a frame at the centre, so a cover is built from `fl(anchor − u·a)`. That is the far sum again, now inside a containment.
   - This is the source of the unsound cover (a cover that misses its own arc's point by 4.2e-10 m).
   - The same reader runs on today's anchored cone sections on the step-A branch (likely; found by reading, not run).
   - So whichever form is chosen, the readers must read from the stored point. That part is not optional.
4. **At the symbolic scalar the stated cause is doubtful.** (unsure)
   - The symbolic tier works on a polynomial normal form, in which `c + u·r − u·r` cancels.
   - The failing row reports `frozen: 8`. The likelier causes are that the anchored evaluator's `sin²(θ/2)` is a different opaque atom from the rim's `cos θ`, or that the forms freeze.
   - The fix differs, so this should be checked.

**Ratified text.** The sentence was written by an agent in `bec0550b1` (the `[ev]` PR 7194). You approved it with "ok i see, i think this plan works, then", after noting that keeping an authored conic's given form exactly is "not a hard requirement". Two things in it would change:
- "a conic is stored anchored" becomes the box rule above;
- "derived, never stored" applies to far conics only.

"Parameter zero at the anchor" stays.

**The answers as final states**

*B. Stored point by the box rule; readers from the stored point (recommended, likely).*
- **What is stored:**
  - the frame's directions and the sizes;
  - one point: the centre if it lies in the box, otherwise the anchor on the arc;
  - for an anchored ellipse, the anchor's phase.
- **The form is a fact of the geometry, not of how the conic was made.** Each mint and each map decides it. Near the box edge either form is well-conditioned, so the choice is not a verdict and a near-edge misread costs nothing.
- **One reading shape.** Every point is the stored point plus an offset: `u·a·cos θ + v·b·sin θ` from a centre, or `u·a·(cos θ − cos θ₀) + v·b·(sin θ − sin θ₀)` from an anchor.
  - The `Conic` trait exposes the stored point and the offset at θ.
  - `Cover::arc`, the extent bounds, chart images and `param_near` read those. About 90 centre-reading sites move to it, by the lane's count (unverified).
- **What readers get:**

  | | `f64` | `Interval` | symbolic |
  |---|---|---|---|
  | In-box conic | exactly what the evaluator draws, as on main | as wide as on main | identities as on main |
  | Far conic | the arc at its own scale, read from the anchor | the same | the same |

  For a far conic, `center()` is a named lossy reading. Its users are export, and pose decisions against a surface whose own axis is equally far.
- **A door handed a centre:** in the box, it stores the centre. Outside the box, it fails loudly. Far conics are minted only by anchored doors, which compute the anchor from the surface's local data at the arc.
- **Consequences:**
  - in-box conics keep main's bits;
  - the ten or so `Interval`/symbolic rows and the in-box `f64` bit agreements return;
  - the minor-vertex row is answered by the existing re-seam rule (`seamed_near`), which is exact in the centre form;
  - the unsound cover is impossible by construction: a far centre is never a coordinate any reader starts from.
- **Leaves possible:** two arms in the placement and the offset, each needing a test. A map that carries a centre out of the box must re-mint at the arc, so the map door needs to know the edge's arc (one carrier per edge already gives it that).
- **Reversible:** yes. The choice sits behind the `Conic` readings.

*A. One anchored form for every conic, done fully (defensible, not preferred).*
- Every door that hands a centre also hands the arc, and anchors on it. Readers read from the anchor, as in B.
- It has one arm instead of two.
- **What it costs, measured:**
  - an exact centre is gone for every conic, including authored in-box ones;
  - at `Interval`, a conic made from a centre carries the anchor's width `w(c) + w(u·a)` everywhere, and up to `w(c) + 3·w(u·a)` on the far side, against main's `w(c) + w(u·a)`. A frame's entries are about one ulp wide at `Interval` since PR 7131.
- **What it gains for in-box conics:** accuracy below the admission floor (about u·|P|·θ² instead of u·box), which no admitted verdict reads.
- So for in-box conics B dominates A: equal gains, lower cost. (likely)
- The near-gap gains (7 → 40 certified) are all on far-centred ρ = 1500 m conics, which B anchors too.

*C. The build as is, or step A's "by how it was made" (rejected, sure).*
- As built: it moves the far sum into readers, ships an unsound cover, and refuses certified `Interval` builds.
- Step A: the form records how a conic was made, and its anchored conics still feed far centres to the readers.

**What the tree gets today by bit agreement**
- **A cover holds its arc: a contract.** At `f64` it holds up to the admission floor; at `Interval` it holds exactly.
  - The 4.2e-10 m miss breaks it.
  - The one-ulp diameter miss on a closed circle (sup 4 against 4.000000000000001) is within the floor. Pinning that miss is a habit, not the contract.
- **A carried certificate equals a fresh one: a contract.** It follows from determinism, and from every map acting on the stored data rather than re-deriving it. The two rows that broke point at a revert that rebuilds a conic through its derived centre (likely, not traced).
- **The STEP round trip's fixed point: import's contract.** B gives it for in-box conics, because the printed centre is the stored one.
- **An exact centre for a circle authored at one: not a contract in itself.** It is how an in-box conic's centre stays a coordinate every reader can read safely. Under B it follows from the rule.
- **Mesh digests, query counts, and the scalars the data walk visits: habits.**

**Confidence:**
- recommendation B: likely;
- premise corrections 1–3: sure;
- that the in-box loss stays within the admission floor: likely;
- the symbolic diagnosis: unsure.

## For the orchestrator

- **Brief errors:**
  - "constructor has no arc" describes the build, not the ratified state.
  - The symbolic "not recognised as c" contradicts `geom-core/src/sym.rs`, whose normal form cancels linear terms. The payload shows `frozen: 8`; worth one probe.
  - Class 1's rows look like all in-box conics (r = 0.1 rims, revolved circles), so B restores them. I did not check each row.
- **Off-question defect (likely, by reading):** on `3193e7c97`, `Curve3::cover_over` calls `Cover::arc(&ellipse.frame(), …)`. For an anchored far-centred cone section, that builds the cover from the derived far centre: the same class as the unsound row. File it under `work/oneround/`.
- **The same disease one level up:** surface carriers (sphere, cylinder, torus, cone) store a centre or axis point with no box check. A far-centred surface feeds far data to every section mint. Neither this report nor PR 7194 has an issue filed for it.
- **"Parameter zero at the anchor" against step A:** step A keeps the parameter measured from `u_ref` and stores the anchor's phase. That agrees with the sentence only while the anchor sits at the seam vertex. A quarter-turn re-seam puts θ₀ at ±π/2, where the parameter grid is u·a. The B wording keeps "parameter zero at the anchor"; the code should match it.
- **Assumptions:**
  - "In box" means `|c|∞ ≤ SESSION_BOX_EXTENT` (`geom-core/src/tolerance.rs`, 1e3).
  - The bound "centre and arc in box ⇒ every term is box-sized" assumes `seamed_near` is applied. It is ratified as a convention in the `conic.rs` docs.
- **Not checked:**
  - the ~90 reader count;
  - which of the "moved, not checked against an oracle" rows are in-box;
  - the 1e-6 and 1e-12 runs;
  - PR 7194's comment thread, beyond the fork-log row's quotes.

  I read PR 7131's body only for the `Interval` frame width (polar factor, about one ulp per entry).
- **Feasibility:** B and A both need the reader re-posing, so B is not cheaper on that part. B's extra cost is the two-arm placement and the box check at the mint and map doors. Nothing makes either infeasible.
- **Housekeeping:** I worked read-only. I made one detached worktree at `/root/.local/share/cad-work/designer-7462` (at `e04a87227`) and removed it afterwards. Nothing in `/home/user/cad` was edited.

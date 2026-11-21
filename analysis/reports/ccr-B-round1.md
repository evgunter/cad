## For Ev

**Recommendation (likely): change the sentence.** Each conic stores one point, and the conic's size decides which point:
- **No semi-axis longer than the session box (1 km):** it stores its centre, exactly as handed.
- **Otherwise:** it stores a point of the arc it serves (its *anchor*), with the parameter zero there, and its centre is derived.

Two further rules:
- **One evaluator.** The conic also stores where its stored point sits in its own unit coordinates: `(0, 0)` for the centre, `(cos θ₀, sin θ₀)` for an anchor. So every point is `O + u·a·(cos θ − k_u) + v·b·(sin θ − k_v)`, where `O` is the stored point and `k` that pair. Nothing records how the conic was made.
- **No reader restates the curve.** Covers, extents, chart images and the inverse read the conic's own evaluator from the stored point, never `centre + size·direction`. Readers that need only directions read the axis and seam directly, so they derive no position.

Proposed wording for "Float rounding": "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores one point of its plane, its centre where no semi-axis exceeds the session box, and otherwise a point of the arc it serves with its parameter zero there; every other position is derived from that point, and nothing restates the curve outside its evaluator."

**Premise check: three corrections**
1. **The loss is a property of large conics, not of the centre form.** (likely)
   - Every point lies within the larger semi-axis `a` of the centre. With `a` within the box and the arc in the box, every term of a centre-form point is of box size.
   - So the rounding is a short chain on in-box numbers, which is what the run's admission floor (about 1.8e-12 m) already accepts. The margin is tight: the worst corner is about 15 unit roundoffs of the box, against a floor of 16.
   - Every measured loss is on a conic larger than the box: radius 1500 m, 1e4 m and 1e7 m.
   - The original far-centre row says the same: "a reading on a conic whose stored terms stay in the box is unaffected".
2. **The unsound cover is main's own, not the new form's.** (sure, by arithmetic outside the tree)
   - In that test the ellipse's axes lie along the coordinate axes, so the derived centre comes back exact.
   - The miss of 4.2e-10 m is `Cover::arc` writing the arc's end out again as `centre − 1e7·cos θ`. Main's evaluator makes the identical error, so the two agree to the bit and the test passes.
   - The anchored evaluator returns the true point, and that exposes the cover.
   - So the defect is a second, lossy statement of the conic living inside the cover. It needs fixing under any stored form.
3. **For ordinary conics, storing the anchor costs and gains nothing.** (sure, measured)
   - The moved rows on in-box conics (radii 0.1–2 m) are pure costs of deriving the centre:
     - about ten `Interval` and symbolic builds now refuse;
     - a derived centre is a rounding off the curve;
     - the STEP fixed point moves.
   - Example: a hole and a boss share a centre, each radius boxed to ±0.1 mm. Anchored, each centre reads back as `(c + u·r) − u·r`, which carries ±0.2 mm, so "concentric?" cannot be decided. With the centre stored, both read the same `c` exactly.
   - Reader work cannot remove this. It comes from the stored form itself.

**Ratified text.** An agent wrote the sentence on PR 7194, and you approved it ("ok i see, i think this plan works, then") before any of these measurements existed. There you asked how a centre form is lossier, and were told: only where the centre is far from the arc. The size rule writes that answer into the code. "Parameter zero at the anchor" stays, for anchored conics only. Its grounds still hold: with zero elsewhere, an edge's stored ends sit on a grid of u·a along a large arc.

**What a constructor handed a centre does**
- **No semi-axis past the box:** it stores the centre as given, with no arithmetic.
- **A large conic:** it must also be told the part of the conic it serves, and it anchors there. It refuses loudly if the centre it was handed lies outside the box. Such a centre has already lost about u·|c| in its own digits, and summing an anchor from it only fixes that error in place.
- **Mints that can produce a far centre** (a shallow cylinder or cone cut, a plane through a far sphere) compute the arc's point from the surface's local data. The cone section already does this; the others are owed it.

**What a reader reads**

| A reader that needs | Gets |
|---|---|
| The curve, or a cover, extent or chart of it | The evaluator, from `O` and `k`, at every scalar |
| Directions | The stored axis and seam, exact |
| The centre, on an ordinary conic | The stored centre: the same number at `f64`, the same range at `Interval`, the same expression at the symbolic scalar |
| The centre, on a large conic | A derived value, good to about u·a at `f64`, roughly ulp(a) wide at `Interval` |

A verdict that rests on a large conic's centre is levered by `a`. It belongs to the filed far-centre rows and should read a datum near the arc.

**Contracts and accidents**
- **A cover holds its arc: a contract.** At `Interval` it is an exact enclosure. At `f64` it holds in real arithmetic, to the floor. Holding to the bit is an accident, and on main it is false for far arcs.
- **A carried certificate equals a fresh one: a contract on the door.** A door may carry a certificate only if it keeps its support bit-identical, and the chart image and the carrier must be stated from the same stored point. (unsure: the two moved rows are not traced)
- **The STEP round trip's fixed point: import's contract.** The size rule keeps it for ordinary conics. For large ones, the inverse must read from the anchor, as a circle's angle reading already does. (likely)
- **An exact centre for a circle authored at one: a contract at `Interval` and the symbolic scalar.** That is where the measured refusals come from. At `f64` the rule gives it for free, and you called the match "not a hard requirement".
- **Last bits, mesh digests, query counts, quarter-turn identities: accidents.** Regenerate them.

**Other final states**
- **Every conic anchored, every reader re-posed on its anchor:** defensible.
  - Gains: one kind of stored point.
  - Costs: the box-scalar widening above, on every authored conic. Its only gain on ordinary conics is precision below the floor, which no admitted verdict reads.
- **Choose by where the centre is rather than by size:** close, with the same numbers on every measured row. Size is better because a move or a reversal cannot change it, so a moved body never has to re-mint a carrier.
- **Two forms kept as they were made, or both centre and anchor stored:** rejected. The first leaves a far centre form representable; the second is two descriptions kept in step by hand.

**What would change this:**
- If the pending loss-by-size measurement shows centre-form conics with ρ ≤ 1e3 m losing answers at ε = 1e-12 that anchored ones keep, the threshold is too high. It should drop to a fraction of the box, with the design unchanged.
- If losses appear at ρ of 10 to 100 m, at ε = 1e-9 or at 1e-12 within the floor, the centre form is not well-conditioned at any useful size. "Anchor everything" would then be right despite its `Interval` cost.

**Confidence:**
- the recommendation: likely;
- the in-box loss staying within the floor: likely, and tight;
- the cover attribution: sure;
- the box-scalar widening being structural: sure;
- one branch-free evaluator serving both stored points at the symbolic scalar: unsure.

## For the orchestrator

**What moved me**
- **(1) Criterion:** I moved from the centre's position to the conic's size, the first other report's rule. Size cannot change under a move. The two rules agree on every measured row.
  - Kept from my earlier report: a door refuses a far centre, because those digits are already gone (the third report's refusal too). That refusal now applies only to large conics.
  - Taken from the first report: a door building a large conic must be told its part.
- **(4) The unsound cover:** I was wrong. I had blamed the derived centre. The third report's arithmetic is right: the axes are axis-aligned, so the derived centre is exact, and the cover's own centre-form arc is the cause, on main too. My conclusion that readers read from the stored point stands, and is stronger.
  - My claim that step A's anchored cone sections have the same cover hole still holds. It is the same `Cover::arc` restatement.
- **The `Conic::frame()` split** into a directions reading and named position readings: taken from the third report. About 112 production reads take only directions through it and derive a centre they then discard.
- **Restating sites:** taken from the first report. They include the chart image `Channel::Free { c, a, b }` and the Reach rule's centre datum.
- **The `(0, 0)` spelling:** taken from the third report. In that case the offset should be written as `cos θ`, so that centre-stored conics keep main's bits.

**Where we still differ, and why**
- **(2) Parameter clause:** the first report would defer it to the general-anchor unit. I keep it for anchored conics: the representable-spacing argument from fork 294 still holds, and dropping it reopens a settled point. It is unbuilt: step A stores a phase and keeps the parameter measured from `u_ref`. That agrees with the clause while the anchor sits at the seam vertex.
- **(3) A large conic handed a far centre:** the first report would anchor it from that centre. I refuse. `fl(u·a)` rounds at about u·a in a general direction, and `c` itself is rounded at ulp(c), so the curve is off its surface beyond the floor. Anchoring from it is exact only for test fixtures whose directions are axis-aligned.
- **(5) The symbolic row:** still unsure. The third report calls the expression mismatch "measured", but the row shows only that the zero was lost.
  - `sym.rs` puts expressions in a polynomial normal form, which should cancel `u·r − u·r` when both atoms are the same.
  - The payload reports `frozen: 8`. The likelier cause is a frozen form, or the anchored evaluator's `sin²(θ/2)` atom against the rim's `cos θ`.
  - It does not matter under the size rule, because ordinary conics store their centre. It is worth one probe if anchored conics must give symbolic zeros.
- **(6) Within the floor:** likely. The third report's worst corner is about 15 against 16, so it is tight. The pending measurement settles it, with the thresholds given in the "For Ev" section.

**Owed and unchecked**
- A survey of which production mints would trip the far-centre refusal. The cylinder and cone section tables are named; far circles (sphere sections, cone latitudes) are likely too.
- The same question one level up: `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`. A size rule carries over to surfaces naturally.
- The Reach page's "a datum fixed by a carrier's form (an apex, a centre) is read there" likely needs a revision for large conics. Not filed.

**Housekeeping:** this round I read only the two reports. Nothing was edited or run.

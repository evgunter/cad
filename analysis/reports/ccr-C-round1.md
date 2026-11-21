## For Ev

**Recommendation (likely): change the sentence. A conic stores one point and where that point sits on it. The point may be the centre only when the conic is no bigger than the session box. Otherwise it is a point of the arc the conic serves. Every reader states the curve from the stored point.**

All three designers who weighed this agree on that much. They differ on the points marked below.

Terms:
- *Session box*: no model coordinate lies past 1e3 m.
- *Admission floor*: the rounding a run already allows for a short sum of box-sized numbers, about 1.8e-12 m.
- *Box scalars*: `Interval` and the symbolic scalar.
- *u*: f64's unit roundoff, 1.1e-16.
- *On-arc*: the stored point is a point of the curve, on the part an edge uses.

**Premise check**

1. **"Anchored" is not the well-conditioned form. A stored point near the arc is.** (likely)
   - A point summed from the centre of a box-sized conic rounds at a few u times the box, which the floor already covers.
   - Every measured loss is on a conic bigger than the box: 1500 m, 1e4 m, 1e5 to 1e7 m.
2. **The unsound cover is main's own, shown up by the new evaluator.** (sure; reproduced in f64 arithmetic, not run in the tree)
   - In that fixture the derived centre equals the given centre exactly.
   - The cover rebuilds the arc's end as `centre − 1e7·cos θ`, which is 4.2e-10 m off the true point.
   - Main's evaluator makes the same error, so on main the two agree by bit. The anchored evaluator returns the true point.
3. **The defect is in the readers.** About 110 production reads take the centre and rebuild `centre + size·direction` outside the conic. Under any stored form these must read the conic's own stored point. (sure)
4. **The build that was measured anchors nearly every conic by summing from its centre.** Only the cone section anchors on an arc. So the 45 rows show the cost of deriving the centre and almost none of the gain. (sure)

**Where the designers differ, and what I recommend**

*1. What picks the stored point.* Three criteria were proposed.
- **The centre's position against the box.** I proposed this last round and withdraw it.
  - "Float rounding" says the session box is "never a carrier's stored terms".
  - `[intrinsic.invariance]` says a designed rule commutes with rigid motions. Moving a part can carry a centre across the box face, which would force a re-mint inside a move.
- **Size alone**: no semi-axis longer than the box stores the centre, anything bigger stores a point of its arc.
  - It is pure geometry and unchanged by a move.
  - It makes a small section conic store a centre its mint never computed.
- **Size as the limit, the mint's own datum within it (my recommendation; unsure against size alone).**
  - A centre may be stored only on a conic no bigger than the box.
  - A mint that computes a point of the arc directly (the cone section's near vertex) stores that point, at any size.
  - Reason: the stored point should be the value the mint holds independently, because the box scalars cannot cancel a derived one. This is the case for keeping an authored centre, and it applies equally to a section's vertex.
  - The lane measured the cone section narrower at `Interval` stored on its arc than stored at its centre.
  - Cost: two small conics can differ in which of centre and vertex is exact. The stored pair says which, and both are inside the floor at f64.

*2. "Parameter zero at the anchor".* Keep it, for on-arc conics only. (likely)
- The reason you approved it is that an edge's ends sit on a grid of u·a along the arc otherwise. That bites exactly on the big conics that are now the on-arc ones.
- The build does not yet hold it after a seam turn. That is a debt of the general-anchor unit, not a reason to drop the clause.

*3. A constructor handed a far centre.* It refuses. (likely)
- The other proposal is a door taking a far centre plus the part served, which sums the anchor from that centre.
- Phase 1 measured that route: a section minted from a far centre misses its own surfaces by 1 to 3.6 u·a, whichever point is read afterwards.
- Every mint I read has an in-box point of the arc: a section's vertex, or a sketch arc's start point.
- A whole conic bigger than the box cannot lie in the box, so asking for one is a typed refusal.

*4. Why the symbolic row lost its zero.* Not established by anyone. (unsure)
- I wrote last round that the derived centre is a different expression. That overstates it: the symbolic tier's normal form would cancel `c + u·r − u·r`.
- The failing reading takes only the centre and the radius, so a changed evaluator cannot be the cause there. Forms freezing on size is the likelier cause.
- At plain `Interval` the widening is arithmetic and sure.

**Worked case for the box scalars** (sure at `Interval`)

A hole and a boss share one centre, each radius boxed ±0.1 mm.
- Anchored: each centre reads back as `(c + u·r) − u·r`, carrying ±0.2 mm. "Concentric?" is undecidable over the box.
- Centre stored: both read the same `c`, and it is decided.

**The recommended final state**

- Stored: one point; the axis and seam directions; the sizes; a pair giving the point's place in the conic's unit coordinates, (0, 0) for the centre or (cos θ₀, sin θ₀) for a point of the curve.
- One evaluator: `P(θ) = point + u·a·(cos θ − ξ₀) + v·b·(sin θ − η₀)`.
- Readers:
  - Directions are stored and exact.
  - Where the curve is (cover, extent, chart image, inverse) reads the stored point through the conic.
  - The centre is the stored value where stored, and a named derived reading otherwise.
- A far-sized conic stored at its centre cannot be built.
- Leaves possible: an on-arc conic whose stored point is far from the part actually used.
- Reversible: yes, behind `Conic`.

**The sentence as ratified** remains defensible only if the box-scalar cost above is accepted for every authored circle. No reader work removes that cost.

**Contracts and accidents**

- *A cover holds its arc*: a contract. It is an enclosure at `Interval` and holds to in-box rounding at f64. Bit-exact containment is an accident.
- *A carried certificate equals a fresh one*: a contract, given the image and the carrier are stated from the same stored point.
- *The STEP fixed point*: import's to provide. The present mechanism is an accident.
- *An exact centre for an authored circle*: a contract at the box scalars. At f64 it comes free here.
- *Digests, counts, last bits*: only "same inputs, same bits".

**Proposed text** ("Float rounding"; agent-written, approved by you on PR 7194 before these measurements):

"A stored form that loses digits is re-minted in a well-conditioned form: a conic stores one point and where that point sits on it. The point may be its centre only where no semi-axis is longer than the session box's extent; otherwise it is a point of the arc the conic serves, with its parameter zero there. Every reading of where a conic is starts from the stored point."

**The pending measurement** (centre-form loss by size, 10 m to 1e4 m). What would change this answer:
- *Wrong definite verdicts at 1e3 m or below, at an admitted ε.* The in-box premise fails. The limit must drop below the box, and the box-scalar cost then has to be weighed against it directly.
- *Typed refusals that grow smoothly with size, with no step at the box.* This is what I expect, and it does not change the stored form. It says the in-box near-graze reading should sum chords from a nearby point at the reader. Phase 1 measured that at 40 of 40 from a stored centre.
- *No loss until well past 1e3 m.* The limit could be larger. Nothing else changes.

**Confidence**
- Recommendation: likely.
- Size as the limit, over the centre's position: likely.
- The mint's datum within the limit, over size alone: unsure.
- A box-sized centre form stays within the floor: likely (by hand, about 8 to 10 u·box against a floor of 16; no in-box probe at ε 1e-12).
- The cover correction: sure.

## For the orchestrator

**What moved me**
- **Criterion, from the centre's position to size.**
  - The second report's own consequence, "a map that carries a centre out of the box must re-mint at the arc", is what convinced me a positional rule is wrong.
  - Two ratified texts point the same way: `crates/geom-core/README.md`, "the session box is one predicate over the region a piece of geometry occupies, never a carrier's stored terms", and DESIGN `[intrinsic.invariance]`.
  - The first report's size rule is unchanged by a move. I adopt its limit, and its typed refusal for a whole conic bigger than the box.
- **Symbolic cause.**
  - The second report is right that the normal form cancels linear terms. `sym.rs`'s module docs say the form is polynomial with exact rational coefficients, and that cancellation through a frozen node is lost.
  - I withdraw "a different expression".
  - One fact that narrows it: the first failing assertion of `the_placed_rim_chains_through_the_sketch_rim_to_the_radius` is `sign(rim − radius)`, with `rim = (q_from − circle.frame().origin()).norm()`. No evaluator is involved, so the `sin²(θ/2)` atom hypothesis is ruled out for that assertion (by reading the test). Freezing of the `c + u·r` forms remains likely. One probe settles it.
- **Smaller points I accept.**
  - The trait defaults `unit_in_plane`, `along` and `semi_axis_vectors` are centre-form readings inside `Conic`.
  - The Reach rule's "a datum fixed by a carrier's form (an apex, a centre) is read there" needs revising for on-arc conics.
  - Step A's `cover_over` builds an anchored cone section's cover from its derived far centre.
  - Only one production door anchors on an arc.

**Whether my earlier arguments are answered**
- *A box-sized centre form is within the floor*: shared by all three. Nobody has measured it.
- *The readers are the defect*: shared by all three.
- *The 1e7 cover is main's own*: not answered.
  - The first report agrees in kind.
  - The second attributes it to a cover built from `fl(anchor − u·a)`. In that fixture the anchor sum and the derived centre are both exact (anchor z = 0, centre z = 1e7).
  - The 4.229e-10 is the cover's centre-form rounding of the arc's end: centre form gives 1.0124999824911356, anchored 1.0124999829140626, and the true value matches the anchored one.
  - This matters for what the lane re-pins: that row's fixture is correct under the new evaluator and the cover is the thing to fix.
- *The stored point is the mint's independent datum*: not answered.
  - Both other reports count the `Interval` cost as a cost on in-box conics but do not state it as the principle.
  - Size alone contradicts it for small section conics.
- *The centre door refuses*: the second report agrees. The first offers centre plus window.
- *One pair, one evaluator*: the first report's `k` is the same pair. The second has two arms; I still prefer the pair.

**Where I still differ, and why**
- **Against size alone (first report).** A cone section with a ≤ 1e3 would store `vertex − u·a`, a centre its mint did not compute.
  - At `Interval` the vertex then reads back with the semi-axis's width twice, the same effect as the concentric case, at the incidence "the vertex lies on the ruling".
  - The lane's own number, step A narrower at `Interval` than centre form on cone sections, is this difference.
  - What would move me to size alone: that measurement restricted to sections with a ≤ 1e3 showing no difference worth a second stored-point shape, or Ev preferring the one-line rule. I rate this difference unsure, not likely.
- **Against the centre's position (second report).** Its reading of `[tolerance.units]` as already excluding a far centre conflicts with the README sentence quoted above. Its "near the box edge either form is fine, so the choice is not a verdict" does not remove the re-mint inside a move.
- **Against moving the parameter clause out (first report).** The clause's reason, the u·a·θ₀ grid, applies to exactly the conics that are on-arc under every proposal. Dropping it from the sentence would un-ratify the one part of PR 7194 whose argument is untouched by the new measurements. Scoped to on-arc conics it costs centre-stored conics nothing.
- **Against the centre-plus-window door (first report).** Phase 1 (b) is the measurement: the loss is in the far centre the mint computed. I checked the cone section (`ConeTilt::near_vertex`) and the sketch arc (`RawLoop` holds the start vertex beside `Arc2`'s centre). I did not check revolve rims, blend arms or the shallow cylinder cut for an in-box arc point; if one has none, that door is the fallback and should be named as lossy.

**On the pending measurement**
- ε = 1e-12 is admitted only for K ≥ 2.8 (`ADMISSION_FLOOR / (K − 1)`). Rows at 1e-12 with K = 2 are outside the admission rule and should not be read as in-box loss.
- The near-gap families are tuned to gaps at the rounding scale, so I expect their certified counts to fall smoothly with ρ with no step at 1e3. The first report's test ("if they refuse at 900 m, the threshold is wrong") would then read a graded effect as a failed threshold. Ask the lane to separate wrong verdicts from typed refusals, and to add the reader-side chord variant (phase 1 (a)) at each ρ.

**Not checked**
- Nothing was run in the tree. I read only the two named reports in the round-1 directory.
- Whether the symbolic scalar folds `0·x` and `x − 0`, which the (0, 0) pair needs. If it does not, `center()` returns the stored point structurally.
- The hand bound of 8 to 10 u·box assumes the arc is in the box and a ≤ box, so `|c| ≤ 2·box`.
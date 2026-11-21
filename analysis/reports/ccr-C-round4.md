## For Ev

**Short answers**

- **How ugly?** Not very. A circle and an ellipse each hold their directions, their sizes, and one small field saying where they stand: at the centre, or at a point of the arc. Each of those two holds only its own numbers. Nothing outside the conic's file sees which.
- **Is it the family pattern?** Built this way, yes. It is the shape `Curve3`'s kinds have, and the sketch arc's `Bulge`/`Via`/`Center` specs: each member its own type, one reading for every consumer. What is built today is not: it is a hidden tag beside a field that one of the two states never reads.
- **No cutoff.** Agreed by all three designers. No size picks the kind. Each construction stores the point it holds, the same way every time.
- **The panic.** Yes, it can be a kernel bug, with one condition on what "ill-conditioned" means: the length is the run's *rounding reach*, not the session box.

Terms:
- *Centre-held*: the conic stores its centre.
- *Arc-held*: it stores a point of the arc it serves, with its parameter zero there.
- *Rounding reach R*: the length "Float rounding" already defines, the longest lever over which a short sum's rounding still fits inside the tolerance. It is `box·(K − 1)·ε / ADMISSION_FLOOR`: about 5,000 km at ε = 1e-9 with K = 10, about 5 km at ε = 1e-12, and never less than the 1 km box in an admitted run.
- *Enclosure scalars*: `Interval` and the symbolic scalar.

**The one final state I think all three can sign**

1. **Shape: a placement inside the conic.**
   - `Circle { directions, radius, at }` and `Ellipse { directions, sizes, at }`.
   - `at` is `Centre(point)` or `OnArc(point)`. An ellipse's `OnArc` also holds where on the ellipse the point sits.
   - No unread field, and the shared part is named once.
2. **What decides the placement: the construction.**
   - A mint that holds a point of the arc stores it.
   - A mint whose only datum is a centre stores that.
   - A mint whose centre runs off to infinity at a pose it can reach always stores a point of the arc.
3. **How each is read.**
   - Arc-held: stored point plus short chords, at every scalar.
   - Centre-held at f64: its seam vertex, summed once from the centre, plus short chords. This is the reading measured equal to arc storage, digit for digit, from 10 m to 10 km.
   - Centre-held at the enclosure scalars: `centre + size·direction`, each stored number once. This is what main does today, so no authored circle's `Interval` or symbolic behaviour changes. (likely)
4. **The limit: a centre-held conic has no semi-axis past R.**
   - The reason: its one far sum (the seam vertex) rounds at the centre's distance, and R is by definition where that stops fitting the tolerance's headroom.
   - A move, a reversal and a re-seam cannot break it: none changes a size.
   - A scaling maps the stored point and multiplies the sizes. It keeps the placement and is checked like any other door.

**What happens past R, and to whom**

- **A kernel mint that holds an arc point:** nothing. It stores the point, so there is no far sum.
- **A kernel mint whose centre runs away** (a cylinder cut toward its axis, the equal-cylinder bisector): it must be arc-held always. Four of these still store a centre today. That is the kernel bug.
- **A person who wrote the far centre themselves** (a `Center` arc, a revolve about a far axis, a file's circle given only by its placement): a typed refusal that reads as the admission rule, "this tolerance cannot see past the rounding of a centre that far; loosen it, or give points on the arc". Sketch arcs already refuse this way today (`ArcBelowSceneResolution`).
- **The kernel:** one assertion where a tolerance is in hand, when an edge is certified. Because every door a person sizes has refused first, tripping it means a kernel door chose wrong. That is your panic.

It is not a weird cutoff. It picks no form, it is the admission rule you already have read as a length, and at ordinary tolerance it sits hundreds of kilometres out.

One thing you should hear: R depends on the tolerance. A 10 km centre-held arc passes at ε = 1e-9 and is refused at ε = 1e-12. That is the admission rule's behaviour everywhere else too.

**If you would rather rule on the panic, the options in one line each**

- **R, asked once (my recommendation, likely).** Sound exactly where the tolerance says. A person meets a refusal only for a centre they wrote themselves, past R.
- **The session box's diagonal, about 3.5 km.**
  - Constant, and no person-facing refusal.
  - Wrong length both ways: too lax at the tightest admitted tolerance, where R is 1 km; too strict at ε = 1e-9, where a 100 km arc reads fine.
  - It can only be a test until large-radius surfaces are settled.
- **No number; only the runaway mints assert.** This was my last answer, and I withdraw it. Past R a centre-held conic's far sum no longer fits the headroom, and nothing would say so.

**The measurement that would decide a remaining doubt**

A centre-held conic with its centre past R, built through a real door and certified. At ε = 1e-12 that is 1e4 to 1e6 m.
- Any wrong definite answer: the R rule is required as stated.
- Only typed refusals at every size: certification alone would do, and the assertion is belt and braces.
- The size table already shows the first kind of trouble at twice R (ε = 1e-12, 10 km): roots one parameter step apart near 2π.

**The sentence** ("Float rounding")

"A stored form that loses digits is re-minted in a well-conditioned form: a conic stores the point its construction holds, its centre or a point of the arc it serves with its parameter zero there, as one of a family of placements behind the one `Conic` reading, and no size chooses between them. At `f64` every point is summed by chords from a point of the curve, a centre-held conic's from its seam vertex; as an enclosure a centre-held conic is read from its centre, each stored term once. A centre-held conic has no semi-axis past the rounding reach `R`: a door at which a person sets that size refuses past it typed, and a kernel mint that breaks it is a kernel bug. Nothing states where a conic is except through that reading."

**Confidence**
- The placement shape: likely.
- R as the length: likely.
- The enclosure-scalar reading: likely (reasoned, and it is main's formula).
- The door list: by reading, not run.

## For the orchestrator

**What moved me, and to what**
- **Shape: from two types per kind to the placement enum inside `Circle`/`Ellipse`** (the first report's).
  - Two full types repeat the directions and sizes. DESIGN `[one-way]` says "two types carrying the same fields are a sign of a cleaner arrangement to find … that part, named for what it means, is one type each kind holds".
  - The enum is the same family with the shared part named once.
- **Panic: from "no number" to R** (the first report's second part). Two things I had wrong:
  - *"No door checks a carrier's size or centre."* I grepped only for the session box. `profile/src/validate.rs` has `ArcBelowSceneResolution`, a typed refusal of an arc "stored … at a magnitude whose `f64` rounding the tolerance cannot see past", with the recourse "a flatter arc stores its centre farther out". A person already meets this, in tolerance terms.
  - *"A person reaches any number through legitimate in-box geometry."* That holds against the box and its diagonal. It does not hold against R at an admitted tolerance, where R is at least the box and scales with ε. My 3 km `Center` arc and 5 km wall pass at ε = 1e-9.
  - What I had missed: chord reading makes a centre-held conic consistent with itself, not with its neighbours. The seam-vertex sum is one short chain at the centre's distance, and R is defined as where that leaves the headroom.

**What I concede, and what I think is wrong in each other answer**
- **To the R answer: the length and the principle.** Three things I would change in it:
  - *Doors that hold an arc point should store it, not refuse.* STEP has the edge's vertex. `homothety` can map the arc point (today it rebuilds through `Curve3::circle(map(center), …)`). Refusal belongs only where the far centre is the person's own datum.
  - *The assertion cannot be an invariant on the type.* `Circle::new` has no tolerance. It sits where one is in hand; I suggest the edge's certification, which "Float rounding" already names as the door that asks the session box of an edge. That is the lane's choice, constrained by this.
  - *`ArcBelowSceneResolution` on a flat `Bulge` arc is itself the stored-form defect, one level up.* The person wrote two points and a bulge, and the refusal exists because `Arc2` stores a centre. Restating it through R is right for now. The real fix is the profile's arc holding its start point for specs that wrote no centre. Not filed; it is `profile`'s question.
- **To the diagonal answer: nothing on the constant.**
  - At ε = 1e-12 and K = 2.8, R equals the box (1 km), so a 3.5 km bound admits conics whose far sum does not fit.
  - At ε = 1e-9 it forbids sound ones.
  - "Test-only until five doors and the far-datum surfaces" is not a final state.
  - Its listing of revolve latitudes as arc-held would move `a_revolved_circles_latitude_over_a_centre_box_is_exact`, one of step B's box-scalar rows. Unchecked.
  - I do take from it that the limit must be on size, not on the centre's position.
- **To the single-layout pair `k`: it no longer matches what all three agreed.**
  - At `k = (0, 0)` its one formula is the centre sum.
  - The agreed centre reading sums the seam vertex first, and the centre-held conic re-seams where the arc-held one does not.
  - So the two cases already behave differently, and a formula that branches on `k == (0, 0)` is a tag written as a number. `two-facts.md` does not bear on this.

**Open point (3), the enclosure scalars**
- For point inputs the two associations enclose alike. For a boxed size, `(c + u·a) + u·a·(cos θ − 1)` holds `a` twice, and by subdistributivity is never narrower than `c + u·a·cos θ`. So the centre placement reads each term once at the `Exact` witness kinds. This is the split `Ellipse::quarter_turned_where` already makes, for the same reason.
- No measurement is needed before the sentence, because at those scalars the reading is main's.
- One check the lane should make: with the f64 point summed one way and the enclosure the other, an f64 point can sit an ulp outside its `Interval` enclosure of the true point. The contract is that the enclosure holds the true value. Rows that assert it holds the f64 bits (`r2_circle_at_is_the_old_arm_at_f64_and_contains_it_at_interval` is one) will say whether anything real leans on that. If something does, the alternative is the chord form at every scalar, and then the widening on boxed sizes has to be measured.
- The symbolic scalar: the registered node is the stored centre, unchanged. `two-facts.md` shows `(q + u·r) − u·r − q` decides `Zero`, so either association reduces. I expect larger forms, hence more freezing, under the chord form. Unmeasured.

**Checks on the coordinator's reading**
- "Each construction stores the point it holds" hides one real difference: a swept sketch arc.
  - The first report's first part says start point (arc-held); its second part says centre-held; the second report says by spec.
  - What I think all can sign: the sweep stores what the profile hands it, which today is `Arc2`'s centre for every spec, and follows the profile if that changes.
- Round 2's wording "every reading starts from the stored point" is no longer exact for the centre placement at f64. The sentence above says "from a point of the curve".

**Unchecked**
- The default K. R's figures are for K = 10; at K = 2 and ε = 1e-9 it is about 560 km.
- Whether revolve refuses an axis distance past R anywhere today, and what `step-import/src/entities.rs` checks beyond finite and positive. Both taken from the other reports and the lane's survey.
- Whether certification's own residual arithmetic past R can accept wrongly. That is what the proposed probe reads.
- Nothing was run. This round I read the message, the two reports, and `profile/src/validate.rs` on `origin/oneround/anchored-conic`.
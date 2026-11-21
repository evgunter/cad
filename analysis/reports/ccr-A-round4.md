## For Ev

**Short answers.**
- **How ugly:** small. A circle and an ellipse each hold one of two placements: "about its centre" or "about a point of its arc". Each placement holds only its own numbers. Only the conic module ever asks which one a conic has. What is built today is the ugly version: one struct with a hidden tag, and an ellipse field documented as unread half the time.
- **Is it the family pattern:** yes, in the form I recommend. In the tree that pattern is the sketch arc's `Bulge`/`Via`/`Center`/`Sweep`/`ArcLen` specs, all lowering to one `Segment::Arc` (`[recipe.forms]`), and `Curve3`'s kinds behind shared readings. The two placements are a small enum of that kind, read through the one `Conic` interface.
- **No cutoff:** none. Each construction always makes the same placement, whatever the size. No number ever picks.
- **Your panic:** yes, for "the kernel chose the wrong placement". No, for a size limit.
  - Choosing wrong is a code bug. The constructions whose centre can run off to infinity are given only the arc-point constructor, so choosing wrong cannot compile. A test gate checks the list.
  - A size limit cannot be a kernel bug. A person reaches any length through ordinary in-box geometry: a gently curved wall, a revolve about a far axis, a 5 km arc through the box, a STEP file. And none is needed, because a centre-held conic read from its seam vertex certified as well as an arc-held one at every size measured (10 m to 10 km).

**The final state**
- **What is stored.**
  - `Circle { directions, radius, at }` and `Ellipse { directions, semi-axes, at }`.
  - `at` is either `Centre(point)`, or `OnArc(point)` plus, for an ellipse, where on the ellipse that point sits.
  - On a centre-held conic the parameter is the angle from the seam. On an arc-held one it is zero at the stored point.
- **What decides it:** the construction, fixed in code.
  - A construction handed a centre stores it: an authored centre, a circle about a surface's axis, a STEP circle.
  - A construction holding a point of the arc stores that point: a cone section's vertex, a swept arc's start, a revolve latitude's vertex, an edge's middle point.
  - Constructions whose centre runs off at a reachable pose must take the arc door: the plane–cylinder tilted ellipse, the plane–cone circle on a near-flat cone, the equal-cylinder bisector, the oblique end face, STEP's recognised circle.
  - Nothing converts one placement to the other. Today `homothety` (in offset) rebuilds a circle through its centre and so turns an arc-held circle into a centre-held one. It must map the stored point instead.
- **Readers.**
  - Everything that says where the curve is goes through the conic: points, covers, extents, chart images, the inverse.
  - Directions are read without deriving any position.
  - The centre is the stored value on a centre-held conic (the same number, range or symbolic node), and a derived reading on an arc-held one.
- **A move, a mirror, a scaling** map the stored point and keep the placement; a scaling also multiplies the sizes.
- **A reversal** flips the axis. An arc-held ellipse also negates its stored phase.
- **A re-seam** turns a centre-held conic exactly about its centre. An arc-held conic needs none.

**Past what length does a centre-held conic stop fitting the run's tolerance, and what happens there?**
- It is gradual, with no step. Read from its seam vertex, the only extra rounding is summing that vertex once from the centre: about u times the centre's distance (u is about 1.1e-16).
- Where that passes ε, the edge sits measurably off its own faces. Certification then refuses with a typed error naming the edge, as the sketch validator already does for a far, flat arc.
- That refusal reaches the person who built the geometry. It is the existing admission rule, not a new limit, and no wrong answer was measured at any size.

**Still open, and the lane must measure it first: the centre-held conic at `Interval` and the symbolic scalar.**
- Summed from the vertex, `(c + u·a) + u·a·(cos θ − 1)` uses the size twice. Where the size is a range, the far side gets up to three times the range's width, against once for `c + u·a·cos θ`.
- At the symbolic scalar, the half-angle rewrite brings in a different trigonometric term than the centre-form identities the sweep registers.
- Only `f64` has been measured.
- My expectation: chords at `f64`, the centre sum at the enclosure scalars. The kernel already reads by scalar kind in `quarter_turned_where`.
- The measurement that decides it: re-run the ten `Interval`/symbolic rows that refused in the one-form build, with the centre-held conic read both ways.
  - If chords keep them all certified, there is one reading at every scalar.
  - If not, the scalar split is the final state.

**Sentence for "Float rounding":**
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there, and its construction fixes which, the same at every size; a construction whose centre can run off at a reachable pose holds a point of its arc. A conic's points are read where they are decided from a point of the curve, and every reading of where a conic is goes through the conic."

**Confidence.**
- The family, fixed per construction, no limit: likely.
- Choosing wrong is a kernel bug, enforced structurally: likely.
- Certification refuses past the headroom, and never answers wrong: likely. Measured on one family; the path is read, not probed.
- The enclosure-scalar reading: unsure until measured.

**If you would rather rule than wait.**
- *Shape*:
  - (a) Two placements inside each kind (recommended).
  - (b) One stored point plus a pair with no tag. Rejected: the two placements differ in parameter, reading and re-seam, so the pair is a tag spelled as a number.
- *Panic*:
  - (a) Structural, per construction, no numeric limit (recommended).
  - (b) A numeric limit with typed refusals at the doors where a person sets a size. That means new refusals on geometry that already reads well.
  - (c) A box-diagonal assertion. A person can reach it, so it is not a kernel bug.

## For the orchestrator

**What moved me, and to what**
- **Panic: I conceded to the no-numeric-limit report.** My round-3 rule was "semi-axis ≤ R, typed refusals at five doors". It adds person-facing refusals on in-box geometry (a 5 km arc at ε 1e-12, where R is about 5 km) that the chord reading already handles. It makes one document pass at one ε and refuse at another. And it duplicates certification's typed refusal at the one place loss actually shows.
  - Its door table convinced me. My own addendum pass had found five unguarded doors and proposed guarding each. Theirs shows the honest guard is certification, not a size.
  - The box-diagonal backstop fails for the same reason: a 5 km arc through the box is legitimate. Its author already marks it test-only.
- **The kernel-bug assertion is structural.** The other report puts a debug assertion inside each mint. Since the mint's code fixes the placement, I would make it unrepresentable instead: those mints get only the `OnArc` constructor, and a gate lists them. A runtime check there has nothing numeric to check. That report also says "nothing numeric at the centre door tells a computed centre from a given one", which agrees.
- **Runaway list:** I add the plane–cone axis-normal circle (near-flat cone, radius `|apex_gap/c|·tan α`; in the survey and the first report, missing from the second) to the second report's four.

**Shape**
- I keep the enum inside each kind, not two types per kind and not the pair.
- Against the pair (first report): the measured good reading for the centre placement is "sum the vertex, then chords". With `k = (0, 0)` the shared formula is the centre sum itself, so one formula cannot carry both measured readings. The parameters also differ: angle from the seam against zero at the anchor. Three behaviours split by an in-band value is a tag.
- Enum-in-kind against two-types-per-kind: equivalent for readers. The enum keeps `Curve3`'s variants as geometric kinds (circle, ellipse), so no `match` on `Curve3` learns about placements. I call it the lane's choice, under these constraints:
  - no unread field;
  - only `conic.rs` matches on the placement;
  - `Curve3`'s conic variants stay two.

**Disagreements with the other reports**
- **First report:**
  - "A `Center` written past the box is refused at authoring by `[tolerance.units]`". Unchecked, and a 5 km arc through the box has its centre in or near the box anyway.
  - Its STEP fix ("store the edge's start vertex") changes what a file's circle is. I keep STEP centre-held: the file wrote a centre, and certification guards it.
- **Second report:**
  - Agreed on everything material.
  - Its "authored arcs: the kind should follow the spec" is right, and it is the profile layer's question: `Segment::Arc` erases the spec at lowering. Until that is decided, sweep's arcs are centre-held, and a tiny-bulge arc's derived vertex `fl(c + u·r)` misses `q_from` by about u·R, which certification sees. File it under `profile` (not filed; owed).

**Owed measurements**
1. The ten rows of step-B group 1, with the centre placement read by chords against by the centre sum, at `Interval` and the symbolic scalar. This decides the scalar split.
2. One certified build of a large centre-held conic (a centre at 1e6–1e7, the arc in the box) at ε 1e-9, confirming a typed refusal and never a wrong answer. This decides whether certification really is the backstop.

**Facts I relied on:** the second report's reading that `replace_face::homothety` goes through `Curve3::circle(map(center), …)`. I did not re-check it.

**Housekeeping:** I read only the round-4 message and the two reports; read-only; nothing run.

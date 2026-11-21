## For Ev

**Short answers.**
- **How ugly:** not very. It becomes one small type with two variants, "stored at the centre" and "stored at a point of its arc". Circle and ellipse each hold one. Each variant holds only its own numbers, and nothing outside the conic module can see which variant a conic has. The work that is genuinely ugly is elsewhere: about a hundred readers rebuild the curve from its centre, and they have to be fixed under any form, including one form.
- **Is it the family pattern:** the build is not, and the version I recommend is.
  - In the kernel the pattern is `Curve3`: an enum of kinds (`Line`, `Circle`, `Ellipse`, `Nurbs`), each its own type with its own fields, all read through one set of readings. The `Conic` trait does the same for circle and ellipse.
  - In the recipe it is the sketch arc's `Bulge`/`Via`/`Center`/`Sweep`/`ArcLen` specs, which `[recipe.forms]` describes: each spec keeps the numbers that were written, and all of them lower to one `Segment::Arc`.
  - The built code is a tag inside one struct (`Born`), with 37 matches on that tag. The ellipse also carries a `phase` field documented "Unread on an ellipse born with a centre". A field that means nothing under one tag is a bad state the type can represent. In the family version that field lives inside the variant that uses it.
- **No cutoff:** agreed, and the new measurements back you. Each mint stores what it actually holds; size plays no part. The measured loss has no step at the box edge. It grows smoothly from inside the box, it is cured by how the curve is read rather than by which point is stored, and the cure works the same whichever point is stored.

**What the new measurements say** (lane, 2026-11-21):
1. **Near-graze families, ε = 1e-9.** These are the 40-pose test families (circles and ellipses grazing a cylinder wall), checked against an exact oracle.
   - Reading points from the centre: all 40 certify up to 500 m. At 900 m (inside the box) 33 certify, at 1 km 28, at 1.5 km 7, at 10 km none. Every loss is a typed refusal; nothing reads wrong.
   - Reading points by short chords from a point of the curve: all 40 certify at every size from 500 m up.
   - That chord reading gives the same results digit for digit whether the conic stores an anchor or stores its centre and sums its vertex from it.
   - So the loss is in the read, and a box limit is not a conditioning boundary.
2. **Small cone sections, near the parabola.** These are a cone cut by a plane almost parallel to a generator; the cases measured are 16.6 m and 166 m, well inside the box.
   - Stored at a centre the mint never computed, a point's `Interval` residual on its cone is 5e-8 m wide at every parameter.
   - Stored at the vertex the mint computed, it is about 1e-15 at the vertex and grows with the chord.
   - So what matters is storing the datum the mint holds, at any size.
3. **The lost symbolic identity.** The sweep registers a rim by the centre's own symbolic node, so a derived centre loses the identity by construction, not through any arithmetic. Again: store what the mint holds.

**The final state I would build**
- **What is stored.**
  - `Circle { directions, radius, at: CircleAt }` and `Ellipse { directions, semi-axes, at: EllipseAt }`.
  - Each `…At` has two variants. `Centre(point)`. `OnArc(point)`, plus, for an ellipse, where on the ellipse that point sits (its *phase*, `(cos θ₀, sin θ₀)`).
  - The variant is the shared part of circle and ellipse, named once, as DESIGN `[one-way]` asks for shared fields.
- **The shared API.**
  - It is the `Conic` trait, under `Curve3`: point, derivatives, angle of a point, centre, zero point, cover, and the inverse.
  - `frame()` is split in two: a directions reading, which derives no position, and named position readings.
  - Each variant has one evaluator, summed by chords from a point of the curve. `OnArc` sums from its stored point. `Centre` sums from its seam vertex, `centre + u·a`, computed once per reading, and its seam is kept at the vertex nearest the arc it serves (the convention already in place). That is the reading the measurement shows to be as good as the anchored one.
- **What the readers get.**
  - `center()` on a `Centre` conic returns the stored value: the same number, the same `Interval` range, the same symbolic node.
  - On `OnArc` the centre is derived, good to u·a. Here u is f64's unit roundoff (about 1.1e-16) and a the larger semi-axis.
  - No reader rebuilds `centre + size·direction`.
- **What decides the variant:** the mint, by what it holds. A sketch circle's centre gives `Centre`. A swept arc's start point, a revolve latitude, the cone section's near vertex, or an imported edge's vertex gives `OnArc`. Nothing records history beyond the variant itself.
- **A move:** maps the stored point and the directions; the variant is kept.
- **A reversal:** flips the axis. `OnArc` negates sin θ₀ and keeps its point.
- **A re-seam:** `Centre` permutes the directions and sizes exactly, with the centre kept bit for bit. `OnArc` keeps parameter zero at its point and never needs a re-seam, because it is seamed on its arc already.
- **A scaling about p:** maps the stored point to `p + s·(x − p)` and multiplies the sizes by s; the variant is kept.

**What replaces the size limit: nothing that picks a form.**
- A far centre loses digits only once, in one place: summing the vertex from it costs about u·(distance of the centre) as one rigid shift of the curve.
- Where that shift exceeds ε, the edge sits off its own faces, and certification's residual check refuses it with a typed error. So it is loud, not silent. (likely)
- The real fix belongs at the mints that today hold only a centre but can make a long conic: the plane–cylinder tilted ellipse, the plane–cone circle across the axis, `bisector_ellipses`, STEP's circle and ellipse, `homothety`, and offset's seam circles.
  - Each should take a point of the arc from its own local data. STEP has the edge's vertex; the section mints have the arc they cut.
  - That is per-door work, filed as such, not a rule about sizes.
- A box refusal on the stored centre would be the session box asked of a carrier's stored terms. The geom-core README rules that out ("never a carrier's stored terms"), and it would refuse a real file's 5 km arc.

**What stays from before:**
- "Parameter zero at the anchor", for `OnArc` conics.
- The unsound 1e7 cover is a bug on main, fixed when covers read through the conic.
- Which bit agreements are contracts and which are accidents (unchanged):
  - A cover holds its arc: contract. In real numbers at f64, as an enclosure at `Interval`.
  - A carried certificate equals a fresh one: contract on the door.
  - The STEP fixed point: owed by import.
  - An exact centre for a circle authored at one: contract at the box scalars.
  - Digests and last bits: habits.

**Sentence for "Float rounding"** (`crates/geom-core/README.md`):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores the point its mint holds, its centre or a point of the arc it serves with its parameter zero there, as one of a family of placements behind the one `Conic` reading; every point of a conic is summed by chords from a point of the curve, and no reader states where a conic is except through that reading."

**Confidence.**
- The family of two placements, chosen by the mint, with no cutoff: likely.
- The chord evaluator for the centre variant: likely. Measured on one family at ε 1e-9 and 1e-12, at sizes from 10 m to 10 km.
- That certification catches a far centre's rigid shift: likely, by reading the check; not probed.

## For the orchestrator

**What moved me, and to what**
- **The size limit is dropped.**
  - The size table refutes my "in-box centre form stays within the floor". At ε 1e-9 the centre-form meter reads about 1.3e4·u·ρ from ρ = 100 up. Refusals begin at 900 m, inside the box. The meter's slack includes the pose's conditioning, so it is not a short chain.
  - The reader-chord row, which matches the anchored row digit for digit, moves the cure from storage to reading.
  - Ev's "no weird cutoff" and the data agree.
- **The single evaluator over a placement pair is replaced.**
  - My earlier form was `O + u·a·(cos θ − k_u) + …`, where `k = (0, 0)` is the centre chain itself. It is replaced by a per-variant chord evaluator, `Centre` summing from its seam vertex.
  - The family enum is what Ev asked about. It removes the `phase` field that the build leaves unread on centre-born ellipses.
- **The mint's datum, at any size.** The near-parabolic `Interval` result (5e-8 m at a = 16.6 m) makes this the principle by itself, with no limit attached. The step-B evidence is consistent with it: `two-facts.md` on the symbolic registration naming the centre's node.

**Are my earlier arguments answered?**
- Hole and boss, readers being the defect, the cover being main's own: all confirmed by `two-facts.md`.
- The refusal at a far-centre door is withdrawn. It conflicts with the README's box sentence, and the vertex-sum shift is caught by certification. That last point is by reading, not probe: check 4's map residual and the carrier-on-faces residual.

**Where I may differ from the others**
- They may keep a refusal or an invariant on far centres. If Ev wants fail-loud at the door rather than at certification, the least-bad form is a gate on the specific doors listed in `doors-that-can-mint-a-long-conic.md`, not on the type.
- **Owed probe:** a `Centre` conic with its centre at 1e6–1e7 and its arc in the box, built through a real door and certified, at ε 1e-9. Confirm it refuses typed and never reads wrong. If it reads wrong, a door refusal is needed after all.
- **The `Centre` evaluator moves the bits of every centre-born conic.** This is the "anchor recomputed from the centre" variant that the PR 4812/7194 record dismissed as 5×. The size table is the new evidence for it. Ev should hear that this reverses that dismissal on measurement.

**Ratified text**
- `[recipe.forms]` is scoped to the recipe ("a form joins a family only because it is how someone authors the value"). The kernel family is keyed on what the mint holds, which is the analogue one layer down.
- If Ev wants the kernel family named in DESIGN, `[one-way]`'s sentence "A family of authored forms is not a second way…" would need a clause for derived carriers. I would not add it unasked.

**Not checked**
- Whether every listed centre-only door really has an in-box arc point. STEP's edge vertex and the section's cut arc are my reading. I did not open `bisector_ellipses`, `homothety` or the offset seams.
- The symbolic folding inside the chord evaluator for `Centre` (`c + u·a` as a node). The registration keys on the centre node, which stays stored, so I expect the identity to hold. Unchecked.

**Housekeeping:** read-only. I fetched `oneround/anchored-conic` and the probes branch. I read DESIGN `[recipe.forms]` and `[one-way]`, `Curve3`'s variants, the built `Born` tag and its match count, and the three measurement READMEs (the size table in full, the others as summarised in the message). Nothing was edited or run.

## For Ev

**How ugly is it?** As built, yes, ugly. Each conic struct carries a hidden tag saying how it was made, and an ellipse holds a `phase` field documented as "unread" in one of the two states. That is a field that means nothing half the time.

**It does not have to be two forms.** The clean version is one layout and one formula:
- each conic stores one point, plus where that point sits on the conic, written in the conic's own unit coordinates;
- (0, 0) means the point is the centre; (cos θ₀, sin θ₀) means it is a point of the curve;
- every point is computed as `stored point + u·a·(cos θ − k_u) + v·b·(sin θ − k_v)`, where `k` is that pair;
- there is no tag and no unread field.

The centre and an arc point are two values of the same field, the way a line's origin can be any point of the line. This is still the "one form" the last change was for.

**Is it the family pattern?** The tree has the pattern at two layers:
- **The recipe:** a sketch arc's `Bulge`/`Via`/`Center`/`Sweep`/`ArcLen` specs, each keeping exactly what was written and all lowering to one `Segment::Arc` (`[recipe.forms]`).
- **The kernel:** `Curve3`'s kinds (line, circle, ellipse, spiric, spline), each its own type behind shared readings. Circles and ellipses share the `Conic` interface.

The built tag-inside-a-struct is neither. The pattern would be separate small types, each holding exactly its own numbers. Here that is unnecessary: both stored points fit one formula, so the "family" collapses to one member. If you would rather see two named types behind `Conic`, that is also clean and equivalent. (likely)

**No cutoff.** Nothing measures the conic to pick a form. Each door stores the point it independently holds:
- a circle authored by its centre stores that centre;
- a cone section minted at its vertex stores that vertex;
- a swept bulge arc stores its start point.

A door always stores the same kind of point, whatever the size.

**Your panic idea works as a backstop:** "a conic held at its centre has no semi-axis longer than the session box's diagonal (about 3.5 km)". That diagonal is just the session box asked again: any conic whose centre and arc both lie in the box passes it. It becomes a kernel bug, and never a refusal a person meets, once five doors are fixed (below) and one prior question is settled: surfaces whose own centre or axis point lies far from their face.

**What the new measurements settle**
- **The f64 far-centre loss was in the reading, not the storage.** (sure, measured) In the near-gap family:
  - a conic stored at its centre and read by chords from a point next to the arc certifies 40 of 40 at every size from 500 m to 10 km, digit for digit the same as one stored on its arc;
  - read the old way, the centre form falls to 28 of 40 at 1 km and 0 at 10 km;
  - nothing was wrong in any form, only refused.

  So every reader near an arc sums by chords from a point near it, whatever is stored.
- **Storage matters at `Interval`, and it must follow the mint.** (sure, measured) Small cone sections near the parabola, stored at their vertex, enclose a point's distance from the cone to about 6e-16 m. Stored at their centre, the same enclosure is 5e-10 m even at a semi-axis of 1.66 m. A size rule storing small conics at their centre would have been wrong.
- **The symbolic loss is about identity.** (sure) The sweep names the centre's own expression, and a derived centre is a different expression.
- **The 1e7 cover is main's own defect.** (sure) Main's evaluator made the same error as the cover, so the test passed by agreement.

**What a move, a reversal, a re-seam and a scaling do**
- **Move:** maps the point and the directions. The pair is unchanged and the form never changes.
- **Reversal:** flips the axis and negates the pair's second entry. The point stays.
- **Re-seam by a quarter turn:** permutes the directions, the sizes and the pair exactly. The point stays.
  - A large conic stored on its arc is not re-seamed: its seam is fixed where it was minted, at its stored point.
- **Scaling about a point:** maps the stored point and multiplies the sizes. The pair is unchanged.
  - A conic held at its centre can grow past the bound this way. That is one of the doors below.

**Door by door: can a person reach a large conic held at its centre?**

Already fine:
- **Sketch arcs swept or extruded:** the stored point follows the authored spec. A `Center` arc stores its centre; a `Bulge`, `Via` or `Sweep` arc stores its start vertex, which the sweep holds (`q_from`).
  - A near-flat bulge arc can have a radius of kilometres, so it must not store the centre the sweep derives. It holds the start vertex.
  - An in-box `Center` arc is within the diagonal by construction.
  - A `Center` written past the box is a coordinate the session box should refuse at authoring (`[tolerance.units]`). Whether that refusal exists is unchecked.
- **Doors that already hold an arc point:** the tilted cone section, revolve latitudes, blend's oblique end face, offset's latitude rule, `transport_curve`, and recognised circles. They store it, so the bound never applies.

Need work before the backstop can panic:
1. **Tilted plane × cylinder ellipse:** today it holds only a centre, which runs far as the plane nears the axis. It needs to compute a point of the arc from in-box data and store that, always.
2. **Axis-normal plane × cone circle:** the same, for a far apex or a near-flat cone.
3. **`bisector_ellipses`:** the same as the axes near parallel. Their minor vertices are already computed, just after the mint.
4. **STEP `CIRCLE` / `ELLIPSE`:** the file's sizes are unbounded. Store the edge's start vertex, which import holds.
5. **`homothety`:** map an arc point (the edge's vertex), not the centre, whenever the source is held at its centre.

The remaining gap is sections of a large sphere, torus, cylinder or cone whose face is in the box: sphere and torus sections, and coaxial circles. Their radius is bounded only by the surface's own, so a person can reach a large centre form through them. This is the filed question of surfaces stored by a far datum. Settle it first, or have those doors store an arc point, before the assertion goes in. Until then it must stay a test-only check.

**Contracts:**
- a cover holds its arc (stated from the stored point, through the conic);
- a carried certificate equals a fresh one;
- an exact centre at the box scalars, for a conic held at its centre;
- the STEP fixed point, which is import's to provide.

Bits, digests and counts are habits.

**Proposed sentence** ("Float rounding"):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores one point and where that point sits in its unit coordinates, the point its mint holds — its centre, or a point of the arc it serves with its parameter zero there; a reading near an arc is summed by chords from a point near it, never through terms of a far distance; and a conic held at its centre has no semi-axis longer than the session box's diagonal, so a mint that can make a longer one holds a point of its arc."

**Confidence:**
- one layout with no cutoff: likely;
- chord readers carry the f64 case: sure for the measured family, likely in general;
- the backstop is a kernel bug once those five doors and the far-surface question are dealt with: likely, from a door survey whose rows were not each re-checked.

## For the orchestrator

**What moved me, and to what**
- **I dropped the size limit as the rule that picks the form.** Two measurements did it:
  - the size table: reading by chords from `fl(centre + u·ρ)` equals on-arc storage digit for digit;
  - the small-cone probe: vertex storage beats centre storage at `Interval` at a = 1.66 m.

  Together they show the f64 case is the reader's job and the `Interval` case is the mint's datum. A size rule picking the form is wrong in both. It survives only as Ev's backstop, at the box diagonal, not the box extent.
- **Ev's "one form"** moved me to state the pair as the single layout and to call the built `Born` tag the anti-pattern (`[unrepresentable]`: a field unread in one state).
- **Symbolic cause:** I was wrong about freezing. Per `two-facts.md` it is node identity.
- **The in-box floor claim:** it held for soundness (no wrong verdict at any size) and failed for completeness near 1 km, where the centre form refused from 900 m. Chord readers fix it, not storage.

**Round-2 arguments:** the mint's datum as the stored point is now carried by the measurements. The move-invariance argument still stands against any position rule, including the backstop. That is why the backstop is a bound on size (the diagonal) and not on where the centre lies.

**Errors or gaps**
- **"Every reading starts from the stored point"** (round 2's wording) is not quite right for a centre-held conic. A reading near an arc starts from a near point it derives, as the reader-chord form does, and charges that point's rounding once.
- **`[recipe.forms]` limits families to authored forms:** "never for the kernel's convenience". A carrier-level family would rest on Ev's PR 6329 allowance, not on that clause. The single layout avoids the question.
- **The `(0, 0)` case:** the branch-free offset `−(2·sin²(θ/2) − (1 − k_u))` is not bit-equal to `cos θ`. So it moves centre-held conics' last bits, which are habits. The symbolic rows in `two-facts.md` decide `Zero` either way.

**Unchecked**
- Whether the recipe refuses an authored `Center` past the box.
- Whether a body-scaling edit re-derives carriers from the recipe or goes through `replace_face::homothety`.
- The survey's two "unsure" rows: `junction::scaled`'s bound, and the bulge-arc cap in `profile/src/validate.rs`.
- Whether import's `build.rs` reaches the `CIRCLE` door for free curves at all, given `[import]`'s section rebuild.

**Off the question**
- The backstop depends on `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`. Weigh it next, with the same "store the held point" answer.

**Housekeeping:** read-only; nothing was edited or run.

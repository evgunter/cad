## For Ev

**Decision asked.** Readers stop taking a conic's centre or frame. A conic hands out one opaque **position**, the kernel names one **polynomial along a conic**, and the centre becomes a wrapped datum that no decision reads.

- **Position**: the conic's point written as `stored + U·ξ(t) + V·η(t)`. `stored` is the one point the conic holds. `U`, `V` are two vectors. `ξ`, `η` are two functions of the parameter that belong to the placement: `cos t`, `sin t` about a centre; `cos t − cos t₀`, `sin t − sin t₀`, formed without cancellation, about a point of the arc. A reader sees none of these, nor which placement it has.
- **Polynomial along a conic**: what a reader gets by composing its reading with the position (a height above a plane, the part across an axis, a squared length, products). It lives in `geom` and replaces `half_angle::TrigPoly` and about ten private harmonic structs.
- **Who does what**: `geom` states numbers and `geom-brep` decides.
  - `geom`: the polynomial's sums and products, its value with a rounding bound, its ranges over a window, its jets, the size its rounding is metered against, and for the first degree its extremes and roots posed about the stored point.
  - `geom-brep`: which polynomial a surface asks for, and every band, meter, subdivision, ladder and refusal.
- **Why the posing sits with the polynomial** (a scratch probe at f64 against 80-digit arithmetic, not kernel code; the kernel measurement is under way). An ellipse held on its arc, semi-axes 1e6 and 3e5 m, a plane crossing within 3 m of the stored point. The root misses by 1.7e-10 m when given the right constant but solved as the doors solve today, and by 5e-16 m when posed about the stored point.
- **Centre**: `centre()` returns a `Centre`, stored or derived, which cannot be added to a vector, and there is no panic. It offers `axis_line()` and one door to its coordinates, and no distance or offset methods, so every reader of the number is on one list. That list is: code that reports the centre (the STEP writer, a description shown to a person) and code that mints a new datum from it (a surface about a circle's axis). "Coaxial", "a parallel of", "concentric" and "one locus within the band" are tests that a polynomial is constant or zero along the conic. On a centre-held circle that is the same arithmetic as today.
- **Off `Conic`**: `frame`, `along`, `semi_axis_vectors`, `unit_in_plane`, and `u_ref`/`v_ref`. The plane normal `axis()` stays. The principal frame is asked as `major_frame()` / `section_frame()`, whose answer already states the parameter at its vertex.
- **A conic made from a conic** is a map on the conic: the existing ones, plus a homothety and a concentric offset.

**README, replacing "Every reading of where a conic is goes through the conic."**
> A conic's position is the point it stores plus two vectors times two functions of its parameter, and every reading of where the conic is (at a parameter, along it, or of a point against it) is read from that position. Its centre is a datum of the whole conic, stored or derived.

**What you should know.** PR 7258 told you nothing about a centre-built conic changes. That holds for the curve and its evaluator: `point_at` keeps its bits. It does not hold for the readers' arithmetic on the curve: their last bits may move, and I do not count keeping them as a goal.

**Pushback I expect.** Whether a named polynomial with its own arithmetic is more machinery than the problem needs. My answer: the kernel already carries that machinery about eleven times over, each copy shaped about the centre.

Confidence: the shape, likely. Every circle decision re-poses as a test on the position: likely to unsure until the 79 are classified.

## For the orchestrator

Your summary of the agreement is right. One nuance: one round-1 report offers a posed half-angle form at every degree, not only the first. That is item 4.

1. **Circle datum reads.** The three positions share one final state: decisions are tests on the position, and `Centre` goes only to reporters and mints. What separates (i)/(iii) from (ii) is order, which is yours to set.
   - (ii) moves me in one respect. Once the reads sit behind `Centre`, a forgotten read is no longer invisible: it is a caller of `Centre`'s coordinate door. My round-1 "nothing is red" objection then holds only if `Centre` offers convenient decision methods. So I sign either order, on the condition that `Centre` has `axis_line()` and one coordinate door and nothing else, and that the unit minting the first arc-held circle is accepted on that door's caller list being reporters and mints only.
   - I dropped concentricity of whole circles from the list: it is one circle's level being constant along the other.
2. **Two conics compared: three.** Read in the tree: `injective::one_stated_conic`'s circle arm compares the origin, the axis up to sign and the radius, and not `u_ref` at all; its ellipse arm admits an opposed `u_ref`. `same_content` answers no to a re-seamed or reversed restatement, and its doc says so. Replacing `one_conic` with it changes answers for two edges on one circle with different seams or senses. I did not trace what then refuses. The third door is exact and conservative: it answers no for two arc-held conics with different anchors. It lives in `conic.rs` because it reads the placement. Likely.
3. **What stays public.**
   - `along` and `semi_axis_vectors` fold in; two of us now say so.
   - `u_ref`/`v_ref` left public are a convention, not enforceable. To make it hold by type they leave the trait and the principal frame is `MajorFrame` (`major_dir` with its `phase`). Unsure how many of the 112 direction reads are `axis` only; the classification should count that.
   - `unit_in_plane` goes. Its three production callers want an angle (`azimuth`), a side (a `level(p)` reading from the stored point), or one conic against another (item 2's band question).
4. **Ladder anchors: a question, not a re-based value.** A re-based polynomial is a second evaluable statement of the residual with a rounded `P(θa)` as its constant. A coefficient array handed to the ladder is solver input, whose later rounding `half_angle.rs` already leaves to its band decisions, and the ladder's answers are dropped anyway. Whether the 32-anchor rule survives, or the anchor is simply the stored point, is the measurement's to say.
5. **Centre-held bits.**
   - The evaluator's bits are a property of the final state: the position evaluates as `point_at` does (D9).
   - The readers' residual bits are a non-goal. My round-1 use of "keeps main's bits" as an argument was "unchanged" standing in for "right", and I withdraw it.
   - What the measurement decides is verdicts against the oracle, not bits. If the offset basis moves a centre-held row the wrong way, the opaque type can hold Fourier coefficients for that placement without any reader changing. That is the one thing a published view could not do.
6. **Sentence.** I sign the one above: (b)'s plain statement of the form, (c)'s three kinds of reading, (a)'s "datum of the whole conic". Against the others as they stand:
   - (a) leaves "harmonic … about" undefined;
   - (b) ends on "never a term of a point", which alludes to the error, and its list (range, roots, residual) names what the measurement may move;
   - (c)'s "offsets" needs a gloss.
7. **What Ev is asked**: the first paragraph of For Ev. Pushback as there; second, the bits note.
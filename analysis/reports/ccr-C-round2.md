## For Ev

**Recommendation (likely): the limit on storing a centre is the conic's size, not where its centre lies. Inside that limit two rules are defensible, and they differ on exactly one kind of conic: a small tilted section of a cone. I lean to "a mint stores the point it computed" over "size alone" (unsure).**

Everything else is agreed by all three designers and unchanged from last round: one stored point and one evaluator; readers state the curve only from the stored point; a far centre is refused at the door; the unsound 1e7 cover is main's own; "parameter zero at the anchor" stays for on-arc conics.

Terms:
- *Session box*: no model coordinate past 1e3 m.
- *Admission floor*: the rounding a run already allows, about 1.8e-12 m.
- *Box scalars*: `Interval` and the symbolic scalar.
- *On-arc*: the stored point is a point of the curve.
- *u*: f64's unit roundoff, 1.1e-16.
- *w(x)*: the width of x's range at `Interval`.

**The three criteria**

*1. The centre's position: store the centre when it lies in the box.*
- Makes true: every stored coordinate is in the box when minted. A long ellipse whose centre is in the box keeps its centre.
- Leaves possible:
  - **A move changes the form.** A part moved inside the box can carry a centre across the box face, in either direction. Then the move re-mints the carrier, or the form depends on where the part used to be. Allowing slack past the box does not close this: the slack needed after a move is not the slack tested at the mint.
  - **Its accuracy rests on a seam convention circles do not have.** The claim that every term is box-sized needs the seam at the vertex nearest the arc. A circle has no re-seam door (`Curve3::seamed_near` answers nothing for one; filed as `a-far-circle-has-no-re-seaming-door`). A circle centred in the box can have radius 3.4 km, and at parameter 5 rad its representable points are 3e-12 m apart along the arc, which is past the floor. (sure on the arithmetic)
  - A long centre-stored ellipse read at the wrong seam loses u·a, with only a convention against it.
- Ratified text:
  - It cites `[tolerance.units]` ("no coordinate larger"). "Float rounding" pins what that means: the session box is "one predicate over the region a piece of geometry occupies, never a carrier's stored terms". So this criterion needs that sentence changed too.
  - `[one-way.places]`: "a finished body moved by an isometry stays finished and asks only the session box again". A re-mint inside a move breaks that.
- Under it: a move may change the form; a reversal changes nothing; a re-seam is required for soundness on long ellipses.

*2. Size alone: no semi-axis longer than the box stores the centre; anything bigger is on-arc.*
- Makes true:
  - The form is a function of the curve. On-arc means large, so an on-arc conic is never a whole closed curve and is never re-seamed.
  - Every small conic has an exact stored centre.
  - The worst-corner rounding is about 9 u·box against a floor of 16 (by hand), with no seam convention needed.
- Leaves possible:
  - A small cone section stores `vertex − u·a`, a centre its mint never computed. At `Interval`, every point near the vertex then carries the semi-axis's width twice.
  - Toward the parabola the semi-axis is badly determined while the vertex is not, so this can exceed ε while the semi-axis is still under 1 km. (likely; by hand, not measured)
- Ratified text: agrees with both sentences above. Size is a length, not the box predicate. `[determinism.equivariance]` prefers intrinsic quantities.
- Under it:
  - A move maps the stored point and nothing else.
  - A reversal flips the axis and the sign of the pair's second entry; the stored point stays.
  - A re-seam of a centre-stored conic is an exact turn about the centre, and is no longer needed for accuracy. An on-arc conic is not re-seamed.

*3. Size as the limit, the mint's own point inside it (my lean).*
- The rule: the centre door refuses a conic bigger than the box; the on-arc door takes any size; no door converts one stored point into the other.
- Makes true:
  - Everything criterion 2 gives at f64.
  - A section's vertex stays the exact datum, as an authored circle's centre does. It is one principle: store the value the construction determines, because the box scalars cannot cancel a derived one.
- Leaves possible:
  - Two mints can store the same small curve two ways, so which of centre and vertex is exact is not predictable from size. The stored pair says which. Both are inside the floor at f64.
  - A small on-arc conic can be a whole closed curve, with points up to 2a from its stored point. That is still within the floor.
- Ratified text: as criterion 2. It sits less easily with PR 7194's "nothing records how a conic was made": on a small conic the pair is a choice of the mint, not derivable from the curve.
- A move, a reversal and a re-seam behave as under criterion 2.

**What decides between 2 and 3**

The only conic they treat differently in today's tree is a tilted cone section with semi-axis under 1 km.
- *A measurement.* Take such sections, including near-parabolic ones, at `Interval`, stored at the vertex and stored at `vertex − u·a`. If any certified build near the vertex is lost under the second, take 3. If the difference is negligible, take 2 for its one-line rule.
- *Your preference.* 2 is "small conics store their centre, large ones a point of their arc". 3 is "a door stores what it is handed, and the centre door has a size limit".

**Worked case, why a derived point costs at the box scalars** (sure at `Interval`)
- A hole and a boss share a centre, each radius boxed ±0.1 mm. On-arc, each centre reads back as `(c + u·r) − u·r` with ±0.2 mm. Centre stored, both read the same `c`.
- The mirror case is what separates 2 from 3: a cone section whose semi-axis is known to ±w. Centre stored as `vertex − u·a`, the vertex reads back with ±2w. Vertex stored, it reads back exactly.

**Proposed text** ("Float rounding"), under 3. Under 2, replace "may be its centre only" with "is its centre".

"A stored form that loses digits is re-minted in a well-conditioned form: a conic stores one point and where that point sits on it. The point may be its centre only where no semi-axis is longer than the session box's extent; otherwise it is a point of the arc the conic serves, with its parameter zero there. Every reading of where a conic is starts from the stored point."

**Confidence**
- Size over position as the limit: likely.
- 3 over 2: unsure.
- A box-sized centre form within the floor: likely; the size measurement is pending.
- The circle arithmetic against criterion 1: sure.

## For the orchestrator

**What moved me this round**
- **Nothing changes my criterion.** I still hold size as the limit with the mint's point inside it, and now rate size alone as defensible rather than second-best.
- **One correction to my own round-1 citation.** I wrote that `[intrinsic.invariance]` says a designed rule commutes with rigid motions, as if position conflicted with it.
  - `[determinism.equivariance]` is "at the semantic level (in ℝ) … not bitwise", with its premise marked unaudited. A positional choice of stored point denotes the same curve, so it does not violate the clause.
  - The clause only "prefers intrinsic quantities".
  - The texts that actually bind against position are the README's session-box sentence and `[one-way.places]`'s moved-body sentence (DESIGN, the canonical-places table, row "A body's at-rest validity").
- **One door I now accept.** The size-alone report keeps a centre-plus-served-part door for a large conic whose handed centre is in the box. That is sound: the anchor is summed from box-sized terms, and the centre there is exact in-box data. I withdraw my blanket "no door converts". The refusal is for a far handed centre.
  - This also takes the position rule's one advantage: the 1e4 minor-vertex ellipse centred in the box is served either way at 1e-14.

**Whether my round-1 arguments are answered**
- *The cover is main's own*: answered; both now agree, one citing the arithmetic.
- *Refuse a far centre; keep the parameter clause for on-arc conics; the `Conic::frame()` split*: adopted by both.
- *The stored point is the mint's independent datum*: not yet answered. Their round-1 reports were written against my first report, which did not separate small sections from authored circles. Both use the concentric case for authored circles; neither applies it to a section's vertex.

**Where I still differ, and why**
- **Against position (the first report).**
  - *Circles.* Its floor argument is "centre in the box, plus `seamed_near`". `Curve3::seamed_near` returns `None` for a circle (pinned at `crates/geom/src/curves.rs`, `circle.seamed_near(1.0).is_none()`). A circle centred in the box has r up to 2√3·1e3, and ulp(5 rad)·r is 8.9e-16 × 3.4e3 = 3.0e-12 m, against a floor of 1.8e-12. Its own "15 against 16" corner excludes this term.
  - *The report's own concession.* It says the README sentence "reads against this rule" and "needs a word". That is a second ratified sentence to change. It was written on PR 6608's fix pass (`47e9bbfde`, 2026-11-09); I did not check whether Ev ruled on it.
  - *Its move handling.* "Re-mint at the arc, or slack, left to the lane" is not a detail. It decides whether the form is a function of the geometry.
- **Against size alone (the second report).** Only the small cone section, as in the Ev section.
  - `geom_brep::intersect`'s tilted-ellipse arm computes `ConeTilt::near_vertex` ("finite toward the parabola"). Its comment says the centre formula "reads a vector that cancels to K … and then divides it by K".
  - So under size alone that mint either goes back to the ill-conditioned centre or stores `vertex − toward·a`.
  - I did not measure the `Interval` widths. The brief's "`3193e7c97` is narrower at `Interval` than the centre form on the kernel's cone sections" gives the direction, not the size.
- **One claim in the second report I think is wrong.** "If losses appear at ρ of 10 to 100 m … anchor everything would then be right despite its `Interval` cost."
  - Phase 1 (a) measured 40 of 40 with the point read as chords from a nearby point summed from the stored centre, with no stored form changed.
  - So a graded in-box loss is answered at the reader. It does not reach the stored form unless that reading also fails.
- **A minor point in both.** Spelling the (0, 0) offset as `cos θ` "so centre-stored conics keep main's bits" is bit stability, which is not a reason. The reason that counts is the symbolic one: `sin²(θ/2)` and `cos θ` are different opaque atoms.

**For the lane's pending measurement**
- Add one row: cone sections with a ≤ 1e3, some near-parabolic, at `Interval`, stored at the vertex against stored at `vertex − u·a`. That row decides between size alone and the mint's point.
- All three criteria share one open detail: a parameter box across which a conic's size (or centre) crosses the limit builds two kinds of stored point. The limit has to be read so that either side is sound near it. Nobody has specified this.

**Not checked**
- Nothing was run. This round I read only the two named reports, plus the tree for the citations above.
- The near-parabolic estimate (width of `a` about a·u/K, so about 1e-8 m at a = 100 m and K = 1e-5) is by hand.
- The 9 u·box corner for a ≤ box is by hand.
## For Ev

**Recommendation (changed; likely).** No boundary image stores its ends. Each straight image (`IsoLine`) and each arc image (`IsoArc`) holds two things:
- a **chart line**: the coordinate it keeps fixed, named as one of the chart's own knot lines (`Knot(k)`), or, where it was measured, as a value (`Measured(T)`);
- one **moving map** along that line over the edge's parameter.

The two variants stay separate. They share one `ChartLine` type and differ only in their moving map: affine for a line, the rational-arc parameter for an arc.

A **corner** is never read off an image's end. The loop's lift reads it as the **meet** of two consecutive chart lines, `(column line, row line)`. The *lift* is `topo::pcurves::Lift`, the ratified one place for a loop's position in its chart. A *joint* between two images on one line, such as a split's cut, is not a corner, and nothing asks it to be exact.

Tier 3's rectangle certificate is check 7's cheapest volume proof on a NURBS wall. It reads the loop as runs of images on one line and requires:
- exactly four runs, alternating rows and columns;
- every line a point value;
- every piece in a run moving the same way, decided at the band.

The four meets are the rectangle, and their cyclic order is the winding. Every other reader that wants a corner reads the same meet from the lift. Those readers are the chart-region entry, the spline trim, the boolean's chart corners and `replace_face`'s row read.

*Terms.* An *image* (pcurve) is an edge's curve in a face's 2-D chart. A *knot line* is a line `u = k` or `v = k` at one of the surface's stored knots; the domain's sides are its first and last knots. An *edge's span* is its parameter interval `[t0, t1]`, where `t1` is a computed length.

**Why a name and not a value (the change from my first answer).** I proposed storing the fixed coordinate as an exact value, `at: T`. A name is stronger, in two places I checked:
- **It is stated once.** A knot line's value already lives in the surface's knot vector. Storing `at` copies it, which is the same duplication this fix removes, only moved.
- **It survives a period shift.** On a chart closed in one direction, the lift moves an image by whole periods: `at + k·period` (`shift_v_branch` and the `u` twin both use `translate`).
  - On a kernel-built `[0, 1]` chart this is exact.
  - On an imported chart, for example a domain `[0.1, 0.7]`, `0.1 + (0.7 − 0.1)` need not round to `0.7`, even at `f64`. A copied value then misses the chart's own side, so the certificate would refuse or misread it.
  - A name has an identity instead: on a closed direction, the first knot shifted by one period *is* the last knot. That is structure, not arithmetic.

Other chart maps never reach these images. Reflection leaves a spline chart alone (`transform.rs`'s `Net` class). `mirror_v` runs on plane charts only, and the offset's `shift_chart_v` runs on cones only. So the period shift is the case that decides it (sure; read at the sites).

**What I keep from my first answer.**
- **Corners as meets, read once at the lift, for every reader** (likely).
- **A `Measured` line still works where it is a point** (likely). A rectangle bounded by an interior column taken from a measured foot keeps working at `f64`, as it does today, and refuses at `Interval`, as it does today. So the certificate takes any point-valued line, not only knot lines. This is my remaining difference with the knot-tag proposal: it would refuse every `Measured` line outright, or keep them only as a fallback.
- **Merging the two image kinds into one variant: I withdraw this.** A shared `ChartLine` type gives the same single statement. The moving maps really do differ, and readers match on them.

**The other answer: each image stores its row by its two ends, with a kind per end, `Row` (the row's end) or `Inside` (a cut).**
- **Accepted: a measurement.** A split `Interval` loft, finished before any re-mint with both kept corners exact, still refuses, and the refusal is at the cut. The certificate tests both ends of every image. So the certificate change is needed whatever is stored (sure, measured), and it can land first.
- **Accepted: no rounding slack.** The certificate's hand `4·ε` end slack goes once cuts are not read. The extra term from a corner's joint gap is second order, and the lift already decides that gap at the band (likely).
- **Answered: "the ends are the description, the map their derivative."** The mints do compute the map from two points and a span. But of those points, only the corners are exact facts, and a corner is a fact about two lines, which the chart's knots already state. The other ends, a cut or a measured foot, are measurements however they are stored. Storing them keeps the image tied to its span, and keeps the `Row`/`Inside` bits stored. Those bits must change at every split and join. The loop already says the same thing: a joint is a corner exactly where its two images' axes differ.
- **Answered: `49·(1/49) ≠ 1` at `f64`.** True, but no reader of this design reads an end as a corner at any scalar, so it does not bite.
- **Answered: "other readers match an end to the domain end exactly."** They read the meet from the lift, which compares a name or point values exactly.

**A worked case: a split rim on an imported closed wall at `f64`, domain `u ∈ [0.1, 0.7]`.**
- *Rows by their ends:* a kept end is `Row` with `to = 0.7`. Across the seam the lift shifts it to `0.1 + period`, which may not equal `0.7`, so the corner test misses.
- *My first answer (`at: T`):* the same miss on the seam column.
- *The knot tag:* `Knot(last)`, whose shift is `Knot(first)` by identity, so the corner is exact.
- *All three:* the cut is not read.

**Ratified text.** None binds the stored ends; they are documented only in code (sure). The `[one-way.places]` row for "a loop's position in its face's chart" would gain "every joint's chart point (a corner as the meet of two chart lines) and the rectangle with its winding". That is wording that lands with the change, not a new decision (likely).

**Confidence.** Recommendation: likely. "A knot name survives a period shift where a copied value may not": sure for the arithmetic; I have not checked that a shipped body reaches an imported closed spline wall at rest. "The certificate change is needed for any split": sure, as measured in the other report.

## For the orchestrator

- **Now in agreement with B,** with one remaining difference: whether the certificate takes a `Measured` line that is a point value (I say yes, in one rule, not as a fallback; B leans to refusing it or keeping it only as a fallback). Whether any construction mints such a rectangle today is unchecked. The candidates are the measured arm's `f0.x` column and a cap rim that an offset moved inward.
- **Required by the tag:** `ChartLine::Knot` must state the period identity, first knot + period = last knot, on a direction the chart decides closed. Otherwise B's tag reduces to my value through the lift's `translate`.
- **Sites read this round:**
  - `topo/src/transform.rs`: reflection is `OrientationClass::Net`, the identity on a spline chart.
  - `geom-brep/src/certify.rs`, `with_chart_v_mirrored`: plane charts only.
  - `topo/src/replace_face.rs`, `shift_chart_v`: the cone offset.
  - `topo/src/pcurves/mint.rs`, `shift_v_branch`: `translate` by `period.times(k)`.
- **Off the question:**
  - Amend `a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map`: the cut refuses first (A's probe).
  - `chart_region.rs`, around line 4339: a diagonal `IsoLine` fixture, which the new type would make unwritable.
- **Sequencing (A's, which I endorse):** change the certificate first, then the chart-line type, then the arc.

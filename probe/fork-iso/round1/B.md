Reconciliation round 1 — report B

## For Ev

**Recommendation now (likely): changed. I now agree with C's form; B's tag (below) does not earn its place.** An iso image holds no ends. It holds the line it keeps fixed, as an exact value, and one map along that line. The straight and arc images become one variant, `Pcurve::Iso { held, at, moving }`:
- `held` is the chart axis kept fixed: `v` for a row, `u` for a column.
- `at` is its value, an ordinary scalar.
- `moving` is the other coordinate as a function of the edge parameter: affine for a line, the rational-arc parameter for an arc.

A loop's corner is read from the loop's lift (`topo::pcurves::Lift`, the one place for "a loop's position in its face's chart") as the **meet** of two held lines: `(column.at, row.at)`. It is exact wherever both held values are points. The volume certificate's rectangle test becomes three discrete checks:
- the loop is iso images only;
- grouped into runs on one held line, there are four runs, alternating row and column;
- the four meets are the corners, and their order is the winding.

A split's cut is a joint inside one run, and nothing reads it as a point.

Terms:
- *Image*: an edge's curve in a face's 2-D chart.
- *Joint*: the vertex where two consecutive images of a loop meet.
- *Run*: consecutive images on one held line (a split rim's pieces).

### Where the three designs stood
All three delete the stored `ends` and `StoredSpan` added by PR 6959, and all three make the certificate read lines and corners instead of eight end points. The split was over what an image holds:
- **A:** the row by its two end values plus a per-end kind, `Row` (this edge end is the row's end) or `Inside` (a cut), with the map derived from them.
- **B:** the held line as a tag naming one of the chart's knot lines.
- **C:** the held line as an exact value, with line and arc merged.

### Why B's tag goes (sure)
Checked at each site that would mint or carry one:
- **No mint knows a knot index.** The importer (`step-import` `adopt.rs`), the mint's boundary rows (`cv0`/`cv1` in `nurbs_iso_derive`) and the loft's section rim all read the chart's *domain end values* and pass them on. Those values are already exact `f64` copies, so an exact `at` is what they have.
- **A knot index goes stale.** Knot insertion renumbers the knots, so an interior `Knot(k)` would need re-mapping at every re-knot. Only "first/last" would survive, and that is just "the domain end", which the exact value states.
- **The chart moves leave both forms equal.**
  - `shift_chart_v` moves `v` only on analytic charts, where a knot tag means nothing; on a spline chart the shift is zero.
  - A `Deck` shift (the lift moving an image by whole periods of a closed chart) adds `k·period` to `at`. That is exact for a dyadic period, and an honest enclosure otherwise. A tag would have to carry the same `k` and read the same sum.
- **`Measured(T)` was the giveaway.** B needed it for a column at a measured foot, so B had two kinds of held line, where C has one value that is a point or is not. "Is it a point" is the same structural read the certificate already makes today.

So the tag makes nothing unrepresentable that C's exact value leaves open, and it adds a second kind.

### What moved me in A, and what did not
- **Accepted from A:**
  - The measurement that the as-split `Interval` loft, with both kept corners exact, still refuses at the cut. That settles that the certificate has to change in any design.
  - The `f64` point, `49·(1/49) ≠ 1`. Today an exact corner is luck even at `f64`. Under the meet it never depends on a map's end at any scalar.
- **Answered:** A argues that "the ends are the description, since every mint builds a straight image from two end points and a span."
  - Mints do pass two ends. But one of the two coordinates of each end is the held line, and the other coordinate, at a corner, is the *adjacent* image's held line.
  - Storing ends therefore states each corner twice, once per image, and that is the duplication we are removing.
  - A's per-end kinds then become a new thing kept in step by hand. A split marks the cut `Inside`, a join restores the kinds, and a re-anchor (moving an edge's end vertex) has to decide whether the moved end is still `Row`. The meet needs no such decision: a joint is a corner because its two images hold different axes.
  - A's re-mint also needs a new per-end candidate schedule (boundary value or measured foot). Under the meet it needs nothing: the mint already offers the boundary rows first, so a re-minted child keeps an exact held line, and its measured free end is never read.
- **Readers outside the certificate** (A's other argument) match an end to the chart's domain end exactly: `boolean::spline_known`'s chart corners, `chart_region::pcurve_entry` and `replace_face::iso_image_row`. They read the meet from the lift instead, which is an exact comparison of a held value against the chart's domain. That is better than today's comparison of a map's end.

### Worked cases
- **Split, then re-minted** (`Interval`). Bottom rim `v = 0`, split at a third.
  - Under C, both children hold `v = 0`. The run is `v=0, v=0`, followed by `u=1`, `v=1`, `u=0`. The corners are `(0,0)` and `(1,0)`, read as meets.
  - The re-mint replaces each child's map with a measured one but keeps `at = 0`, because the boundary row is offered first, so nothing changes for the certificate.
  - Under A, the re-minted child needs the new per-end schedule to state its kept corner as `Row` at exactly `1`; without it, it refuses.
- **Arc rim.** `IsoArc` today holds `p0`, `pd` with a held coordinate that `pick_arc_rim` mints exact. Under C it is `Iso { held: v, at: 0, moving: Arc{…} }`. Its corners are meets, and the width its end picks up through `tan` at `Interval` is never read. A needs the kinds on the arc as well.
- **Sub-rectangle at an interior column measured at `Interval`.** `at` is an enclosure, the meet is not a point, and the certificate refuses honestly, as today (C, and A alike). At `f64` it is a point, and quadrature over that sub-rectangle is sound.
- **What remains possible:** an `at` that the carrier does not follow. Checks 3 and 4 refuse it, as today.

### Ratified text
The `[one-way.places]` row for the lift would gain "every joint's chart point: the meet of two held lines between a row and a column, the map's value otherwise". Its provenance should be checked with `git log --all -S'A loop'` before it is treated as yours to sign. No ratified text binds the stored ends.

**Confidence:**
- Recommendation (C's form): **likely**.
- "A tag adds nothing over an exact value": **sure** (sites read).
- "Meets need no per-end schedule on re-mint": **likely** (read from the mint's candidate order, not run).
- "The certificate must change in any design": **sure** (A measured it).

## For the orchestrator

- **Remaining difference with C:** none on substance.
  - Small points for the unit: the run's pieces' rates should be decided as one sign (C's "not checked" item), so a back-and-forth run refuses in the certificate rather than in tier 3's simplicity check.
  - `LiftedRow`'s span should come from its cache, behind private fields.
- **Remaining difference with A:** whether corners are data per image (A) or a lift-level meet (B/C). If Ev prefers A, its kinds should at least be derived at the holder from the span, never stored beside it.
- **Sequencing** (A's note, which I agree with): the certificate change can land first, independently, and makes the as-split `Interval` loft pass even with 6959's ends in place.
- **Not checked:** `Sym<Interval>` keeping a domain-end constant as a point.
- **Checked:** C's diagonal `IsoLine` (`chart_region.rs` ~4337) is a test fixture (`iso_line_and_iso_arc_entry_points_are_exact`), to be re-spelled axis-aligned with the change; no production spelling is lost.

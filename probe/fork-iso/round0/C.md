## For Ev

**Recommendation (likely).** An image's ends should not be stored at all. Where an exact corner exists, it is the point where two straight images meet, not a point either image stores. Concretely:
1. Drop `ends` from `Pcurve::IsoLine`, and with it `Pcurve::span_ends` and `StoredSpan`.
2. Restate the straight image so that it is axis-aligned by its type. Merge it with the arc image (`IsoArc`) into one variant: `Pcurve::Iso { held: ChartAxis, at: T, moving: IsoMove<T> }`.
   - The *held line* is the chart coordinate the image keeps fixed (`v` for a row, `u` for a column) and its value `at`.
   - The *moving map* is the other coordinate as a function of the edge parameter `t`: `IsoMove::Line { at_zero, rate }` (affine) or `IsoMove::Arc { … }` (today's rational-arc parameter, with its weights field renamed from `ends` to `weights`).
3. Any reader that needs to know where a loop sits at a joint reads it from the loop's lift. The *lift* is `topo::pcurves::Lift`, the chain of a loop's chart images, already the ratified one place for "a loop's position in its face's chart". The lift reads a *joint* (the vertex where two consecutive images meet) as follows:
   - **between a row and a column:** the *meet* `(column.at, row.at)`. This point is exact because both held values are exact by construction;
   - **between two images on the same held line** (a split's cut): the moving map's value there, an enclosure;
   - **anywhere else:** the image's value there, read through its map.
4. Tier 3's rectangle certificate (check 7's cheapest volume proof on a NURBS wall) stops reading points. It reads a discrete fact off the held lines:
   - the loop is iso images only;
   - grouped into maximal runs on one held line, there are exactly four runs, alternating row and column;
   - the four meets are the rectangle's corners, and their cyclic order gives the winding.
   Each image's end lies within its joint's chart gap of the meet. On a spline chart the lift already decides that gap as Zero within ε (`spline_gap_closes`). The certificate's existing per-end slack takes that bound in place of the envelope term PR 6959 added.

**Premise check.**
- *"An end its construction placed on the chart's edge"* (sure). An image's end is its edge's end. It is read at `t1`, which is a computed length, so it is inexact by nature at `Interval`. What the construction makes exact is each image's **held line**: a rim is `v = 0`, a seam is `u = 1`. A corner is the meet of two of those. Storing ends restates, per image, a fact that belongs to the pair.
- *"The certificate may need a discrete fact, not two points"* (sure). Yes, but the fact is derived from the held lines above, not recorded.
- *"For a `Chart` edge the pcurve is the description"* (sure). True, but it does not decide this question. A cap–wall rim is an `Intersection` edge, so its image is a cache. A wall–wall seam on one surface is `Chart`, so its image is the description. The answer is the same for both.

**What this makes true (final state).**
- Nothing is stated twice (sure). The image is its held line plus its map. The ten hand-kept sites (`split_at`, `with_end_at`, `over_span`, re-anchor, `map_chart`, `shift_chart_v`, the lift) have no ends left to keep in step, because a span change moves no held line. The chart maps are per-axis, so they keep the held axis.
- Bad states cannot be written (likely).
  - A diagonal "iso" line has no spelling. Today the quad lane and `replace_face::iso_image_row` refuse one at run time; the production mints I found are all axis-aligned.
  - Pairing an image with another edge's span becomes unwritable, since no reader asks an image for its ends. `LiftedRow` should take its span from the cache, behind private fields.
- One check per statement (likely). Check 3 reads the map at the end stations. Check 4's envelope loses its "stated end off the map" term, so a freshly minted `Interval` image is back to zero there. The domain gate is already a banded decide over the map and needs no stated ends.
- One reading for every reader (likely). Today five readers each read image ends themselves:
  - the rectangle certificate;
  - `chart_region::pcurve_entry`;
  - `spline_trim::image_pieces`;
  - `boolean::spline_known` (which chart corner a vertex is at);
  - `replace_face::iso_image_row`.
  All five read the joint from the lift instead. `spline_known`'s corner test becomes an exact comparison of a meet against the chart's domain.
- **The arc image (`IsoArc`):** cured by the same rule (likely). Its held line is exact (`pick_arc_rim` mints its direction as `(±1, 0)`), so its corners are meets, and whatever its map reads at `Interval` no longer matters. The parked arc-rim row becomes moot.
- **The split rim:** cured (likely). The re-mint's measured arm offers the boundary rows `[cv0, cv1, …]` first, so a child keeps an exact held line. Its kept corner is a meet with the seam column, and its cut is a same-line joint, which the certificate does not read as a corner. The question of whether `mint_pcurves` should keep carried rows goes away.
- **What stays refused, correctly:** a rectangle whose held line was measured, such as an interior column taken from a foot at `Interval`, is an enclosure, not a point, and the certificate refuses it as it does today.

**Alternatives weighed.**
- **B. Store the two placed ends as the only statement**, deriving the map from them and the span (rejected; sure). This removes the duplication inside the image, but:
  - the image then depends on its span, so every span change re-states an end, and the pairing hole stays;
  - the measured re-mint has no exact corner to state, so the split case stays broken;
  - the arc image needs its own copy of the mechanism.
- **C. A face-level record, "this trim is its chart's patch"**, set by the loft (rejected; sure). It records what can be derived, goes stale under a split or a boolean, and restates the construction (`[intent.statements]`).
- **Reversibility.** Option A is easy to back out locally: a stored end could be added back on the merged variant. B would be harder to undo once the map is derived from the ends.

**Ratified text to change (likely; it waits for you).** The `[one-way.places]` row "A loop's position in its face's chart … every window, hull and winding a method on it", and the matching row in `crates/topo/README.md`, would gain: "and every joint's chart point: the meet of the two held lines between a row and a column, the map's value otherwise." The geom-brep README has no stated-ends text to retire; the stated-ends prose is in code docs only.

**Confidence.** On the recommendation: likely. The unmeasured link is that every `Interval` loft's boundary rows and columns hold point-valued held lines on the probe fixtures. That is read from the mints (`iso_row`/`iso_column`, the measured arm's `[cv0, cv1, f0.y]`, `describe_section_rim`), not run.

## For the orchestrator

- **Assumed, not measured:**
  - `pick_arc_rim` gives `IsoArc` an exact held channel, i.e. `pd = (sign, 0)` with `p0`'s held coordinate a domain end. Read at `edge_nurbs.rs` ~1366.
  - `describe_section_rim` and `step-import/src/adopt.rs` mint held lines as exact domain ends. The importer chooses its line by a banded selection, so its value is a domain end too.
- **Not checked:**
  - What `rectangle_winding` needs beyond corners. Under A, each run should also have its pieces' rates decided as one sign (a definite `1/L`), so a back-and-forth run is refused rather than left to tier 3's simplicity check.
  - Whether `Deck` shifts of a held `u` by a non-dyadic period keep `at` a point. With period 1 on `[0, 1]` it does; with an imported `3√3` it would not, and the certificate would refuse honestly.
- **Possible off-question defect:** `chart_region.rs` ~4339 constructs a diagonal `IsoLine` (`pl = (0.5, 1.0)`). It is probably a test fixture; if it is production code, it is a defect under any design.
- **Unit shape:** one unit, cost M–L, review FULL.
  - Re-baseline `geom-brep/tests/an_iso_lines_stated_ends.rs` and `sweep/tests/a_lofts_chart_ends_read_as_placed.rs`. The split row's pin (`a_split_rims_cut_is_never_read_as_a_chart_end`) should flip to finishing.
  - Close `an-iso-arc-rims-end-reads-through-its-map-at-interval` and `a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map` with it. The arc row still needs an `Interval` arc loft or a fixture to *show* the cure, and that is blocked at the skin door (`CarrierWeightNotAPoint`), per its own body.
- No provenance search was run on the `[one-way.places]` lift row. Run `git log --all -S'A loop' -- docs/DESIGN.md` before treating it as Ev-ratified.

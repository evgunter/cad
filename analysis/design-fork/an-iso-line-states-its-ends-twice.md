# Design fork: an IsoLine states its ends twice (BOXFINISH)

Protocol: ddd201f411 (`docs/DESIGN-FORK-PROTOCOL.md`). Item: `work/boxfinish/an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends.md` (P1, design).

## Draws (2026-11-15, 02:10 UTC, at dispatch)

| Byte | Value | Meaning |
|---|---|---|
| arm | 58 | below 86: THREE designers (one Fable, two Opus) |
| labels | 117 | not 255; 117 mod 3 = 0: Fable is A; the two Opus designers are B and C |

Mapping: **A = Fable 5.2, B = Opus 5.5 (first Opus dispatched), C = Opus 5.5 (second Opus dispatched)**. Versions as configured at dispatch; confirmed from each session's served model when the reports are in.

Dispatched concurrently as three cloud sessions with the same statement of the problem and the same pointer to `docs/prompts/designer.md`.

## Sessions (dispatched 02:09 UTC, one turn, concurrently)

| Label | Model (configured) | Session |
|---|---|---|
| A | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWU5GkLjNmyBmhNGAtf |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWX43ubJbGeKbqQSmCe |
| C | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXQagXVTSE1tSzNevK |

The statement of the problem (identical for all three) is the orchestrator's brief of 2026-11-15; it names no candidate solution. The item file's "The question" paragraph carries one of the orchestrator's guesses (that the certificate's need "may be a discrete fact and not two points"); the brief tells the designers to treat that paragraph as a hypothesis.

Token counts for a three-designer fork: read from each session's `result` events when the reports are in (first report, and total).

## First reports (all three in by 02:19 UTC; recorded before any reconciliation)

Report times: C 02:14, B 02:15, A 02:19 UTC. None saw another's report.

| Label | Answer | One line of argument | Confidence | Rejected the framing? | Proposed changing ratified text? |
|---|---|---|---|---|---|
| A | An iso image stores its row by its two ends (the map derived from them) with a per-end kind (`Row` or `Inside`, a cut); the rectangle certificate reads four lines and its corners, not eight ends. | Every mint already builds from two end points, so the ends are the description and the map the derivative; only corners need exactness, and a cut never does. | likely | Yes: the question is which ends anything needs exact (corners only); and it MEASURED that the filed split fix would not make a split `Interval` loft pass (the cut refuses). | No. |
| B | Store no ends. The fixed coordinate becomes a structural tag naming one of the chart's own knot lines (`Knot(k)`, or `Measured(T)`); the moving coordinate stays one map; the certificate reads the side tags off the lift. | The certificate needs which side each image is on, a discrete fact the chart's knot structure states exactly; "there" is a name, not a value. | likely | Yes: the certificate never needed exact ends, only exact sides; the split case is not cured by stored ends (read, not run). | No (a re-wording of the lift's `[one-way.places]` row lands with the change). |
| C | Store no ends. One variant `Pcurve::Iso { held, at, moving }` for line and arc; a corner is the meet of two held lines, read from the lift; the certificate reads four alternating runs of rows and columns. | What the construction makes exact is each image's held line; a corner belongs to the pair, so storing ends restates per image a fact of the pair. | likely | Yes: an image's end is its edge's end and is inexact by nature; the exact thing is the held line. | Yes: the `[one-way.places]` lift row (and `crates/topo/README.md`'s) gains "every joint's chart point". |

Agreement, judged on the final state: all three delete PR 6959's `ends: Option` and `StoredSpan` and change the rectangle certificate to read four lines and their corners. They split on what the image stores: A keeps the two ends as the description, with kinds; B and C keep the held line and one map and no ends; B and C differ on whether the held line is a tag (B) or an exact value (C).

### A, first report, verbatim

````markdown
## For Ev

**Recommendation** (likely). Two changes, which only work together.

1. **An iso image stores its row by its two ends, and says of each end of its edge whether it is the row's end.** A *row* is a straight, axis-aligned segment of a face's chart: which coordinate is held fixed and at what value, the moving coordinate's two end values `from` and `to`, and the carrier interval `(ta, tb)` those two are attained over. The map between is derived, `from + (to − from)·(t − ta)/(tb − ta)`; no `p0`, no `pl`, no separate `ends`. Each end of the edge the image is stored on carries a two-valued *kind*: `Row` (this end of the edge is the row's end; its chart point is `from` or `to`, read as data) or `Inside` (a cut; its chart point is the map at the span's end). The straight image and the arc image hold the same row and the same kinds, and differ only in how the row's parameter follows the carrier's (affine for a line, the rational-quadratic map for an arc).
2. **Tier 3's rectangle certificate asks only what it needs.** Today it demands that every end of every boundary image read as an exact number. It needs: the four fixed values (they are the rectangle), that at each corner the two meeting ends are of kind `Row` and each row's end value is exactly the other line's fixed value, and that pieces along one side run the same way. A cut inside a side is never read exactly.

**Premise corrections.**
- *The filed split fix would not make a split `Interval` loft pass.* The row says the loft refuses because a re-minted child reads its kept corner through its map. Measured (scratch probe on `a_split_rims_cut_is_never_read_as_a_chart_end`, finishing before the re-mint, when both kept corners read exactly `0` and `1`): `f64` finishes, `Interval` refuses with the same "not exact structure". The cut is refused, because the certificate applies its exactness test to both ends of every segment (`quad_lane::nurbs_face`, the `exact` closure). Sure.
- *The question is not "what does the image store so an end reads exact" but "which ends does anything need exact".* Corners only. The certificate's soundness never rested on exact cut points: each image is certified within its envelope of its edge, the certificate already pads by that envelope times the boundary length, and consecutive images share a vertex. Likely.
- *The map-only form was fragile at `f64` too.* A rim of length `L` is minted with slope `1/L`, and `49·(1/49)` is not `1` in `f64`; the certificate compared that product to the chart's `1` exactly. The arc image carries a clamp in `iso_arc_g` whose stated reason is this same comparison. So "where the end is" has to be data at every scalar, not only at `Interval`. Sure of the arithmetic; unsure whether a shipped `f64` body ever hit it.
- *Every straight image in the tree is already built from two end points and a span* (`Pcurve::iso_row`/`iso_column`, the measured-feet arm in `nurbs_iso_derive`, the importer's row, `describe_section_rim`); `p0` and `pl` are computed from them at every mint. The ends are the description and the map the derived form; PR 6959 stored the description beside its own derivative. Sure.

**Ratified text.** None changes. `docs/DESIGN.md` and the two README pages do not mention stored ends (searched "stated end", `span_ends`, `iso_row`, `StoredSpan`); `[edges]` and the `[one-way.places]` rows stand, and this serves them: for a `Chart` edge the image is the description, and it then states the locus once.

**What the recommendation makes true.**
- Nothing is stated twice: a `Row` end is `from`/`to`, which define the map. The envelope term for "an end off its own map" is deleted, not padded; check 3's end stations and check 4's envelope certify one thing.
- `Option` and its four states go. Placed by a mint, selected in band by the importer, and measured are all "a row with stated ends", exact or wide as their values are; "forgot" has no spelling, since a mint must give both ends. The only other state is `Inside`, made only by cutting.
- A split keeps the row and marks the cut `Inside` on each child; a join takes the outer kinds back; a chart map moves the row once. No end is re-read through a map anywhere. What is left to keep in step is two bits that change only when the span does, so span, image and kinds change through one method on their holder (the cache, the edge's certified curve, the lift's row), and ends are asked of that holder. `StoredSpan` goes, and a foreign span cannot be paired with an image.
- The three cases are one rule. *Straight*: whole rows read exact corners. *Split, as carried*: corners exact, cut `Inside`, the certificate does not ask for it. *Split, re-minted*: the mint's schedule offers, per end, the chart's boundary value or the measured foot, judged by the one probe it already uses; a child is then a row from an exact corner to a measured foot, both `Row`, and the certificate reads the corner exactly and the foot not at all. *Arc*: `IsoArc` already holds a whole row (`p0`, `pd`, `t0`, `angle`) and reads its ends by evaluating; it gains the kinds and reads a `Row` end as data, and the clamp loses its reason.

**Worked example.** A wall whose bottom rim (length `L`, an enclosure) is split at `L/3`. Before: row `v = 0`, `u: 0 → 1` over `(0, L)`, kinds `[Row, Row]`. After: both children hold that same row; kinds `[Row, Inside]` and `[Inside, Row]`. The certificate reads the lines `v = 0`, `u = 1`, `v = 1`, `u = 0`; at the corner `(0, 0)` the first child's `Row` end `0` equals the left column's fixed `0`; at the cut it reads nothing but that both children run toward larger `u`. Re-minted, the children become `u: 0 → [0.333…]` and `[0.333…] → 1` with all kinds `Row`: same reading.

**What it leaves possible.** A kind saying `Row` on an edge whose span end is not the row's end is still writable inside the holder's own method; check 3 refuses it beyond the band, as today. A face whose fixed values are not exact numbers (a sub-rectangle at a computed column) still refuses the rectangle lane and belongs to the trimmed lane.

**Alternatives.**
- *Less: delete the stored ends, change only the certificate, corners checked by containment.* Sound by the same envelope argument, and the smallest final state. I lean against: other readers match an end to the chart's domain end exactly (`spline_known`'s chart corners, `chart_region::pcurve_entry`, `iso_image_row`) and would go back to rounding luck at `f64` and miss silently at `Interval`. If Ev judges none of those matter at a box scalar, this is defensible. Reversible either way.
- *Two-point form anchored on the edge's own span, children re-anchored at a split, no kinds.* Simplest for lines, but the image then cannot be evaluated without its span, and an arc cannot follow (a part of an arc row is not an arc row of the same form), so arcs would need the kinds anyway and the two images would differ. Rejected for that.
- *Corner positions held by the face or its lift.* A third statement of what two images' ends already say. Rejected.
- *Keep PR 6959's shape and extend it to the arc.* Rejected: it is the duplication, and it does not fix the split.

**Confidence.** Recommendation: likely. Certificate change is required for any split case: sure (measured). Corner-only exactness is sound: likely (argued from the existing pad, not proved in code). One row type serves line and arc: likely. The re-mint offering boundary-or-foot per end certifies without moving `f64` rows: unsure, unmeasured.

## For the orchestrator

- **Brief errors.** (1) `a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map` misdiagnoses: the as-split body, before any re-mint, already refuses at `Interval` (probe output: `PROBE Interval as-split finish: Some(NotFinished … "a NURBS-face pcurve endpoint is not exact structure…")`, `f64` `None`). Its "Measured" list never finished the as-split body. Amend the row; its remedy alone closes nothing. (2) `Pcurve` lives in `crates/geom-brep/src/pcurve_cache.rs`, not `pcurve.rs`.
- **The probe.** I added three lines to the test locally (a `topo::finish(body.clone(), …)` after `read(&body, "as split", true)`), ran it, and reverted with `git checkout`; the tree is clean, nothing pushed.
- **Not checked.** Whether any boolean or chart-region path runs at `Interval` today (decides how much the "less" alternative loses). Whether `Sym<Interval>` keeps a placed constant as a point (I assumed yes, since `exact` goes through `Interval::from_certified`). Whether the mesher or the lift relies on `iso_arc_g`'s clamp for evaluation off the ends; I only claim its stated reason goes. I did not open PR 6959's review thread, only its rows and the code.
- **Assumed.** `replace_face`'s Chart-edge image `(fixed, ±t)` (carrier is the chart's own row, slope exactly ±1) fits the row form as `from = ±ta, to = ±tb`; it wants a named constructor so that case does not spell its span twice by hand.
- **Shape of the work, if ratified.** One unit can land the certificate change alone first (it is independent of representation and makes the as-split `Interval` loft pass with 6959's ends in place); the row type, the kinds, the holder-owned span and the mint's per-end schedule follow, then `IsoArc`. That is sequencing, not a fork.
- **Off the question.** `rectangle_winding`'s join test uses a hand `4·ε` rounding slack (`end_slack`), which `[simple.conditioned]` says nothing should charge; under the recommendation it has no reader (joins inside a side are not read, corners are exact) and goes. `nurbs_face` docs say "exact-structure `0`/`1`" while the code takes any exact rectangle; rewrite with the change.
````

### B, first report, verbatim

````markdown
## For Ev

**Recommendation (likely).** Stop storing ends at all. What the volume certificate needs of a loft wall's trim is not two exact points per boundary image but a *discrete* fact: each image lies on a named line of its chart (a side of the chart's domain), and the loop visits the four sides once, in order. Make that the stored fact: the fixed coordinate of a straight (or arc) boundary image becomes a **chart line** — a structural tag naming one of the chart's own knot lines (the domain's two ends in each direction among them), exact `f64` structure at every scalar — instead of a `T` value. The moving coordinate stays one map over the edge's parameter, and nothing else is stored. The rectangle certificate reads the side tags off the loop's lift (the loop's one position in its chart), takes the rectangle as the chart's own domain (or the four knot lines named), and reads the winding from the order the sides are visited. No image's end is ever read as a corner, so no end has to be exact, and there is nothing to keep in step.

Terms: *image* = an edge's curve in a face's 2-D chart (a pcurve). *Fixed / moving coordinate* = for a row `v = c`, `v` is fixed and `u` moves. *Span* = the edge's parameter interval `[t0, t1]`. *Lift* = `topo::pcurves::Lift`, the loop's position in its face's chart (DESIGN `[one-way.places]`).

### Premise check
- **The certificate never needed exact ends, only exact sides** (likely). `topo::props::quad_lane::nurbs_face` reads every end as an exact `f64` point, hulls them into a rectangle and walks it. But a corner is the meeting of two sides, which the chart already states exactly (its knot vector is `f64` structure); reading it off a free end is reading through arithmetic what the structure says outright. The stored ends of PR 6959 were a second copy of the corner made to survive that read.
- **The split case is not cured by stored ends either** (likely, read from code, not run). The rectangle certificate applies its exactness test to *both* ends of every image, including a split's cut. At `Interval` the cut's moving coordinate is the map read at a computed parameter, an enclosure with width, so a split `Interval` loft refuses at the cut even where both kept corners read exact. Under the recommendation the cut is on the same side as its sibling, and its moving coordinate is never asked to be a point.
- **The arc rim dissolves the same way** (likely). `IsoArc`'s map `p0 + pd·g` lies on the side whatever `g` reads; only its fixed coordinate must be exact, and a tag makes it so. Its end-through-`tan` width stops mattering. The arc and the straight image get the same treatment, and neither gets stored ends.
- **The root of "`t1` is a computed length"** is that a rim's span is its sketch line's metres (`sweep::swept::placed_segment_spec`, `param_end = distance`), shared by the wall image. That is a legitimate choice for the shared parameter; it only bites because the certificate reads geometry (where the image ends) that should be structure (which side it is on). Re-parameterising rims by the chart's own `[0, 1]`, as seams already are, would fix only rims described in the wall's chart — not the measured rows of the cap rims, nor arcs, nor cuts — so it is not recommended (likely).

No ratified text stands in the way: the stored-ends prose is PR 6959's code docs, with no ratified clause behind it (`git log -S` over DESIGN.md and the crate READMEs finds none). The change sits inside `[edges]` (the image stays the description of a `Chart` edge, and is now stated once) and `[one-way]` (one fact, one place).

### The final state
- `Pcurve::IsoLine { line: ChartLine, moving: c + l·t }` with `ChartLine` = `{ axis, at }` where `at` is either `Knot(k)` (the chart's own knot line `k` in that axis — a domain side is a first or last knot) or `Measured(T)` (a column at a measured foot, a chart wider than its face). No `ends`, no `Option`.
- `Pcurve::IsoArc` gets the same `line` field for its fixed coordinate; `p0, pd` become the moving coordinate's start and extent along it. Its weight list (now confusingly named `ends`) is renamed.
- Every mint states the tag it already knows: the loft's section rim (`describe_section_rim`, `v = 1`), the mint's boundary-row candidates (`nurbs_iso_derive` already offers the chart's edge rows `cv0`/`cv1` before the measured foot — they become `Knot` tags, the foot `Measured`), the importer's row (its banded selection is the *decision* that the row is the side, recorded as the tag and certified by checks 3 and 4).
- `split_at`, `with_end_at`, `over_span` and the re-anchor become the identity on the image: a child, a moved end or a re-read span changes the span, not the line or the map. `shift_chart_v` and `map_chart` map the tag with the chart (a re-knotted chart keeps its sides). `StoredSpan`, `span_ends`' stated arm and the envelope's end-off-map term are deleted; at `Interval` a fresh image's envelope pads nothing it does not measure.
- The rectangle certificate becomes a method of the lift: every image on a `Knot` line, the lines four distinct ones, the side sequence one cycle around them (split pieces repeating a side consecutively), each piece's moving direction agreeing (a sign decided at band). The rectangle is the four knot values; the winding is the cycle's direction. Boundary defect is the envelope, as today. Other readers that wanted a corner (`chart_region::pcurve_entry`, `spline_trim`) read it from the lift as the meeting of two lines.

**What it makes true:** a boundary image placed on its chart's edge is known to be there at `f64`, `Interval` and `Sym<Interval>` alike, because "there" is a name, not a value; nothing is stated twice; "none" is unrepresentable (a line is always named); no image can be paired with the wrong span for its ends, since nothing is read from ends. **What it leaves possible:** a `Knot` tag on an image the carrier does not follow — refused, as today, by checks 3 and 4, which read the tag's exact value. **Reversible:** yes; it is a variant's shape and one certificate.

Worked example — the split `Interval` loft: the bottom rim on side `v = 0`, split at a third. Both children hold the parent's image unchanged (`Knot(v, first)`, same map). The loop reads `v=0, v=0, u=1, v=1, u=0`: one cycle, the first side visited by two pieces. The rectangle is the domain; no value is compared, so the cut's width and the kept corner's measured `[0.9999999999999994, 1.000000000000001]` never enter.

**Alternative (unsure it is better; I lean against).** Store the image by its segment — `from`, `to` in the chart — and derive the map from the span it is paired with. One source of truth too, but it still states each corner twice (once per adjacent image), still needs the pairing made unwritable (the image's `eval` then needs its span), still needs the measured mint arm to be offered exact corner candidates, and still refuses the split's cut at `Interval` unless the certificate changes anyway — at which point the segment form buys nothing over the tag.

Confidence: recommendation **likely**; "the certificate only needs sides" **likely**; "a split cut refuses today regardless" **likely** (code read, not run); "the arc case needs nothing more" **likely**; "no ratified text binds the stored ends" **sure**.

## For the orchestrator

- **Brief error / incompleteness.** The `exact` closure in `quad_lane::nurbs_face` is applied to both ends of every image, cut included. `a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map` diagnoses only the kept corner; restoring it alone would still refuse at the cut's width. I did not run it — a one-line probe (finish the split `Interval` body *before* `mint_pcurves`) would confirm, and the row should say so. If that body does finish, my reading of the closure is wrong and the split argument weakens (the main recommendation stands on the duplication alone).
- The rectangle certificate also accepts a sub-rectangle bounded by an interior column with exact `f64` values (variant docs: "a chart wider than the face it trims"). `Measured(T)` keeps that working at `f64` only if the certificate also takes a `Measured` value that reads as a point; I would keep that as a fallback rather than drop the case. Check whether any construction mints one; if none does, refuse it outright.
- I assumed cap rims' wall rows are mint-derived caches (not `Chart` on the wall), from the split row's account; interior section rims are `Chart` on the wall (`describe_section_rim`). Seams already use span `[0, 1]` and exact ends.
- Naming: `IsoArc::ends` holds joint weights, not ends — rename whatever is chosen, it collides with `IsoLine::ends` in every reader.
- Corners for non-certificate readers (`chart_region::pcurve_entry`, `spline_trim.rs`, `boolean/spline_known.rs`, `replace_face.rs` `span_ends` caller) need a lift-level "meeting of two lines" read; I did not check each one's need of exactness, only that each reads through `span_ends`.
- `[one-way.places]` row for the lift should grow "the trim's rectangle and its winding" as a method on the lift if this lands — a re-wording that lands with the change, not a new decision.
````

### C, first report, verbatim

````markdown
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
````


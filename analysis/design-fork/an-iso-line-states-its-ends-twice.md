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

## Reconciliation

- **Round 1, dispatched 02:21 UTC.** Each of A, B and C was sent, in its own session, the other two first reports in full (by label, on the scratch branch `boxfinish/fork-iso-reports`, `probe/fork-iso/round0/`), the orchestrator's one-paragraph statement of where the three stand, and four questions: what moved it; whether its own argument is answered (each pointed at the sites to check); a worked case one design handles and another cannot; its recommendation now. B and C were also told A's measurement (the as-split `Interval` loft refuses at the cut before any re-mint). No further designer was dispatched.
- **Round 1, results (all three in by 02:23 UTC). Every position moved; the three crossed.**
  - **A: changed, to C's first form.** No stored ends; the held line an exact value; one merged variant; corners as meets read from the lift. Moved by: a corner belongs to the pair; its own per-end kinds were a new thing kept in step. It checked its own objection at the three readers and found the meet serves them. It argues against the tag: a lifted column names no knot line, so the tag needs a second kind.
  - **B: changed, to C's first form.** Drops its own tag: no mint knows a knot index; knot insertion renumbers; `Measured` was a second kind of line. Sure that a tag adds nothing over an exact value.
  - **C: changed, to B's first form (the tag),** keeping two points of its own: a `Measured` line that is a point value still certifies, and it withdraws merging the two variants. Moved by: on an imported closed chart a period shift of a copied value need not land on the chart's far side even at `f64`, and a name carries that identity as structure; a stored value copies the knot vector.
  - **What is agreed by all three after round 1:** no image stores ends; a corner is the meet of two held lines, read from the lift; the certificate reads four alternating runs, their meets and one sign per run; the certificate change is required in any design (A measured it) and can land first; no ratified text binds the stored ends; the lift's `[one-way.places]` row gains the joint's chart point.
  - **What is left:** whether the held line is an exact value (A, B) or a name for a knot line with a measured case (C); and whether line and arc are one variant.
- **Round 2, dispatched 02:25 UTC.** Each was sent the other two round-1 reports (scratch branch, `probe/fork-iso/round1/`), the one remaining difference with both sides' arguments stated, and three questions: is the period-shift case real, and what is the shifted value compared with (read and run); is there a form with the name's identity and the value's single kind; its recommendation now and what would change it. The same message to all three. No further designer was dispatched.

### A, round 1, verbatim

````markdown
Report A, reconciliation round 1: I change my recommendation to C's shape (no stored ends, held line as an exact value, one merged iso variant, corners read from the lift as the meet of two held lines).

## For Ev

**Recommendation now (likely): changed.** A boundary image that is a straight row or column of its face's chart stores no ends. It stores the line it holds fixed (which chart coordinate, and its value, an exact number where a mint placed it on the chart's edge) and one map for the other coordinate along the edge's parameter. The straight image and the arc image are one variant differing only in that map. Where anything needs the chart point at a joint (the vertex where two consecutive images of a loop meet), it asks the loop's lift (`topo::pcurves::Lift`, the ratified one place for a loop's position in its chart): between a row and a column the point is the *meet* of the two held values, exact because both are data; between two images on one line (a split's cut) or anywhere else it is the map's value, an enclosure. Tier 3's rectangle certificate reads four held lines, four meets and the order they are visited, and no end.

In the first round I recommended keeping the two ends as the stored description, with a per-end mark saying whether that end of the edge is the row's end or a cut. I withdraw that.

**1. What moved me, and what did not.**
- *Accepted: a corner belongs to the pair, not to either image.* My form removed the duplication inside one image but still stated each corner once per adjacent image, and it needed two bits per image kept in step with the span at every split, join and moved end. Under the meet a split, a moved end and a re-read span are the identity on the image; nothing is kept in step because nothing depends on the span. A cut is not marked; it is the joint between two images on one line. Derived beats recorded here. Sure.
- *Accepted: the re-mint needs nothing new.* My form needed the mint to offer, per end, the chart's boundary value or a measured foot, which I had marked unsure. The mint already offers the boundary rows as the held line before the measured value (`nurbs_iso_derive`, the measured arm's `[cv0, cv1, f0.y]`), so a re-minted child gets an exact held line with no change. Sure of the code; likely that it certifies.
- *Not accepted: the held line as a tag naming one of the chart's knot lines* (the other no-ends report). Answered in 3 below.
- *Answerable:* one report rejects stored ends partly because "the measured re-mint has no exact corner to state". It has one (the chart's boundary value, offered as a candidate); that was not the weakness. The weakness is the bookkeeping.

**2. Is my own argument answered? Yes, checked at each reader.**
My case was that some readers match an end to the chart's edge exactly, and that the map alone cannot give that even at `f64` (a rim of length `L` has slope `1/L`, and `49·(1/49)` is not `1`).
- `chart_region::loop_uv_polygon` (through `pcurve_entry`): it already walks the loop's lift and wants one chart point per joint, as a value, with no exactness test. A meet gives it, more exactly than an end did. Answered.
- `boolean::spline_known::corners`: it wants "which corner of the chart's domain is this vertex at", by exact comparison. Today it reads each iso image's two ends singly. The meet of a row held at a domain end and a column held at a domain end is that corner, exactly, at every scalar. Answered, with one residue: a boundary row that ends at a chart corner where its neighbour is not an iso image (a cut passing exactly through the corner and removing the whole adjoining side) has no meet, and the corner goes unlisted. That reader only collects corners two faces are known to share, so the loss is a missed shortcut, not an unsound answer; and my form's exact end there was only as good as the mint's claim. I do not think the case earns stored ends. Likely.
- `replace_face::iso_image_row`: it wants the image's held axis and value (structural under the merged variant) and the stretch of the row the edge covers, to extract that stretch from a new fit. It reads per edge today; the old face's loop is intact when it runs, so its lift is available. At a meet the stretch's end is exact, which is better than today's map reading for a rim. Answered; it is the one reader that must change from an edge read to a loop read.
- The `f64` fragility: answered. It only bites where an end's computed value is compared with the chart's edge, and no reader does that any more.

**3. Tag against value, worked.** A wall whose chart is periodic in `u`, its loop lifted one period over (the lift moves an image by a `Deck`, a whole number of periods). With the held line a *value*, the moved column is `at + period`, still an exact number when the period is, and the meet with a row is still exact. With the held line a *tag* naming a knot line of the chart, the moved column names no knot line; the tag has no spelling for it and needs its second case ("measured"), which is the `Option` back under another name, with the same question of what a certificate does with each case. A reflected chart (`map_affine`) has the same shape: a value maps through, a tag must be re-indexed by hand. The tree's convention is already that exactness is a value being a single number (`exact`, `exact_zero`), and knot values are `f64` at every scalar, so a placed value is exact without a tag. Likely.
In the other direction, what row-with-ends could do that the meet cannot is only the corner-without-a-neighbour case in 2. I found no other.

**4. What I hold to from the first round.**
- The certificate change is required whatever is stored, and it is measured: a split `Interval` loft whose kept corners both read exactly `0` and `1` still refuses, at the cut (I ran it; the other two reports argued it from the code and are right).
- One variant for line and arc, axis-alignment by type, so a diagonal "iso" has no spelling.
- Pairing an image with another edge's span becomes unwritable because no reader asks an image for ends; the lift's row should hold its span privately.

**Remaining differences with the report I now agree with** (small):
- The certificate should require the pieces along one side to run one way, a decided sign, so a back-and-forth run is refused there. That report lists it as unchecked; I would make it part of the rule.
- A held value that is not an exact number (an interior column taken from a measured foot) refuses the rectangle lane, as today. Agreed by all.

**Ratified text.** One addition, which lands with the change as a re-wording of what the lift already is: the `[one-way.places]` row for a loop's position gains the joint's chart point and the trim's rectangle among the lift's methods. Nothing ratified mentions stored ends.

**Confidence.** Recommendation: likely. Readers are served by the meet: likely (read at the three sites, not run). Value over tag: likely. The soundness of "an image's end is within the joint's gap of the meet" rests on the lift's existing joint decision on a spline chart (`spline_gap_closes`) and the envelope pad; likely, argued, not run.

## For the orchestrator

- **Measured fact for the record:** the probe the tag report asked for exists in my first report: `Interval`, as split, before `mint_pcurves`, refuses with "a NURBS-face pcurve endpoint is not exact structure"; `f64` finishes. Its fallback caveat ("if that body does finish…") does not apply.
- **Checked this round:** `spline_gap_closes` exists (`pcurves/decision.rs`); `loop_uv_polygon` already takes `lifted_images`; `spline_known::images` takes `derived_pcurve_of` per half-edge, not the lift, so it changes source; `replace_face` calls `iso_image_row(&c.pcurve, curve.edge_span())` on the edge's description.
- **Not checked:** whether a spline chart's `Deck` period is always an exact number on kernel-built walls (C flags an imported `3√3`; there the meet is wide and the lane refuses honestly). Whether check 3's end stations, back on the map alone, still certify every `Interval` loft that certifies today: PR 6959 moved those two stations onto stated ends, and before it the pcurve checks passed and only check 7 refused, so I expect yes; one run of the loft suites settles it.
- **Minor, for the implementer:** the affine map stored as value-at-zero plus rate is `u0 − rate·t0`, which cancels for a span far from zero; anchoring it at a parameter in the span is the `[simple.conditioned]` form. Not a fork.
- **On the reports' labels:** C's "alternative B" is my first-round form; its third bullet (arc needs its own copy) is right, its second is not, as said above. Neither changes the outcome.
- **Rows:** the split row's body should be amended (the cut refuses, not only the kept corner) whichever way Ev rules; both parked/open sibling rows close with the unit, the arc one still unshown until an `Interval` arc loft builds.
````

### B, round 1, verbatim

````markdown
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
````

### C, round 1, verbatim

````markdown
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
````


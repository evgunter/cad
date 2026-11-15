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

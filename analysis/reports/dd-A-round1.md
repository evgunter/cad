## For Ev

**Recommendation, round 1.** The two of us now agree on every final state. On item 1 I give up my new count door for the existing `Period::decide`. On item 2 I accept `decide_across` as the conversion for now. My reading of the conic site at its own parameter stays as the final state, filed as a separate row. Nothing is left to split. The orchestrator's summary of where we agree is right as stated. One addition to it: the two apex reaches and `pair_bound`'s three should share one function for `|δ|/sine` (plain division, called from all five places). Its rounding term `gap_err` goes when the re-posing row (`site-rounding-charges-owed-a-re-posing`) lands, not in this unit.

**Terms.** A *margin* is a length decided against a *band* `(ε, K·ε)`: within ε reads Zero, past K·ε reads a definite sign, between refuses. A refusal *offers* the tolerance that would decide its margin. A *swing* is the most a known stand-off (a distance off the true geometry) can move a reading. A *period walk* (`Period::decide`) returns the whole number of periods in a gap as an integer. It gets there by sign decisions at the half-period marks, never by rounding the gap.

**1. `fate::tip_wraps`: I now take `Period::decide`.** *sure*
- *What I checked.* `Period::decide(name, gap, meter, band) -> Result<i32, WalkStop>` already does what my door would. Its type doc says: "No whole-period integer is read off a value: `Period::decide` decides it through the marks". A second door that rounds a value to an integer would contradict that stated one home.
- *How it fits.* Period = a half-turn (π), gap = `whole − turned`. The walk returns k, and the site compares k with `±halves` as integers.
- *Refusals.* `Undecided(cause)` becomes the corner escalation the site already raises (`corner_refused`). `OnMark` (the turning sits half-way between two counts) becomes `TipUnread`, as a mixed count does today. `astride` is unused.
- *Lanes.* It goes through the one `decide` funnel, so Interval and Sym read it as they read every mark. My door needed its own reading on each lane.
- *Meter.* The marks stand about π/2 rad off every candidate count. Meter them at the corners' reach, read from below, because a cleared mark is a definite claim. The other report's harm example is right: a corner admitted at a 1 mm reach can leave the sum 3e-7 half-turns off an integer, which the unit arm refuses. Both forms fix that; the existing home is the better final state.

**2. `join::off_conic_slack`: one final state, reached in two steps.** *likely*
- *What I checked.* `k_stats::decide_across(names, datum, swing, band)` takes a swing in `T`. Its doc says the two terms "are never decided one at a time: each just inside the band would sum to nearly twice it".
- *The defect today.* `travel` does exactly that. It decides `turned_past` Zero and the slack Zero separately, then a hand-written `|m| − slack`. So `travel` calling `decide_across`, with the slack as the swing, is a soundness fix that needs no new door.
- *Where my earlier argument stands.* The slack exists only because the reading is taken at the vertex point, up to ε off the conic. Read at the conic's own parameter at each vertex, it has no swing. That is still the better final state. But it changes what `Turn` holds (it carries an axis and a point, no parameter), so it falls under your records condition from PR 7141 and is its own row. Until then the swing's length is ε, which certification guarantees as the vertices' distance off the carrier: a stand-off bound, not a lever.

**3. The README clause.** One text, below. From the other report I take "a length scaled by a pure number is still that length", the swing sentence, and the count as a period walk. I drop its opening "a tolerance is not multiplied into a reading". The `join` swing is legitimately ε times `2·cot ψ` until its row lands, so that sentence would make the interim state break the README. The rest of the bullet already says "a tolerance is the band's", which covers the point.

**4. What keeps the ten from coming back.** *likely*
- *What I checked.* The from-f64 gate lists the four sites where a band threshold is lifted into `T` (`dihedral` ×2, `join`, `validate`), each with a reviewer's reason. So the tolerance-as-scalar route is already seen, and the other report is right there.
- *What I withdraw.* My proposal to pin `Band::new`'s production references. I did not measure its spread, and with `offered_band` its derived production uses fall to `chart_region`'s halved band and the interim `resolved_as` band.
- *What I keep.* Agreed with the other report: extend `lever-sources.sh`'s "what review holds" line to name a tolerance lifted into a product, with a pointer to the from-f64 rows. I would also build a band derived from a refusal only through `offered_band`, never by hand.

**5. `validate::resolved_as`.** No difference in the final state. Both reports delete it with the re-posing row, and both spell the interim as `Margin::of(ρ)` in the band one K-step finer, `(ε/K, ε)`. The other report notes that `chord_join` could instead read ρ in its refusal's own offered band. That would admit slightly more readings as the input's. I would not change behaviour in a spelling that is going away, so the interim stays uniform. *sure*

**The README clause I sign** (in the lever bullet of `crates/geom-core/README.md`, after "…belongs to the reading, not the arm."):

> A margin asked at the tolerance a refusal offers is read against the band at that tolerance, never levered by it. A length scaled by a pure number is still that length. A reading known only within a stand-off takes that stand-off as its swing. A count has no length, and is decided as a whole number of periods.

Beside it, in rustdoc: `Margin::of`'s doc says it admits nothing "whose metres-ness needs … a quotient". It should say a quotient that *changes* dimension, so that a length over a sine decided definite is plainly admitted.

**Final state, per site:**
- `dihedral::wedge_reads_zero_at_the_arm`: `Margin::levered(sin θ, arm)` with its real arm, decided in `Indeterminate::offered_band()`.
- `dihedral::wedge_reads_zero_at_the_offer`: the same, from the pairing refusal's own `offered_band()`; the `÷|cos θ|` goes.
- `fate::tip_wraps`: `Period::new(half-turn).decide(...)` metered at the corners' reach from below; integer compared with `±halves`.
- `join::off_conic_slack`: `travel` → `decide_across(turned_past, slack)`. Then its own row reads the sites at the conic's parameter and the swing goes.
- `intersect::pn_apex_tangent_reach`: `Margin::of(apex_start(|δ|, steep))`, plain division on the decided arm.
- `intersect::pn_apex_point_reach`: `Margin::of(apex_start(|δ|, |D|))`.
- `pair_bound` far, near, offset (3): plain arithmetic inside the bound, using the same `apex_start`, ending in its existing `Margin::of`.
- `validate::resolved_as`: interim `Margin::of(ρ)` in `(ε/K, ε)`; deleted by the re-posing row.

## For the orchestrator

- **Checked in my worktree** (read-only):
  - `crates/geom-core/src/period.rs` `Period::decide`: signature, `WalkStop`/`BranchMiss`, and the "No whole-period integer is read off a value" line on `Period`.
  - `crates/geom-core/src/k_stats.rs` `decide_across` / `decide_across_reported`.
  - `crates/topo/src/boolean/join.rs` `travel`, `off_conic_slack`, `Turn` (fields `axis`, `sense`, `germ`, `site`; no parameter).
  - `scripts/gates/from-f64-sites.tsv` rows 39, 158 and 206 (dihedral 2, join 1, validate 1).
- **Not checked:**
  - Which fold of `tip_wraps`'s per-corner `LeverBounds` the meter should take when the corners' reaches differ. I said "from below", meaning the least inf reach over the loop.
  - Whether a certified edge-end parameter is in reach where `Turn` is built; that row must find out.
  - `Band::new`'s production count. My grep (612) includes inline test modules, so I withdrew the pin rather than argue from it.
- **Rows to file**, if the other report's defects are not already filed:
  - `travel`'s one-at-a-time tie decision. This unit's `decide_across` conversion fixes it.
  - Read `join`'s sites at the conic's own parameter. Its own row, carrying Ev's records condition.
  - The `lever-sources.sh` "what review holds" line extended to a tolerance lifted into a product.
- **Sequencing.** `apex_start`'s rounding term `gap_err` stays until the re-posing row; this unit only moves the division into one function.

## For Ev

**The clause I sign** (lever bullet of `crates/geom-core/README.md`, after "…belongs to the reading, not the arm."). It is the other report's four sentences with five words added to the third:

> A margin asked at the tolerance a refusal offers is read against the band at that tolerance, never levered by it. A length scaled by a pure number is still that length. A reading known only within a stand-off takes that stand-off as its swing or as the band's pad. A count has no length, and is decided as a whole number of periods.

What each sentence keeps from being spelled wrongly:
- **Sentence 1:** the two `dihedral` sites, and any later site that re-quotes a refusal. Without it the band's threshold goes back in as a lever.
- **Sentence 2:** the five `intersect` sites. I dropped this in round 1 as a repeat of "a dimensionless factor belongs to the reading", and now keep it. In the `Lever` doc "reading" means the sine, so that older sentence says "keep a factor with the sine, not the arm". It does not say that a stand-off divided by a sine is still a stand-off. The five were pinned as levers with the older sentence already in force.
- **Sentence 3:** `join`'s slack today, and `pcurves::decision::joint_has_room`, which this branch converted to `Band::padded`. Both were pinned as levers, so this role of a length needs naming. The added words "or as the band's pad" are needed because without them the README contradicts the branch's own joint-room conversion. The sentence outlives the join's row.
- **Sentence 4:** `fate::tip_wraps`, and any later winding count. Without "a whole number of periods", "read as an integer" still permits rounding a value.

**Final state per site (agreed).**
- `dihedral::wedge_reads_zero_at_the_arm`: `Margin::levered(sin θ, arm)` at its real arm, decided in the arm refusal's `offered_band()`.
- `dihedral::wedge_reads_zero_at_the_offer`: the same wedge in the pairing refusal's `offered_band()`. The division by `|cos θ|` goes.
- `fate::tip_wraps`: `Period::new(half-turn).decide(..)`, metered at the corners' least reach from below, with the integer compared to ± the corner count.
- `join::off_conic_slack`: the swing of `decide_across` inside `travel`. A separate row reads each site on the conic and deletes the swing.
- `intersect` `pn_apex_tangent_reach` and `pn_apex_point_reach`: `Margin::of(apex_start(..))`, one function for `|δ|/sine`.
- `intersect` `pair_bound` far, near, offset: plain arithmetic, using the same function.
- `validate::resolved_as`: `Margin::of(ρ)` in the band one K-step finer, `(ε/K, ε)`, until the re-posing row deletes it.

**Asked of you.**
- Sign the clause.
- One line of rustdoc rides the same PR: `Margin::of` excludes a quotient that *changes* dimension, not a length over a sine. It restates D4's margin clause (i), so I would rather you see it than not.
- Nothing else needs you now. The join's read on the conic comes to you later on its own PR, under your records condition.
- Shown to you, not asked: the turning sum in `tip_wraps` has no arm of its own. We meter it at the least of the corners' reaches, read from below, which follows the README's rule for a definite claim. The alternative with no lever is the same `Period` marks read for sign alone.

## For the orchestrator

- **The clause.** If the other report will not take "or as the band's pad", the split is that one phrase. My lean is firm: `joint_has_room` on this branch reads `band.padded(band.zero())`. I checked that in the branch diff of `crates/topo/src/pcurves/decision.rs` in round 0.
- **Pinning `Band::new`: a row for later, an agent's to decide.**
  - Part of this recommendation: the two named constructors in geom-core (`Indeterminate::offered_band`, and the one-K-step-finer band).
  - Not part of it: the pin. It is not needed to delete `Lever::unsourced`. It needs a real census with a reason per site; my count of four or five came from a crude script.
  - One of those sites is a question in itself. `step_import::interpret::file_band` is the hidden `Band::linear_at` spelled by hand in production, at a file's ε.
  - It is a gate under `scripts/gates/`, not binding text, so it does not wait for Ev. My lean is to file it as P2 under `work/oneround/`, citing `topo::census::span_clear_as`, `step_import::interpret::file_band` / `band_at` and `topo::chart_region::in_band_premise`.
- **Discoverability, in the stand-ins unit.** The implementer did not find `decide_across`. `Band::padded`'s rustdoc should point to it for a pad in the run's scalar, and `lever-sources.sh`'s "what review holds" line should name a tolerance lifted into a product (agreed in both reports).
- **Not rechecked this round.** I read nothing new in the tree. The `LeverBounds` fold for the count's meter and the conic parameter's availability at `Turn::facing` remain unverified by either report.
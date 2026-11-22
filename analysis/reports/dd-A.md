## For Ev

**Recommendation.** None of the ten is a lever. Once each is given its own reading, none of them needs a new kind of margin, and none needs a door that would do what `Lever::unsourced` does under another name. Sorted by what each reading is:

1. **A question asked at another tolerance (the 2 `dihedral` sites).** Each asks: "would the wedge read zero at the tolerance this refusal offers?" Decide the wedge itself, `Margin::levered(sin θ, arm)` with its real folded arm, in the band of the offered tolerance. That band comes from one new geom-core door, `Indeterminate::offered_band() -> Option<Band>`, built from the same spelled offer the refusal's sentence prints. `None` means no offer, which replaces the separate `offers_tolerance()` / `re_quotes()` checks. *likely*
2. **A count (`fate`'s `tip_wraps`).** The loop's half-turn total is an integer by construction, so read it as one. A new count door rounds the reading to the nearest integer and refuses if it is not within a fixed fraction of one. That refusal is a kernel invariant, not a tolerance. Then compare `half_turns == ±halves` exactly. Today it is compared with ε (about 1e-9) at a unit arm, which is meaningless, because ε is a length and this is a number of half-turns. The door could also be a constant band like `Band::enclosure_sign` (a `Margin::of_count` decided only under a fixed band, Zero within ½, definite at ≥ 1); the effect is the same. *sure* that it is a count; *likely* on the door.
3. **Interim numbers that are not margins (`intersect`'s `pair_bound`: far, near, offset).** All three call `Margin::levered(..).value()` and exit at once. The door is being used as a multiplication. They are steps in a bound derivation that already ends in `Margin::of(far.min(start + offset) + azimuth)`. The fix is plain arithmetic with no door. The divisor D is decided positive on this lane, which the re-posing rule already permits. *sure*
4. **A computed distance (`pn_apex_tangent_reach`, `pn_apex_point_reach`).** `size / steep` and `size / |D|` are how far the real section starts from the apex, measured along the plane. That is a length, and it is the reading itself. The ratified bullet already says this: "a stand-off is the margin … a dimensionless factor belongs to the reading". The divisor is decided definite on each arm. So it enters as `Margin::of` of a named geometric function, just as `pair_bound`'s result already does. This is not "the same number under another door". The earlier rejection was of `levered_inv`, whose contract claims a per-metre divisor, which is false here. `Margin::of`'s doc ("nothing whose metres-ness needs … a quotient") has to say what it means: a quotient that *changes* dimension (a measure over a lever, a span over a rate) has its own door; a length over a pure number decided definite does not. *likely*
5. **A charge the README already forbids (`validate::resolved_as`, and the `gap_err` inside 4's `size`).** `resolved_as` decides `K·ρ ≤ ε` on a hand-charged rounding ρ. Its own doc says it stays only until `work/oneround/site-rounding-charges-owed-a-re-posing.md` re-poses the readings that need it. `intersect`'s gap and axis charges are on the same row. In the final state both are gone, by the README's "this is the one statement about rounding, and no site charges it". Until then, the honest interim spelling is `Margin::of(ρ)` in `Band::new(ε/K, ε)`: the band one K-step finer. The Band docs say a derived scale goes through `Band::new` (the precedent is `chart_region`'s halved-zero band). *sure* on the destination.
6. **A reading made off the wrong point (`join`'s `off_conic_slack`, `2ε·cot ψ`).** Not a lever and not a tolerance either. It exists because `turned_past` is read at the vertex point, which may stand up to ε off the conic, and that offset moves the reading by `δ·cot ψ`. Read each site *on* the conic instead, at the conic's own parameter at that vertex, and both the slack and its two decisions go. The slack's tie check is in fact `2·cot ψ ≤ 1` against K, with the tolerance cancelling. This means `Turn` holds a parameter, not only a point, so it falls under your records condition from PR 7141. *likely*. Fallback: decide `|m|` in a band padded by the slack (`Band::padded`). That needs the pad as an `f64`, which it is not today, so it is the weaker answer.

**Premise check.** The four families are not the right cut. Family 3 mixes three things: interim arithmetic (3 sites), a computed distance (2) and a misplaced reading point (`join`). Family 4 and `intersect`'s `size` are one thing: a rounding charge the ratified rule already removes. The brief also says "the band's escalation threshold" is the dihedral sites' lever. It is only an algebraic step: `sin θ · Kε` against (ε, Kε) is `sin θ` against (1/K, 1). The question underneath is about the offered tolerance.

**Is each reading needed?**
- **dihedral:** yes. Without it, a tangent seam whose wedge is pure rounding offers a tolerance decades too small. It affects only which refusal is quoted.
- **tip_wraps:** yes. It tells a tip the face wraps from a spur.
- **pair_bound's three:** they are not decisions at all.
- **The two reaches:** yes. They decide when the apex outcome stands in for the section.
- **resolved_as:** only until the re-posing row lands.
- **join slack:** only because of where the reading is taken.

**Ratified text.** The lever bullet in `crates/geom-core/README.md` ("A length that is the reading itself … is the margin, and a tolerance is the band's … A dimensionless factor belongs to the reading") was written by agents and approved by you on PR 7141 (commit f2a9024a45, "sounds great!"). It already classifies all ten correctly. What it lacks are two kinds of reading it does not name. I would add one clause, which binds later sites and so needs your sign-off: *"A count is an integer and is read as one. A reading asked at a tolerance other than the run's (the one a refusal offers) is decided in that tolerance's band, never levered by it."* No other ratified text changes. The `Margin::of` doc fix in item 4 is rustdoc, not a README design page.

**Alternatives weighed and rejected.**
- *(a) A "ratio" margin door decided in a band `(1/K, 1)` or `(1, K)`, for any dimensionless reading against K.* It fits dihedral and the join tie check by algebra. But it states the derivation instead of the question, and it would become the general home for "a number against K". That is the escape hatch again, one level over. Rejected.
- *(b) `Margin::scaled(length, factor)` for every stand-off times a factor.* It is honest about dimension. But it invites any length times any number with no check that the factor's divisor was decided definite. Items 3 and 4 do not need it. Rejected; reversible if a later site needs it.

**What is still possible afterwards.** With `Lever::unsourced` gone, the remaining routes for a site's own number are `Band::new(any, any)` and `Margin::of(x * y)`. Neither is gated. I would give the derived bands named geom-core constructors (`offered_band`, the finer-share band) and pin `Band::new`'s production references the way the lever doors are pinned. *likely* worth doing; it is a ratchet, not a proof.

**Reversibility.** Every item above is local and easy to undo. Only item 6 changes a record (`Turn`).

## For the orchestrator

- **The ten**, checked in the tree:
  - `dihedral.rs` `wedge_reads_zero_at_the_arm` and `wedge_reads_zero_at_the_offer`
  - `fate.rs` `sign`, called twice from `tip_wraps` (one reference)
  - `join.rs` `off_conic_slack`
  - `intersect.rs`: the two reaches, plus far, near and offset in `pair_bound`
  - `validate.rs` `resolved_as`
- **Brief error.** "ten references remain" is the unit's ten. The tree holds about 45 `Lever::unsourced` references across `lever-sources.counts`; the rest belong to sibling units. Ev should not read "ten" as the whole remainder.
- **The `.value()` pattern.** `Margin::levered(..).value()` used as arithmetic is a pattern worth a grep across the tree. The two `ssi::certify` and `ssi::march` sites also call `.value()` immediately. It is invisible to both gates.
- **`resolved_as` callers.** They are `chord_join` (`SECTION_PIN_RESOLUTION`) and `vertex_section` (`VERTEX_SECTION_RESOLUTION`). Both are already named in `site-rounding-charges-owed-a-re-posing.md`, as is `intersect` (its line 96). The interim `Band::new(ε/K, ε)` spelling should land with the stand-ins unit; deleting it belongs to that row.
- **`offered_band` spelling.** It should use the spelled offer (`Band::offer_told`, which is private today), not `|m|/K`, so the decision matches the number the sentence prints. That moves last bits at the edge, in the direction of the printed offer. Measure it on both lanes as the unit does. The pairing site's offer comes from the pairing refusal's own band, so build it from that `Indeterminate`, not from `band`.
- **Not checked:**
  - That a conic parameter at each join site is available upstream (the edge's certified endpoint parameter). `Turn` holds only `(Point3, Vec3)`.
  - How `Sym` and `Interval` read a count door. A constant-band form rides `sign_within` on every lane, which is why I offered it as the alternative.
  - I built and measured nothing.
- **Fallbacks.** If item 6's re-pose proves infeasible, the `Band::padded` fallback needs a pad read as an `f64` bound. Recommend it only with that measured.
- **Process.** The README clause in "Ratified text" is new binding text, so it goes through an `[ev]` PR. The `Margin::of` rustdoc change does not.
- **Off-question defect.** `pair_bound` charges its own rounding throughout (`f64::EPSILON` terms, step 5's `u` budget). It is listed as `intersect.rs prod 6 re-pose` in `bare-rounding-sites.tsv`, so it is already tracked. No new issue filed.

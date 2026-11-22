## For Ev

**Recommendation (likely).** None of the ten is a lever, and the vocabulary owes them one new door, on the band. The four families are the wrong cut. Cut by what the *length* at each site is, which the README's lever bullet already names ("a stand-off is the margin, a tolerance is the band's, a dimensionless factor belongs to the reading"):

- **A. A tolerance multiplied into the reading (3 sites: `dihedral` ×2, `validate`).** Owed: a band door, "this band at the tolerance this refusal offers".
- **B. A count (1 site: `fate::tip_wraps`).** Owed nothing: `Period::decide` already reads a whole number of periods as an integer.
- **C. A stand-off scaled by a pure number (6 sites: `join` 1, `intersect` 5).** Owed nothing: plain arithmetic inside a derivation whose result enters doors that exist (`Margin::of`, `decide_across`).

No dimensionless-reading type, no fourth lever source, no new `Margin` door.

**Terms.** A *margin* is a length decided against a *band* `(ε, K·ε)`: within ε reads Zero, past K·ε reads a definite sign, between refuses. A *lever* is the length that turns a sine into a margin. A refusal *offers* the tolerance `m/K` that would decide its margin `m`. A *stand-off* is a distance the site reads (a plane's distance from an apex, a vertex's distance off its curve). A *swing* is how far a stand-off can move another reading. A *door* is a named constructor whose doc states what its operands are.

**A. The band at an offered tolerance.**
- *What the sites decide.* `wedge_reads_zero_at_the_arm` and `_at_the_offer` run after a refusal, to choose which margin the refusal quotes. They ask: at the tolerance this refusal offers, would the wedge `sin θ · arm` read Zero? Only Zero is acted on, and no verdict depends on it. Without it, the refusal offers a tolerance that does not decide the edge (the suite cites an offer of `1.05e-25 m` on a tangent pair).
- *What the reading is.* The ordinary wedge at its real, sourced arm, which both callers hold. What is not the run's is the band. `Band` has no door for "this band at another tolerance", so each site divides the arm out of both sides and is left with `sin θ` against `1/K`, with the band's own threshold handed in as the lever.
- *Final state.* A door beside `Indeterminate::offers_tolerance`, say `offered_band() -> Option<Band>`: the band at the tolerance the refusal's own sentence tells, K kept, `None` where it offers none. The sites become `decide(Margin::levered(sin θ, arm), offered)`. The `sin θ / |cos θ|` quotient in the twin goes, and so do two `T::from_f64(band.escalate())` lifts.
- *Why not `Band::linear_at`.* That hidden instrument takes a caller's own ε. This door's ε can only come from a refusal, so it does not open what D4 ¶1 closes.
- *`validate::resolved_as` is the same question.* `K·ρ ≤ ε` is "the rounding `ρ` reads Zero at the tightest tolerance an in-band refusal here can offer, `ε/K`". It is `Margin::of(ρ)` against that band. Its own doc says it stays only until `site-rounding-charges-owed-a-re-posing` re-poses its two callers, so this spelling is interim.
- *Rejected: a pure-number door* (sine against `1/K`). A threshold in pure numbers is a global angular tolerance, which D4 ¶1 rules out. (sure)

**B. The count.**
- *What it decides.* Whether a corner whose two straight edges leave along one ray is a slit's tip (the face wraps it) or a spur. Locally the two look alike, so the site sums the loop's turning and asks whether `(whole turn − turned)/π` equals minus or plus the number of such corners. It is needed.
- *What the reading is.* An integer plus the rounding of a sum of `atan2`s. Today it is read "Zero within ε at a 1 m arm", a number of half-turns held against a tolerance in metres.
- *Worked example of the harm (likely; argued, not run).* A corner is counted as a half-turn when its cross product reads Zero at its *own* reach. On 1 mm edges that admits a corner up to `ε/1 mm = 1e-6` rad short of a half-turn. The sum is then `3e-7` half-turns off an integer. At the unit arm that is `3e-7`, past the band, and the tip is refused as unread. The corner passed at its own lever and failed at a lever no geometry has.
- *Final state.* `Period::new(half turn).decide(...)`, metered by the corners' reaches, returns the integer, which is compared with the count as integers. The candidates are a whole half-turn apart, so it decides wherever the edges are longer than the band.

**C. Stand-off times a pure number.**
- *`intersect` (5).* Each is `(|δ| + rounding)/sine`: the plane's distance `δ` from the cone's apex, read along a generator. It is the real distance from the apex to where the true section starts, and it decides whether the apex outcome may stand in for that section (Zero only). The length is needed and the number is right. Only the door's claim was false.
  - Three of the five (`pair_bound`'s far, near, offset) are never decided at the lever door. They are read straight back with `.value()` as steps of a derivation that ends in `Margin::of`.
  - The other two are the same formula. Final state: one function for `|δ|/sine`, plain division, called from all five places, its result entering `Margin::of` under that door's clause "a distance computed by upstream geometry code".
- *`join::off_conic_slack` (1).* `2ε·cot ψ` is how far two vertices' ε stand-off from the conic can move the arc reading `turned_past`. `travel` decides the reading clear of it. That is exactly `k_stats::decide_across(datum, swing)`, which takes a `T`-valued swing and has about 38 references. `travel` hand-rolls it and decides the two terms one at a time, which `decide_across`'s doc says is unsound (each just inside the band sums to nearly twice it). Final state: `travel` calls `decide_across`, and the slack is its swing.
- *Is this "the same number under another door"?* For C the number should stay; for A it stays up to bits; for B it changes. The test of a real conversion is whether each operand passes a door whose stated argument is true of it, not whether the number moves. `levered_inv(size, sine)` failed that test (a sine is not a per-length), as `Lever::unsourced(size)` does (a stand-off is not an arm). `Margin::of(size/sine)` and a swing pass it.
- *Defensible alternative:* a `Margin::scaled(factor)` door, so the multiplication sits inside a door as the `Margin` doc prefers. It names the product but checks nothing `Margin::of` does not, and nothing here needs it. I lean against. (likely)

**Premise check.**
- "Ten remain" is this unit's ten. `lever-sources.counts` on the branch still pins 45; the other 35 belong to sibling units.
- "A `T`-valued stand-off cannot pad a band" is true of `Band::padded` and `Margin::less`, but `decide_across` is the door for it.
- The implementer's four stated facts hold. (sure)
- Four of the ten are also rows of the from-f64 gate ("the band's width, a tolerance"). README DL5.2 says a band enters "as the tolerance type", never as a scalar. That gate, not the lever gate, sees kind A.

**Ratified text.** The two sentences in the lever bullet this rests on were agent-written in the `[ev]` PR 7141 (commit `f2a9024a45`) and approved with "sounds great!", not written by you. I would keep them and add what they leave unsaid, in the same bullet:

> A tolerance is not multiplied into a reading: a margin asked at the tolerance a refusal offers is read against the band at that tolerance. A length scaled by a pure number is still that length. A reading known only within a stand-off takes it as its swing. A count has no length, and is decided as a whole number of periods.

`Margin::of`'s doc says it admits nothing "whose metres-ness needs … a quotient". That means a quotient that *produces* metres (area over length), and should say so; as written it reads as forbidding a length over a sine.

**What each state leaves possible.** Nothing in the types stops `Margin::of(x * any_number)`. That is true today and under every option, because D4 declines dimension algebra; review holds it. The recommendation removes the one door that invited it. Everything here is additive and reversible.

**Verdicts that move.**
- A: last bits, and the offer's rounding to the told grid.
- B: more tips decided.
- C `intersect`: none if the product is kept, last bit if spelled as a division.
- C `join`: a tie now needs reading plus swing within ε together, which is stricter.

**Confidence.** The cut and A: likely. B: likely. C `intersect`: sure that it is arithmetic, likely on the single home. C `join` through `decide_across`: likely. Nothing was built or measured.

## For the orchestrator

**Checked in the tree** (worktree `/root/.local/share/cad-work/designer-dd-b`, read-only, nothing written):
- `crates/geom-core/src/predicate.rs`: `Margin` is a length by signature; `Margin::sign_of` is the only no-length door, and only under `Band::enclosure_sign`. `Band::padded` and `Margin::less` take `f64`. `Band::linear_at` is `doc(hidden)`. `Band::tolerance_deciding`, `Band::offer_deciding` and `Indeterminate::offers_tolerance` already compute the offer, and the last one's doc names this use ("for a door that re-quotes an escalation at a reading it meters").
- `crates/topo/src/chart_region.rs` `in_band_premise` is the one production derived band, as the brief says.
- `crates/geom-core/src/period.rs` `Period::decide`: "the whole number of periods … as the integer it is … never by rounding the gap".
- `crates/geom-core/src/k_stats.rs` `decide_across(names, datum: T, swing: T, band)`.
- `scripts/gates/from-f64-sites.tsv` rows for `dihedral.rs` (2), `boolean/join.rs` (1) and `validate.rs` (1). 27 rows in that file mention a band threshold; I did not classify the other 23. Those that multiply a threshold rather than add it are the likely later sites of kind A.
- `scripts/gates/bare-rounding-sites.tsv`: `intersect.rs` prod 6 `re-pose` (the `gap_err` terms) and `vertex_section.rs` prod 6 `re-pose`. So the rounding half of `size`, and `resolved_as` whole, are already owed away by `work/oneround/site-rounding-charges-owed-a-re-posing.md`.
- Provenance by `git log --all -S`: "neither is an arm" and "belongs to the reading" → `f2a9024a45` (2026-11-19, PR 7141). "Rates are not levers" and "No `Margin` door takes hand-written slack" → `31576a6c6e` (2026-11-02, "[ev] margin scale per Ev"). "A tolerance is the band's" returned nothing as a phrase (wrapped line), but it is in the same sentence as the first.

**Errors or thin spots in the brief.**
- "Ten references remain" reads as tree-wide; it is this unit's residue.
- `git diff origin/main...HEAD` here is 29 files, not large.
- Family 3's "stand-off" is two different things: a read residual in `intersect`, the tolerance ε standing in for an unread residual in `join`.
- Family 4's own doc already calls it an interim amplification share.

**Not checked; the unit must.**
- Whether the offered band should sit at the told offer (`offer_told` is a private `fn` on `Band` taking `f64`) or at the raw `|m|/K`, and what the refusal's margin is on the Interval lane. `MarginDiag` is deliberately unreadable outside geom-core, so the door has to live in `predicate.rs`.
- That the pairing refusal's margin is `|cos θ|·arm` (read from `normals_alignment`'s `Margin::levered(n1·n2, lever)` only).
- How `Period::decide`'s `WalkStop` maps onto `tip_wraps`'s two refusals (`unread` and an escalation), and which fold of the corners' `LeverBounds` the meter takes. A mark's clearance is a definite claim, so it should read from below.
- `vertex_section` calls `resolved_as` before it decides, so it has no refusal in hand. It needs the `(ε/K, ε)` form, or a reorder. `chord_join` has the refusal (`cause`) and could read `ρ` in that refusal's own offered band, which admits slightly more as "the input's".
- The B worked example is arithmetic, not a reproduced failure.

**Defects off the question** (I could not write rows; they belong under `work/oneround/` unless noted):
1. `topo::boolean::join::travel` decides `turned_past` and its slack one at a time for a tie. The fix is the `decide_across` conversion above.
2. `off_conic_slack` charges both sites' stand-off at `from`'s `ψ`, and charges the worst case ε, not the sites' actual distance off the conic. `Turn` holds an axis and a site but no conic, so the residual cannot be read today. Reading it would remove the tie escalations the doc reports "from about `cot ψ = 0.5` on". That changes what a record holds (Ev's condition from PR 7141), so it is its own row, not this unit.
3. `fate::tip_wraps` reads its count at a 1 m arm while its corners read at their reaches (example above).
4. `intersect.rs` holds three copies of `|δ|/sine` (`pair_bound`'s `far` and `near`, and the two reach arms).
5. `wedge_reads_zero_at_the_offer` divides by `|cos θ|` with no pose decided first. It is benign today (an infinite quotient reads not-Zero) and is removed by A.
6. `lever-sources.sh`'s "what review holds" line names `Margin::of(x * arm)` only. Once `unsourced` is gone it should also name a tolerance lifted into a product, with a pointer to the from-f64 rows.

**If Ev takes the recommendation,** the README sentence is ratified text and rides an `[ev]` PR. The `Margin::of` doc line is rustdoc, but it restates D4's margin clause (i), so I would put it in the same PR.
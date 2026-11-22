**The decision: add four sentences to the lever bullet** of `crates/geom-core/README.md` ("Float rounding"), after "A dimensionless factor belongs to the reading, not the arm." (the diff):

> A margin asked at the tolerance a refusal offers is read against the band at that tolerance, never levered by it. A length scaled by a pure number is still that length. A reading known only within a stand-off takes that stand-off as its swing or as the band's pad. A count has no length, and is decided as a whole number of periods.

Two designers weighed it and both sign this text. There is no split.

**Why it is asked.** ONEROUND is deleting `Lever::unsourced`, the door that lets a site hand in any number as a lever. After the stand-ins unit (PR 7268) ten references remain that are not levers at all: each uses the lever door as a multiplication because no sentence says what it is. The bullet you approved on PR 7141 ("a stand-off is the margin, a tolerance is the band's, a dimensionless factor belongs to the reading") classifies them rightly, but it was in force when five of them were pinned as levers anyway. The four sentences name the four roles it leaves unsaid.

**What each sentence does, by the site that needs it.**
1. **A tolerance.** Two `dihedral` sites ask: would this wedge read zero at the tolerance this refusal offers? Today they multiply sin θ by the band's own threshold, as if it were an arm. After: the real wedge at its real arm, decided in the offered tolerance's band. That needs one new door on the refusal (`offered_band`).
2. **A scaled length.** Five `intersect` sites read a plane's distance from a cone's apex over a sine: the distance to where the true section starts. That is a length, and it enters `Margin::of` as one.
3. **A stand-off.** The join's slack (2ε·cot ψ) is how far a vertex's ε stand-off from its conic can move an arc reading. It becomes the swing of an existing door (`decide_across`). This also fixes a defect: today a tie decides the reading and the slack one at a time, and two terms each just inside the band sum to nearly twice it.
4. **A count.** `tip_wraps` sums a loop's turning and compares it with a whole number of half-turns. Today that is "zero within ε at a 1 m arm": a number of half-turns held against a tolerance in metres. After: the existing period walk (`Period::decide`), which decides the integer by signs at the half-period marks and never rounds a value.

**The alternative both rejected:** a door for "a pure number against K" (a dimensionless margin). It fits two of the sites by algebra, but it would become the general home for a site's own number, which is the door being deleted, one level over.

**Not decided here.**
- Reading the join's sites on the conic itself, so the slack goes entirely, changes what a record holds. It comes to you on its own PR, under your condition from PR 7141.
- `validate::resolved_as` (a rounding held against ε/K) is a hand-charged rounding the README already forbids. It gets an interim spelling until its re-posing row removes it.
- Shown, not asked: the count's marks are metered at the least of the corners' reaches, read from below. The alternative with no lever is the same marks read for sign alone.
- A gate pin on bands built by hand from the band's own thresholds is an agent's to land and is filed as a row.

**One line of rustdoc rides along** so you see it: `Margin::of` excludes a quotient that changes dimension (a measure over a lever), not a length over a sine.

**What this rests on.** Nothing was built or measured; both designers read the tree and checked each other's claims in it. When the sites are built, the verdicts expected to move are last bits at the `dihedral` sites, more tips decided, and a stricter tie at the join; they are reported on the unit's PR.

The rounds are in `work/oneround/log.md` (2026-11-22); the fork's row is `docs/DESIGN-FORK-LOG.md` 303. Below, each designer's final section as written. "The other report" in a section is the other designer.

---

## A

I sign the other report's four sentences, word for word. They go in the lever bullet of `crates/geom-core/README.md`, "Float rounding", after "…belongs to the reading, not the arm.":

> A margin asked at the tolerance a refusal offers is read against the band at that tolerance, never levered by it. A length scaled by a pure number is still that length. A reading known only within a stand-off takes that stand-off as its swing or as the band's pad. A count has no length, and is decided as a whole number of periods.

I checked the three claims that moved me in my worktree. All three hold.
- **Sentence 2, the scaled length.** The `Lever` rustdoc uses "reading" for the sine: "a dimensionless reading (a sine, a cosine…)". So "a dimensionless factor belongs to the reading" says the factor stays with the sine. It does not say that a stand-off over a sine is still a stand-off. That sentence was already in the README (commit f2a9024a45, PR 7141) when PR 7227's review re-pinned the five `intersect` quotients on `Lever::unsourced`. My round-2 argument that sentence 2 repeats it is answered.
- **Sentence 3, the pad.** `pcurves::decision::joint_has_room` on this branch reads `band.padded(band.zero())`, which I confirmed. A stand-off used as a pad outlives the `join` row that deletes the swing, so the sentence describes a lasting role, not an interim one.
- **Sentence 4, the count.** "Read as an integer" still allows rounding a value, which `period.rs` forbids ("No whole-period integer is read off a value"). "Decided as a whole number of periods" names the `Period::decide` walk, which is what we agreed for `tip_wraps`.

The per-site final states are unchanged from round 2 and agreed by both reports.

## B

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

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw

## For Ev

I sign the other report's four sentences, word for word. They go in the lever bullet of `crates/geom-core/README.md`, "Float rounding", after "…belongs to the reading, not the arm.":

> A margin asked at the tolerance a refusal offers is read against the band at that tolerance, never levered by it. A length scaled by a pure number is still that length. A reading known only within a stand-off takes that stand-off as its swing or as the band's pad. A count has no length, and is decided as a whole number of periods.

I checked the three claims that moved me in my worktree. All three hold.
- **Sentence 2, the scaled length.** The `Lever` rustdoc uses "reading" for the sine: "a dimensionless reading (a sine, a cosine…)". So "a dimensionless factor belongs to the reading" says the factor stays with the sine. It does not say that a stand-off over a sine is still a stand-off. That sentence was already in the README (commit f2a9024a45, PR 7141) when PR 7227's review re-pinned the five `intersect` quotients on `Lever::unsourced`. My round-2 argument that sentence 2 repeats it is answered.
- **Sentence 3, the pad.** `pcurves::decision::joint_has_room` on this branch reads `band.padded(band.zero())`, which I confirmed. A stand-off used as a pad outlives the `join` row that deletes the swing, so the sentence describes a lasting role, not an interim one.
- **Sentence 4, the count.** "Read as an integer" still allows rounding a value, which `period.rs` forbids ("No whole-period integer is read off a value"). "Decided as a whole number of periods" names the `Period::decide` walk, which is what we agreed for `tip_wraps`.

The per-site final states are unchanged from round 2 and agreed by both reports.

## For the orchestrator

- **No split remains on the clause.** I conceded on checked facts, not to close the round.
- **Noted, no reply needed.** You are filing the `Band::new` pin as its own row and keeping the two named constructors in the recommendation. That matches my round-2 position.

# Dual Opus review — reviewer concordance log

**This is process data (an experiment log), not a design
reference** — nothing here binds kernel design; it moves out of
`docs/` when the experiment concludes.

Standing experiment (Ev, in-chat, 2026-09-23), opened the day the
model A/B protocol was suspended (`docs/MODEL-AB-LOG.md`, the
suspension entry). **The protocol in force lives in
`docs/DUAL-REVIEW-PROTOCOL.md`.** This log holds the rows; each names
the protocol it was recorded under by commit hash, so `git show
<hash>:docs/DUAL-REVIEW-PROTOCOL.md` reads that version.

## Rows

Columns: **protocol** (commit hash of the protocol in force at merge — the last
commit on main that touched `docs/DUAL-REVIEW-PROTOCOL.md`); **unit** (program, PR); **class** (difficulty S/M/L and task
class, logged at spec time) and the **triage reason**; **head** (the
frozen commit); **R1** and **R2** (verdict, MAJ/MIN/NOTE, one line of
prose per MAJOR, tokens, wall-clock); **correspondence** (bilateral
headline; unilateral findings R1→ and R2→; severity divergence);
**tally** (candidates, with the 6(a)–(e) disposition of each);
**fair** (yes, or the relaxation / interruption that excludes it);
**fix pass** (size, and who executed it).

| # | date | protocol | unit | class · triage reason | head | R1 | R2 | correspondence | tally | fair | fix pass |
|---|------|----------|------|-----------------------|------|----|----|----------------|-------|------|----------|
| DR-1 | 2026-09-24 | `c3129311b` | TOPO, PR #3148 — the fan `mev`'s re-basing gate and null edges (`a-null-edge-can-be-re-based-onto-a-distinct-point` + `the-re-basing-gate-refuses-m7-8-where-nothing-moves`) | M / STRUCTURAL · the unit decides what a kernel gate may ask about "nothing moved" with no point-identity door at `T: Real`; one shape was a structural-identity door on `geom-core`'s ratified surface — an architectural decision hard to change later (recorded at spec time) | `36c7d0f36` | APPROVE-WITH-FIXES, 0/6/6. No MAJOR. 268,769 tokens, 31 min (harness) | APPROVE-WITH-FIXES, 0/6/4. No MAJOR. 229,973 tokens, 52 min (harness) | Coded attribution-stripped, byte 233. BILATERAL, all executed: the both-halves refusal's stated reason false and the one-half mutant surviving; the null-vs-carrier run order unpinned; `Display`/variant doc claiming what the gate cannot know; the M7-8 no-move spelling oversold (its row asserts tier 1 only); the filed `kev` over-refusal hypothetical (zero measured instances); the stale-curve-key drive-by unpinned (MINOR vs NOTE, severity divergent); plus `program.md` rot, `19032e09f` authorship read as ratification, `needs_ev` unscheduled, a `NullScaffoldCurve` sibling, the rationale restated in five or six homes, and a structural both-halves test preferred (style/notes). UNILATERAL R1→: the production bit compare `topo::query::same_point_bits` (in `rim_of`, unratified, invisible to the bit-identity gate) — MINOR, claim class, orchestrator-verified; it contradicts R2's "no door exists, shape 1 forced"; zip's 58 ulp-distinct kills (inside MINOR); the `mev_null` control's F9 attribute (unsure). UNILATERAL R2→: the two-call spelling is not atomic (inside MINOR, executed); "bit for bit" spelled two ways (style); the "wherever" row's second point pins nothing current (style; R1 judged it fine) | No candidate: neither review raised a MAJOR (6(a)). Running: 0 | yes — identical briefs modulo lane paths (sha256 stored), no relaxation, neither interrupted; both disclosed seeing the head commit's co-author trailer (a model name; not a finding of the other review) | ten items, all taken, none refuted, by one fresh Opus lane (299,225 tokens, 77 min harness): the refusal refined to one-half-in-the-run, run order and drive-by pinned, rationale given one home and corrected for `query.rs` (filed on TQUERY), M7-8 asserted through `Body::mev` with tier-3 reading and non-atomicity rowed, `kev` evidence corrected and its probe a test; head `1baa7735e` run 35964734053 green, full matrix. The implementer (Opus): 262,570 tokens, 151 min (harness) |

**Tally: 0 of 8. Fair pairs toward twelve: 1.**

# Opus 5 vs Fable 5 implementation A/B — experiment log

Standing experiment (Evan, in-chat, 2026-07-25, Opus 5 release
day). Protocol:

- Every IMPLEMENTATION dispatch gets a fair coin flip from
  /dev/urandom: heads = Opus 5 (`model: "opus"`), tails = Fable 5
  (session default). Design, specs, adversarial reviews, and
  fix-pass rulings stay Fable regardless. The fix pass runs as the
  implementer's agent, so it inherits the arm.
- Reviewers are BLINDED: implementer model is never named in
  review prompts or in anything the reviewer reads (reports must
  not state it either — orchestrator checks before handing the
  implementation report to a reviewer). The arm is recorded ONLY
  here until the review + fix pass conclude.
- Every review includes a CODE QUALITY REPORT with a fixed rubric,
  identical across reviews (in addition to the usual findings):
  - counts: MAJOR / MINOR / NOTE findings; spec deviations
    (reported vs silent — silent ones counted separately and
    weighted worst)
  - ratings 1–5 with one line of evidence each: idiom/structure,
    test quality (do the tests pin the real contract?), doc/comment
    honesty
- Per-row objective companions: pre-dispatch difficulty guess
  (S/M/L, logged BEFORE the flip), fix-pass size, battery outcome,
  subagent tokens, wall-clock.
- Small-n caveat, stated up front: this yields a suggestive
  comparison, not significance. Read stratified by difficulty.

| # | date | task | difficulty (pre-flip) | arm | review findings (MAJ/MIN/NOTE) | silent devs | idiom | tests | docs | fix-pass size | battery | tokens | wall-clock |
|---|------|------|----------------------|-----|-------------------------------|-------------|-------|-------|------|---------------|---------|--------|------------|
| 1 | 2026-07-25 | #93 join-stage seam-region anchors | M | fable (draw 197) | 2/1/2 (both MAJ = claim-level; one credited a main bug the fix already fixed) | 0 | 4 | 4 | 4 | moderate (doc corrections + 2 adopted pins + 2 minors) | 145 suites: 1263/1263/1408, all pin families green | ~1.15M (incl. 2 crash resumes) | ~19h wall incl. crash gap (~5h active) |
| 2 | 2026-07-25 | #99 tour ε-panic | S | fable (draw 220) | 0/1/3 | 0 | 5 | 4 | 5 | 1 line (orchestrator-applied) | tour 3/3 ε rows; zero kernel diff | 58k | 6.4 min |
| 3 | 2026-07-25 | M4 PR 6 persistence | L | fable (draw 221) | — | — | — | — | — | — | — | — | — |
| 4 | 2026-07-25 | #101 declared tangency | M | fable (draw 218) | 1/1/3 | 1 (falsified doc claim) | 5 | 4 | 4 | moderate (fillet Result + fit predicate + 6 pins) | 147 suites: 1283/1283/1429 + eps 3/3 | ~880k | ~9h wall (incl. crash gap) |
| 5 | 2026-07-26 | interval transcendentals crate | L | fable (block-1 remainder) | — | — | — | — | — | — | — | — | — |

PROTOCOL CHANGE (2026-07-25, Evan approved: "shuffled pairs makes
sense") after four consecutive fable draws (197, 220, 221, 218;
p = 1/16): remaining dispatches use BLOCKED randomization —
opus/fable pairs, order shuffled per block via /dev/urandom.
Transparency note: block 1's order is FORCED (opus, fable) as an
explicit balance correction for the 4-0 fable start — the actual
draw came out (fable, opus) and was overridden, recorded here
rather than silently; random order resumes from block 2. Blinding
unchanged (reviewers never learn arms; difficulty still logged
pre-assignment).

(Reference rows, pre-experiment, both Fable, unblinded — context
only, not comparable: M4 PR 4 impl (L): 2 MAJ / 2 MIN / 6 NOTE, 0
silent devs, fix pass P1–P4, all-green battery. Demo refresh (M):
0 MAJ / 5 MIN / 4 NOTE, 0 silent devs, light fix pass, all-green.
M4 PR 5 impl (L, first rubric-scored reference): 1 MAJ / 3 MIN /
3 NOTE, 1 SILENT deviation (skip-lane tier-3 posture unstated),
idiom 4 / tests 4 / docs 4, substantial fix pass (F1 re-describe
machinery + plumbing + door tests), battery 1256/0 ×2 + interval
1400/0, ~1.44M impl+fix tokens.)

# Severity pilot: is Fable harsher, or more thorough?

Question (Ev, 2026-11-22): how much of Fable 5.2's MAJOR excess as a reviewer is
Fable grading harder, or raising things Opus would not think worth fixing,
rather than finding more?

## Material

The only concurrent Opus/Fable pairs whose full review reports and blind
correspondence coding survive: PR 6972 (`u6972`, from
`analysis/model-compare/boxfinish-2026-11-14`), and the OneRound pairs
box-region (PR 7225) and anchored-conic (PR 7203), from
`analysis/model-compare/oneround-2026-11-19` and the review sessions' committed
reports. Every other pair's reports lived in sessions this account cannot read.
92 underlying defects (6972's class row 1 dropped for its instances 1a/1b):
29 Fable only, 15 Opus only, 48 both. All three units are H-lettered.

## Method

1. `key.py` reads each blind coding's table: who raised each defect (A/B → model
   from the analysis-branch ledgers) and at what label.
2. One agent per unit (`NEUTRALIZE.md`) rewrote every defect into one template
   (claim, where, evidence, consequence), stripping severity labels, confidence
   words and attribution, and merging both reports' facts where both raised it.
3. `build_packets.py` gave each defect an opaque id and each rater its own
   shuffle of units and items.
4. Four blind raters (`RATE.md`) graded MAJOR/MINOR/NOTE/STYLE, said act yes/no,
   and ranked each unit's findings. Planned: two Opus, two Fable. The Fable
   raters hit the account's Fable usage limit (resets 2026-11-27 17:00 UTC) and
   were replaced by Sonnet: a model that is neither reviewer.
5. `sev_analyze.py` unblinds against the key → `sev_results.json`.

## Result

| | Fable only | both | Opus only |
|---|---|---|---|
| n | 29 | 48 | 15 |
| blind grade, mean (MAJOR 3 … STYLE 0) | 1.57 | 1.56 | 1.38 |
| mean rank percentile (0 most severe) | 0.49 | 0.49 | 0.56 |
| in their unit's bottom third | 34% | 27% | 33% |
| raters would ask the author to act | 54% | 58% | 50% |

- **No long tail of Fable-only findings.** They grade like the findings both
  raised, and no lower than Opus-only ones (within-unit permutation, Fable-only
  minus Opus-only: rank p = 0.40, grade p = 0.48). Each unit separately shows
  the same.
- **Neither reviewer is harsh against the blind grade; Opus is the more
  lenient.** Own label minus blind consensus: Fable −0.25, Opus −0.45 grade
  points (the raters' rubric runs higher than both reviewers' habits, so read
  the gap, not the level). On the 39 shared findings both labelled: Fable higher
  in 12, Opus higher in 5, same in 22; Fable nearer the blind grade (mean
  absolute error 0.42 vs 0.56).
- **The raters agree** (Spearman 0.84–0.99 on ranks; all four within one grade
  on 95% of findings), and the Opus raters did not favour Opus-only findings.

Limits: three units, all H; one neutralizer per unit (a subagent inheriting this session's configured model, Opus 5.5);
no Fable rater; the raters assume each finding's facts are true.

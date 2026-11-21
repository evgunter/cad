# conic-centre-readers: what went to Ev

- `[ev]` PR 7258, opened 2026-11-21 05:25Z from `oneround/ev-conic-stored-point` (`2a4bf8ea5`): fork-log row 295, recommendation half, labels only. The body carries the three round-2 `For Ev` sections verbatim as A, B, C.
- No round was run after round 2. Two measurements arrived between round 2 and the PR, both of which all three designers had named with the outcome that would move them:
  - small cone sections at `Interval`, vertex-stored against centre-stored (`analysis/oneround/conic-vertex-probes` `10aa769d4`): a material difference toward the parabola, so "the mint's own point" over "always the centre";
  - the size table (`253b8a6c9`): no wrong verdict at any size; graded refusals of the centre form from 500 m up, recovered by a chord reading; so the limit stays at the box.
- Token totals as the harness reported them are in `tokens-as-reported.txt` (first report / after round 1 / after round 2).

## After Ev's three comments (2026-11-21, 05:40Z to 05:59Z)

- Rounds 3 and 4 were run (messages under `round3/` and `round4/`; reports under `reports/`). The body of PR 7258 was rewritten at about 06:12Z (an earlier draft of this note guessed 06:25Z) with the three round-4 `For Ev` sections verbatim as A, B, C, and the branch `oneround/ev-conic-stored-point` moved to `32b2ae4ce` (README sentence, unit row, fork-log row 295's cells).
- After round 4 the three agree on the placement family, the construction fixing it, the kernel-bug assertion on constructions whose centre runs off, and the reading split by scalar. They split on a length rule for centre-held conics (A and B: none; C: at the rounding reach R), with positions having crossed between rounds 3 and 4. No round 5: all three named one probe with its outcomes, and it went to the conic lane.
- A further agent, not a designer: a subagent of the orchestrator session (default model) set the PR body from a file and verified it byte for byte; it weighed nothing.

## The probe (2026-11-21, 06:22Z)

`analysis/oneround/conic-vertex-probes` `6929ed83d`, `far-centre-through-real-doors/`: 0 wrong definite verdicts in 18,432 decisions; every other cell a typed refusal. By each designer's round-4 statement of what the outcomes mean (A: "If it reads wrong, a door refusal is needed after all"; B: "Only right answers or typed refusals: (i)"; C: "Only typed refusals at every size: certification alone would do, and the assertion is belt and braces") this is no numeric limit, so the split closed with no round 5 and no designer was re-dispatched. Told to Ev in a comment on PR 7258 at 06:23Z; the body's open-point section was replaced; `oneround/ev-conic-stored-point` is at `206e4e836` (row 295's cells).

## The probe corrected; rounds 5 and 6 (2026-11-21, 07:03Z to 07:16Z)

- 07:03Z: the lane reported that probe 1's chord cells had been judged against the circle through the rounded seam vertex. Re-tallied against the stored centre's circle (`analysis/oneround/conic-vertex-probes` `145cf0987`): twelve wrong roots in the chord reading, none in the centre sum. The "closed with no round 5" note above is superseded. Ev was told to hold at 07:04Z.
- Round 5 (messages under `round5/`): all three withdrew the uncharged chord reading; C withdrew the `R` rule and took main's centre sum; A and B proposed chords from an error-free seam vertex with the centre sum as fallback.
- Round 6 (`round6/`; each shown the other two, and the orchestrator's proposed text): all three signed; A and B conceded C's frame objection; three wording amendments to the sentence, all taken (C: "stored numbers"; B: "and directions"; A's "and of its size" is covered by B's).
- The PR body was replaced with the round-6 `For Ev` sections as A, B, C and the hold lifted in a comment at 07:16Z. Branch `oneround/ev-conic-stored-point` at `69ee0725e`; the fork-log row is renumbered 296 (295 was taken on main by another program's fork).
- Token totals after round 6: A 279788, B 283417, C 406317 (first reports 156066, 156613, 207033).
- Probe 3 (the exact-vertex reading, with a tilted frame judged against both candidate truths) stays with the lane as a measurement for a possible later change; it is not part of what Ev is asked.

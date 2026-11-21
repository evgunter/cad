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

`analysis/oneround/conic-vertex-probes` `6929ed83d`, `far-centre-through-real-doors/`: 0 wrong definite verdicts in 18,432 decisions; every other cell a typed refusal. By each designer's round-4 statement of what the outcomes mean (A: "If it reads wrong, a door refusal is needed after all"; B: "Only right answers or typed refusals: (i)"; C: "Only typed refusals at every size: certification alone would do, and the assertion is belt and braces") this is no numeric limit, so the split closed with no round 5 and no designer was re-dispatched. Told to Ev in a comment on PR 7258 at 06:27Z; the body's open-point section was replaced; `oneround/ev-conic-stored-point` is at `206e4e836` (row 295's cells).

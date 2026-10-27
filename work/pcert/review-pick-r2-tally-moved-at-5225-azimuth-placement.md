---
id: review-pick-r2-tally-moved-at-5225-azimuth-placement
kind: issue
title: review_pick_r2's pinned tally moved at PR 5225's merge (azimuth placement); the nightly is red at every ε
status: open
opened: 2026-10-19
priority: P2
cost: E
refs: [review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose]
---

Found by CONTACTHOLD's nightly-red lane.

**The red.** `viewer::all review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus` (slow set) fails at every ε row of the 2026-10-19 nightly (run 39205007024, `07a57312bf`):

    the tally over the corpus moved from its pin; ...
      left: (441198, 141805, 13524, 6936)
     right: (441198, 141763, 12306, 6534)

The pin was last written by INTENT stage 3 C (PR 5267, `683f44c3c0`), which matched main's tally then.

**Bisected** over first-parent merges from `683f44c3c0` to `07a57312bf`, with `cargo test -p viewer --test all -- review_pick_r2` at the default ε. It passes at `4fe9b67d4e` (the first parent of the merge) and fails at **`f681c58a17`**, the merge of PR 5225 (`pcert/azimuth-placement`, B1: a row is its image placed in its chart's principal window).

**Direction.** The ray count is unchanged, so no vertex moved in or out of the corpus. Against the pin:
- 42 more rays are answered at the aimed vertex;
- 1218 more candidates refuse at the determinant;
- 402 more rays carry a refusal.

The row's other asserts still hold: no genuine crossing is refused, and there are no wide winners.

**Owed.** Decide whether the new tally is what B1's placement should produce. If it is, re-pin with that reading in the pin's doc comment. If not, fix what moved it. The earlier movements and the open question about PR 4861's 60 lost aimed-vertex answers are `review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose` (CLEAVE).

Reproduce: `cargo nextest run -p viewer --profile ci --ignore-default-filter -E 'test(=review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus)'`.

## 2026-10-24 (INTENT nightly triage)

Still red at every ε, in the full suite and in `viewer app feature`
(nightly run 40066431545, main `3dfb96b91c`):

    left: (441036, 141423, 24792, 7386)
   right: (440460, 141163, 13482, 6882)

On 2026-10-23 (run 39906156937) the tally was
`(441036, 141438, 24600, 7266)`. Overnight, 15 fewer rays are answered
at the aimed vertex, 192 more candidates refuse at the determinant, and
120 more rays carry a refusal. The ray count held. The 10-23 triage
left the re-pin to INTENT D′2 (PR 5613), but D′2 merged without touching
this row, and D′3 (PR 5636) does not touch it either. The refusal
columns have nearly doubled since 10-22 (13482 → 24792), so this needs
reading before any re-pin. The row's other asserts still hold: no
genuine crossing is refused, and there are no wide winners. The
overnight move is not bisected. The window's code merges are listed in
the 2026-10-24 triage PR (`intent/nightly-triage-1024`).

## 2026-10-26 (SPLICE nightly triage): the 10-24 → 10-25 move is #5659

Still red in `viewer app feature` on nightly run 40274924857 (main
`38f1a24ed3`), with the same tally as 2026-10-25 (run 40165703340,
`9f5b44561d`):

    left: (441036, 141417, 25542, 7998)
   right: (440460, 141163, 13482, 6882)

That tally appeared between 10-24 (`3dfb96b91c`: `(441036, 141423, 24792, 7386)`)
and 10-25. I bisected over the window's 35 merges by running this row alone
(the reproduce line above) at each candidate:

| merge | tally |
|---|---|
| `40ea8d2114` (#5655) | `(441036, 141423, 24792, 7386)` |
| `8a6d97e661` (#5659, `chart/chart-inversion-one-reading`) | `(441036, 141417, 25542, 7998)` |

So the move is **#5659**, "a point's chart coordinates are one reading
(`geom::ChartInverse`)". At the aimed vertex, 6 fewer rays are answered,
750 more candidates refuse at the determinant, and 612 more rays carry
a refusal. The other asserts in the row still hold. One merge in the
window, `6045ce5ff4` (#5657), does not compile (W2's `walk_refusal`
missed `PcurveMintError::OuterSpanPoisoned`); #5662 fixed that at
`09011e4179`, which reads the old tally. Whether #5659's single chart
reading should refuse more here is the question this row already owes,
answered from that PR. Do not re-pin before it is answered.

## 2026-10-26 — PR 5742 (the value chart reads a written orientation)

Main `6ff6b23` reads `(441036, 141417, 25542, 7998)`, and PR 5742 reads `(441036, 141388, 25542, 7998)`. The PR's 29 fewer aimed-vertex answers all come from one landing, `kitchen_sink` after its first edit, measured landing by landing against the merge base:
- the edit sets the moved copy's `Slide(Y)`;
- under the least-turn chart that slide runs along another carried axis;
- so the edited copy stands elsewhere.

Every other landing reads the base's tally exactly.


## 2026-10-27 (nightly triage): red again after PR 5717's re-pin, from PR 5742

PR 5717 (`pred/mesh-sizing-rule`, `463956a6`) re-derived the pin as
`(392112, 131712, 26058, 8292)`. Nightly run 40455870959 (main
`fb476030a9`) reads `(392112, 131691, 26058, 8292)` at every ε, in the
full suite and in `viewer app feature`. Main `57e3f95a` reads the same.
Bisected over first-parent merges from `463956a6` with the reproduce
line above: last good `767d26a9`, first bad **`19eff38c`, the merge of
PR 5742** (`intent/value-chart-orientation`). That is the move the
2026-10-26 entry measured on PR 5742's branch: aimed-vertex answers
fall (21 now, 29 then) and the refusal columns hold. PR 5742 merged
without re-pinning. The question this row already asks still stands
before any re-pin.

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

---
id: tess-meter-sampled-retune-figure-unreproducible
kind: issue
title: tess-meter's 21-sample retune figure names a draw nothing records, and its reference moved with SPLIT_SCAN_SAMPLES
status: open
opened: 2026-09-08
---


Found by METER unit 6 while re-deriving every sample-count-dependent
figure for `D206`.

`tools/tess-meter/src/lib.rs`, `best_split_scan`'s docs: *"Measured on
the shipped `ceil`'d count over 200,000 random bounds, a 21-sample call
site alone moves the reported cell count by +14.93% on average and
+100% at worst"*, with a paragraph beneath it arguing — correctly —
that a sampling statistic over random bounds is not a property this
crate exposes, that nothing re-takes it, and that the guard it argues
for is live where the number is history.

**Two things that paragraph does not cover, and unit 6 hit both.**

1. **The figure is relative to the shipped pair, so raising
   `SPLIT_SCAN_SAMPLES` moves its reference.** "The shipped `ceil`'d
   count" is the denominator; the constant went 321 → 379 in that unit,
   so the sentence is now a comparison against a scan that no longer
   ships. Nothing said so, and no test could.

2. **The draw is not recorded, so the figure cannot be re-taken even
   in principle.** Unit 6 re-took it over an independently drawn
   200,000 bounds (`muu`, `mvv` log-uniform on `[1e-6, 1e2]`, `muv`
   zero half the time and log-uniform otherwise, unit box,
   `δ_s = 1e-3`) and got **+8.46% average at 321 samples** and
   **+8.59% at 379**, against the stated +14.93%; the worst reproduced
   exactly at +100% both times. So the pair's move disturbs the figure
   by about a tenth of a percentage point — the direction the paragraph
   rests on is untouched, which is why unit 6 left the line standing —
   but the average is a property of a distribution the tree does not
   name, and a reader who re-takes it will not reproduce it and cannot
   tell whether that means the code moved.

**What is owed is a decision, not a re-measurement.** Either the draw
is written down beside the figure (its distribution, its count, its
seed) so a re-take is a comparison, or the average goes and the worst
case stays, which is the half that reproduced and the half the
argument actually needs.

## The same defect at `SPLIT_SCAN_DECADES`, and what unit 6's fix pass did

Unit 6 filed this row and, in the same PR, **replaced one supremum over
an unrecorded draw with another over its own unrecorded 400,000-draw
search** (`floored_worst_excess`'s 2.0768%/2.0918% became 1.75256%).
That is this row's complaint restated by the row's own author.

The fix pass took the second branch there rather than the first,
because it could: **the sampled figure was never the argument.**
`floored_worst_excess` is a DERIVATION — a bisection on the placement
inside a sweep over `r` — and the family member `floored,
cross-term-free` sits at `r = 0.29808`, its analytic argmax, so the
derivations suite already carries the class's worst ratio on every run.
The random search only ever corroborated it. So the percentages from
all three draws are gone from `tools/tess-meter/src/lib.rs`, the
corroboration is stated as corroboration, and both claim sites point
here.

**That does not close this row**, and the reason is the figure this row
is actually named for: `best_split_scan`'s +14.93% average is NOT
corroboration of a derivation — it is the whole evidence for the guard
below it, and there is no closed form to fall back on. The decision
this row asks for is still owed there. What the fix pass established is
that the tree has two kinds of sampled figure and only one of them can
be answered by deleting it.

Fence: `tools/tess-meter/*`, METER's.

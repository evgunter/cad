# CHARTTAIL — the plan

Opened 2026-10-24 by CHART's priority-seam cut (`work/README.md`, Track
size). CHART closed every P0 and P1 row it kept and folded into this
track on 2026-10-26 (`docs/doc-ledger/chart-leaves-the-tracker.md`), so
the chart boundary and its consumers are one track again. The sitting of
2026-11-20 to 2026-11-23 closed every P0 and P1 row (U1–U8, in
`log.md`), and the track is `ready` with nothing dispatched.

## The slate

**12 P2, 20 P3 and 11 P4 rows**, with no P0 or P1. The only weight on the
load is one unbanded row, `cell-region-reach-reads-a-spline-cell-by-wide-eval`,
which the next orchestrator bands first, along with
`a-split-reduce-on-a-torn-body-panics-at-pcurves-unreachable-own-loop`, which
has no cost.

Two closed CHART rows are kept here because live rows lean on them:
`clearance-cells-are-decided-in-model-space` (the model-space cell
verdict's design record, whose measure half CLEAR's
`min-separation-tightening-crosses-the-drive` builds) and
`exact-region-cells-for-lower-bound-only` (one of the two reasons
`Certified::LowerBoundOnly` still refuses, cited there and by
`crates/editor-core/src/measure.rs`).

## Order

The P2 rows first. Three are hard:
- `a-planar-face-with-a-conic-trim-has-no-region-door` (H);
- `a-closed-collapsed-side-has-no-turn-or-side-reading` (H), the follow-up
  of the collapsed-side ruling (`[ev]` PR 7287), whose joint refuses at a
  closed side today;
- `a-seam-faces-image-refusal-is-dropped-so-off-and-uncovered-both-end-unplaced` (C).

CHART's two region-door rows,
`a-landings-edge-parameter-is-spelled-three-times-in-the-region-doors`
and `the-region-legs-spell-a-circle-and-a-line-beside-curve3`, are both
copies inside `region.rs` and its doors; they run as one unit, and touch
nothing INTENT owns. `azimuth-inf-arm-has-two-homes` (P3 E) rides with
them if its lane is on `chart_region.rs`'s `certified_arms` anyway.

The E rows go as sweeps: the refusal-text rows
(`chart-refusal-prose-outgrows-the-viewer`,
`chart-region-ray-exhausted-says-read-at-a-tighter-eps`,
`chart-region-ray-exhausted-tells-the-point-to-leave-a-boundary-it-is-off`,
`chart-region-corrupt-spells-its-kernel-defect-ending-by-hand`,
`chart-region-arm-unbounded-names-its-chart-by-string`) are one unit, and
the two `pcurves.rs` doc rows are another.

The spline-chart P3s filed off the collapsed-column unit
(`a-spline-charts-closure-is-read-at-a-band-and-folds-in-band-to-open`,
which carries Ev's "fix it later when necessary";
`a-spline-net-collapsed-over-its-whole-u-domain-reads-periodless`) wait for
a body that needs them.

## Wait on ATREST before building on the declared-pair door

`chart_region::declared_pair_overlap` is the door the at-rest census's
confirm pass reads. ATREST is rebuilding that census under D10:
`mates-declare-no-contact` (stage 4 I) is dispatched, and
`the-at-rest-census-is-a-check` (stage 5 C: nothing at rest refuses)
waits on it. Before dispatching these rows, read where that work left
the door:
- `a-planar-face-with-a-conic-trim-has-no-region-door`;
- `chart-region-cylinder-pair-gates-tilt-and-offset-one-at-a-time`;
- `chart-region-carrier-gates-read-vertices-not-arcs`;
- `chart-definite-diag-labels-an-interval-lower-end-as-an-f64-value`.

A row whose consumer has become a check rather than a refusal may change
band or retire.

## INTENT overlap

INTENT builds D10 at the document layer (`editor-core`, placements, the
census's records). The region-door rows are not on that ground; the
one consumer outside topo is `editor-core/src/clearance.rs`'s
`window_of`, which reads face geometry and is untouched by placement.
The declared-pair door rows are, through ATREST (above).

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). Each unit's
tier is named at dispatch in this program's log.

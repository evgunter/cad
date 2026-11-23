# CHART leaves the tracker — 2026-10-26

CHART (the chart boundary and its consumers: the polygon, the metring
arms and the clearance window that reads them) was opened on 2026-09-20
by TRIM's cut, cut SSI, PCERT and ISO out the same day, and cut its
P2–P4 rows into CHARTTAIL on 2026-10-24. Its sitting of 2026-10-24 and
2026-10-25 closed every P0 and P1 row: the region rows were answered
together by the model-space cell verdict (`topo::cell_region`, PR 5673,
which retired `chart_bound.rs` and `pcurves::chart_boundary`), and the
three region doors were put on one skeleton in `region.rs` (PRs 5664,
5671 and 5674); the chart inversion (PR 5659), the arms' tags and the
normalize sites (PR 5654) landed beside them.

With three rows left, all on the chart boundary's tail, CHART folded
into CHARTTAIL on 2026-10-26 (Ev, in chat). CHARTTAIL's title, charter
and `paths` (now with `cell_region.rs` and `chart_region.rs`) cover
both. CHARTTAIL is over its budget of 45 after the fold (47); Ev
accepted that, and its next orchestrator splits it when it opens.

Moved to `work/charttail/` by `git mv`, ids and bodies unchanged:

- `a-landings-edge-parameter-is-spelled-three-times-in-the-region-doors` (P2);
- `the-region-legs-spell-a-circle-and-a-line-beside-curve3` (P2);
- `azimuth-inf-arm-has-two-homes` (P3);
- two closed rows a live row leans on:
  `clearance-cells-are-decided-in-model-space` (the cell verdict's
  design record; CLEAR's `min-separation-tightening-crosses-the-drive`
  builds its measure half) and `exact-region-cells-for-lower-bound-only`
  (cited by that row and by `crates/editor-core/src/measure.rs`).

Deleted, closed, with nothing live leaning on their content:
`angular-arms-are-an-untagged-lever-beside-a-typed-rate`,
`cell-region-reads-an-ellipse-arc-as-its-whole-conic`,
`certified-arms-are-an-untagged-inf-rate-beside-a-typed-pair`,
`chart-bound-outer-span-decides-a-poisoned-margin`,
`chart-boundary-refuses-pole-crossing-sphere-loops`,
`chart-inversion-is-spelled-three-times`,
`clearance-window-cone-sphere-torus`,
`curved-face-containment-answers-none-on-a-ringed-face`,
`normalize-without-the-length-question-two-more-sites`,
`round-holes-get-no-chart-bound-benefit`,
`the-region-doors-share-one-skeleton-three-times`,
`the-sphere-ray-and-the-circle-leg-stand-outside-the-region-skeleton`,
`three-tables-of-the-chart-arms`,
`w2-describes-folds-a-corrupt-face-into-a-miss` and
`w2-reads-a-one-sheet-predicate-off-the-walk`. Several are still named
by path in other rows' origin notes and in `docs/K-REPORT.md`; this
note is where those paths resolve.

Its band 6200–6299 in `docs/MODEL-AB-LOG.md` closes with it. Its plan
set no `## Exit criteria`, so it closes without an exit walk.

Recover the rows and the program files from the commit that deletes
`work/chart/` (its parent holds them):

    git log --diff-filter=D --format=%H -1 -- work/chart/program.md
    git show <that sha>^:work/chart/<FILE>

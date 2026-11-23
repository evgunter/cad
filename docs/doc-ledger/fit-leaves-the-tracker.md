# FIT leaves the tracker — 2026-10-26

FIT (the camera, the display budget and the pick index: what the viewer
decides to show and how much of it) was cut from VGEOM on 2026-09-20,
when VGEOM measured 43 budget points, so that the two could run in
parallel (Ev, in chat, that day). Its sittings closed eleven rows,
among them zoom-to-fit framing committed profiles, ctrl-wheel zoom, the
id-readback failure reading as nothing under the cursor, the pick
index's rounded-t merge, and the display budget made a target with its
Flat bound (PR 5600).

At 6 points of P3 the parallel-track reason is gone, so FIT folded
back into VGEOM on 2026-10-26 (Ev, in chat). VGEOM's title, charter and
`paths` (plus `crates/viewer/src/pickcache.rs`) now cover FIT's ground.
VGEOM is over its budget of 30 after the fold (36.5); Ev accepted that,
and its next orchestrator splits it when it opens.

Moved to `work/vgeom/` by `git mv`, ids and bodies unchanged:

- `a-dense-floor-is-priced-off-the-law-from-the-scale-probe` (P3);
- `flat-harmlessness-is-unproven-on-the-nurbs-refinement-lane` (P3);
- `mispaired-ids-exempts-the-empty-window` (P3);
- `a-moved-copys-pick-span-crosses-its-frame-unconverted` (P1, parked
  on D′);
- `one-failed-root-refuses-the-whole-pick-index` (P2, parked on D′).

Deleted, closed, with nothing live leaning on their content:
`a-degenerate-frame-refusal-prints-an-aspect-nobody-can-act-on`,
`a-failed-id-readback-wait-leaves-the-buffer-mapping`,
`a-flat-rung-pair-is-read-as-flat-below-it`,
`a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value`,
`a-refused-camera-op-prints-its-finite-fields-at-full-precision`,
`ctrl-wheel-reaches-no-zoom`,
`id-readback-failure-reads-as-nothing-under-the-cursor`,
`name-of-answers-none-for-both-nothing-and-an-unassigned-id`,
`pickindex-merges-parts-on-a-rounded-t-it-never-converts`,
`the-budgets-predicted-count-is-not-always-an-over-count` and
`zoom-to-fit-frames-no-committed-profile`.

Its band 8500–8599 in `docs/MODEL-AB-LOG.md` closes with it. Its plan
set no `## Exit criteria`, so it closes without an exit walk.

Recover the rows and the program files from the commit that deletes
`work/fit/` (its parent holds them):

    git log --diff-filter=D --format=%H -1 -- work/fit/program.md
    git show <that sha>^:work/fit/<FILE>

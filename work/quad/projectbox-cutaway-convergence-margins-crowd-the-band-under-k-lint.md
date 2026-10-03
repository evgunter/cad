---
id: projectbox-cutaway-convergence-margins-crowd-the-band-under-k-lint
kind: issue
title: k-lint (dev-probe) flags projectbox_cutaway's props_quad_converged margins at 1e-9 and 1e-12 (eps-coupled headroom under 1.5e2·eps; one zero-classified near the coincidence threshold)
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Found by the REACH lane `reach/lily-leaf-1e12`. Until that lane, the nightly
`k-lint (dev-probe)` row never reached its lint step: `scripts/k_probe_sweep.sh`
panicked in its 1e-12 demo pass (`work/reach/lily-leaf-b-mass-exhausts-the-quadrature-budget-at-eps-1e-12.md`).
With the sweep exiting 0, `tools/k-lint` over the three fresh CSVs exits 2.
Most of its flags already have rows (`work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md`,
`work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md`,
`work/cleave/lily-walls-curved-clearance-crowds-the-band-under-k-lint.md`); this is one that had none.
The flag is main's: the demo CSV built from `origin/main` 55ba6226 (whose
1e-6 and 1e-9 demo passes run) carries the same row.

## The flags

At 1e-9, six rows on `demo/projectbox_cutaway` (`props_quad_converged`):
margins 1.33e-7 and 9.91e-8 (eps-coupled headroom under 1.5e2·eps), and
3.86e-10 zero-classified within 10^2 of the coincidence threshold — each
twice, the cutaway building the same faces twice. At 1e-12, eight rows,
4.85e-11 to 1.32e-10, eps-coupled headroom under 1.5e2·eps. A face whose
convergence margin lands within a few band widths of the target is a
round that barely certified (or barely did not); which faces, and
whether the next round would have cleared them, is unmeasured.

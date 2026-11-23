---
id: the-seam-split-rim-arms-have-no-native-fixture-once-a-full-revolve-is-one-band
kind: issue
title: The blend's seam-split closed-rim and hostless arms lose their native fixture once a full revolve is one band
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---


Found by the LATHE one-band lane (2026-11-23, PR 7414).

## What

The multi-link closed-rim band (`resolve_seam_split_rim`, the annulus
walk through chart-seam vertices) and the hostless rim arm
(`HostFoot::Strut`, `refresh_annulus_seams`) were built for a full
revolve of a pole-touching profile, whose curved walls were two π-bands
and whose rims were two arcs each (`crates/sweep/src/blend/surgery.rs`,
`battery.rs`; `docs/KERNEL-VERBS.md`, the fillet row). A full revolve's
curved wall is now one band and every rim one closed edge, so no native
door hands those arms a rim of two arcs at rest: a rim split by hand is
construction state (`fillet_h5_r2_probes::a_curved_single_face_carrying_both_arcs_is_construction_state`).
PR 7414 re-baselines the suites that read those arms on revolved
fixtures (`blend_seam_split_rim`, `fillet_h5_hostless_rim`,
`fillet_h5_r2_probes`, `review_fillet_h5_r1_probes`, `blend1_r1_probes`,
`review_blend1_r2_probes`, `verbs_arms3`) to the one-edge rims, which the
one-edge annulus door carves.

Still reaching the arms: STEP import's two-meridian pole form, until
`work/importgate/import-mints-the-wrap-tip-at-a-chart-singularity.md`
lands, and rims a boolean or split cuts into arcs. Decide whether the
arms keep fixtures from those producers or retire with the import flip.

## Ground

`crates/sweep/src/blend/`: band.

---
id: the-join-builds-what-the-rest-zip-builds
kind: issue
title: D10 stage 4 PR A: the join gains the three arms the declared-REST zip covers (partner-edge chord, curved-chart ring re-homing, the mekr NotSameFace cause), then boolean/rest.rs's surgery is deleted
status: review
opened: 2026-10-08
priority: P0
cost: H
branch: intent/s4-a-join
---

INTENT stage 4, PR A. Spec: `docs/INTENT-STAGE4-SPEC.md` §2. Needs nothing from stage 2 or 3; dispatchable now.

The build order `the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms` states: ring re-homing on a curved chart in aligned contact (the 41 cylinder-bore mates), the partner-edge chord (the 33 tangent plane×cylinder `SectionInvariant`s), the `mekr` `NotSameFace` cause (17), then `try_rest_union` (`crates/topo/src/boolean/rest.rs:191`), `RestZipFrontier`, `RestZipUnsupported` and the door at `ops.rs:896` are deleted. The carrier-pair doors move to `boolean/carrier_pair.rs`. The 91 unions' digests move (volume-checked); every other digest is bit-equal. May land as three PRs; only the last deletes the zip. Closes the zip row and releases 14 rows (spec §12).

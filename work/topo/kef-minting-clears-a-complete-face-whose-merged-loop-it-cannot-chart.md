---
id: kef-minting-clears-a-complete-face-whose-merged-loop-it-cannot-chart
kind: issue
title: kef_minting leaves a complete periodic face wholly unminted where the merged loop's re-mint cannot chart it
status: open
priority: P3
cost: M
opened: 2026-10-05
---


Found by the one-off kill probe of PR 4037's review fixes and PR 4039
(the R build), measured on both heads and on their merge base.

`Body::kef_minting` (`crates/topo/src/euler_kill.rs`) re-mints the
surviving face where the remnant's rows do not stand, through the site
mint (`pcurves::site_rows`). Where the merged loop cannot be charted,
the site mint answers `SiteRows::Clear` and the door drops every row of
the face. So a face whose rows were complete before the kill leaves
storing none: unminted, never half-minted. Nothing reads the state
today, because every producer runs its closing pass.

**Measured** (sweep's `ci` profile, f64, faces on a periodic chart that
either killed half is on, read after the outermost kill door):

- PR 4039's head `f1a04e59`: 404 complete → unminted, every one at
  `kef_minting`, every one all of the face's rows (6 of 6, or 4 of 4).
  168 cylinders, 128 spheres, 108 cones.
- PR 4037's head `41d6248d` and its merge base `8e68ba07`: 400, the
  same faces. The 4 more on PR 4039 come from sweep rows `main` added
  since.
- Why the re-mint clears, read at the `Clear` sites: 184 a joint no
  element meets (`decide_joint` → `Discontinuity`), 130 a carrier with
  no chart image (`chart_pcurve` refuses), 90 a map residual definitely
  over the band (`PcurveCache::certify`). The merged loop carries edges
  that do not lie on the surviving face's chart: a producer's kill
  across surfaces, mid-surgery.
- The tests: blend_tworims, fillet_h5_r2_probes,
  fillet_h5_hostless_rim, review_blend1_r2_probes,
  review_closed_chain_junctions_r1_probes, closed_chain_junctions,
  blend_seam_split_rim, contact_edge_must_carry, and others.

**Shapes:** the door could refuse rather than clear a complete face,
as the keys-only doors do with `KeysOnly`. The producers that kill
across surfaces could take the describing door
(`kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`).
Or the posture could state that a `_minting` kill may leave a face
unminted, which is a weaker contract than its docs' "re-mints the
surviving face".

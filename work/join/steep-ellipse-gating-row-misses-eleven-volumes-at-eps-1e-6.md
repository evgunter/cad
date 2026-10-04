---
id: steep-ellipse-gating-row-misses-eleven-volumes-at-eps-1e-6
kind: issue
title: pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed is red at eps 1e-6: 11 sound unions at theta 60 miss their closed-form volume by about 5e-8 relative
status: open
opened: 2026-10-04
---


Found by PR 3985's three-ε battery (`reach/arc-from-pairing`);
reproduced on main `e4a0a0d18` alone:
`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --all-features
--test all -E 'test(=pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed)'`
fails with the same `11 runs miss`. Green at 1e-9 and in hosted CI
(default ε).

## What

`crates/sweep/tests/pocket_ring_steep_ellipse.rs:639` collects the
runs whose outcome is off its expectation. At ε 1e-6, 11 runs at
θ = 60° that build a body passing tier 2, tier 3′ and the certificate
miss the closed-form volume, for example `x = (−0.95, 0.05)`, unmirrored,
A ∪ B: 3.2679290807 against 3.2679292977 (B ∪ A: 3.2679294534) — a
relative 5e-8 to 7e-8, which the row's volume check rejects. Whether
the row's tolerance should scale with ε or the volume should be tighter
is this item's question (not measured here); the ring trim on the steep
ellipse is in
`work/flux/an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane.md`'s
neighbourhood.

## Done when

The row is green at all three ε, by a tolerance argued from the band
or by a volume that meets the closed form.

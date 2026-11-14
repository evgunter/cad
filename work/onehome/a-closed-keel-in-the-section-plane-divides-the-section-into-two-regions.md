---
id: a-closed-keel-in-the-section-plane-divides-the-section-into-two-regions
kind: issue
title: A closed keel lying in the section plane divides the section into two regions where drop_slits promises one
status: open
opened: 2026-11-02
priority: P1
cost: M
---


Found by the ONEHOME orchestrator's delta review of PR 6365 (`onehome/small-bundle`).

## What

Take a full revolve of the annulus `0.5..3 × 0..2` about `y`, less a full revolve of a V whose tip lies on `y = 1`. Split it, or section it, by that plane. The keel is one closed circle of radius 2 in the plane, with material below on both sides of it. `plane_section` reads two regions, the annuli `0.5..2` and `2..3`. The right answer is one annulus, with the keel a slit inside it. That is what `section::drop_slits` promises, and what the open-ended keel already gets (`a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`).

Area and centroid agree either way. Region count and outlines do not.

## Why

This is a builder gap. The join's strut kill (`Sweep::strip_spurs`, through `Body::toward_free_end`) prunes a tree from its free ends. A closed edge has no free end, so the below side's section face stays divided by the coplanar circle. The delta review reads main as giving the same answer, so PR 6365 did not cause it.

## Witness

`crates/sweep/tests/a_curved_keel_in_the_plane.rs`, `a_round_keel_in_the_plane_reads_two_annuli_today`. It pins the current two-annuli reading.

## Owed

The section reads one annulus. Flip the witness when it does.

## The witness at ε 1e-6 (GREEN, 2026-11-14)

At `CAD_TOLERANCE_EPS=1e-6` the witness is red on main `b363f5fa1`, and has been since the first night it ran (nightly of 11-03, main `2d9dc809`): `the V's tip 0.6 ε off the plane, the plane's normal -1 y: the section's regions hold [11.780973958926847, 15.707964775914], not the annuli [11.780972450961723, 15.707963267948966]`. Both regions read 1.5e-6 over their annulus, against the row's `1e-8`. It is still two regions with one hole each, so the reading this row pins has not changed; the areas have moved by about `1.5 ε` with the tip `0.6 ε` off the plane.

Bisected, first-parent, at ε 1e-6: the witness does not exist at `89ca86d8e`, `af17f93b9` or `0ba834a0e`; it arrives in `e2be742e3`, the merge of PR 6365 (`onehome/small-bundle`), whose first parent is `0ba834a0e`, and is red there, as at `e55b75ca3`, `0adb9f691`, `5570df506` and main. So nothing moved it: the witness has never held its `1e-8` at ε 1e-6. (`work/green/main-red-a-round-keel-in-the-plane-reads-off-annuli-at-1e-6.md`, closed onto this row, read it as green at PR 6336, where it was absent.)

Owed with the flip: say whether a region's area may stand `1.5 ε` off its annulus when the keel's tip is `0.6 ε` off the plane, and state the witness's bound from that, or fix the reading.

The witness is under `test_utils::pinned_red::row` at ε 1e-6 on that message (`green/eps-1e-6`). Take the pin off with the fix.

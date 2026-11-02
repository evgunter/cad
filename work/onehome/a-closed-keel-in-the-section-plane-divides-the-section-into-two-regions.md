---
id: a-closed-keel-in-the-section-plane-divides-the-section-into-two-regions
kind: issue
title: A closed keel lying in the section plane divides the section into two regions where drop_slits promises one
status: open
opened: 2026-11-02
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

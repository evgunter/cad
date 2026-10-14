---
id: carrier-cross-one-curve-reads-tilt-apart-and-conic-helper-copies-conic-of
kind: issue
title: carrier_cross's one-curve margin leans on the tilt row's separate Zero, and its conic helper re-spells Conic::of
status: open
opened: 2026-10-12
priority: P3
cost: E
---

Found by the narrow re-review of PR 4782 (head d88f865dd1, verdict
MERGE); filed rather than held for, as style findings and notes.

## What

1. **Composition (likely).** `carrier_cross.rs` `one_curve`'s margin
   says it bounds the curves' distance "to first order", but it
   under-reads an out-of-plane tilt by up to √2 (ratio 1.41 for
   near-circles, 1.94 at a/b = 2). Its Zero therefore relies on the
   tilt row's own separate Zero: one decision inferred from two others
   (DESIGN.md Q1). Never measured past 1.5z in 1500 fuzz pairs per ε.
2. **A fresh copy (sure).** The new `conic(center, axis, major, minor,
   u_ref)` helper in `carrier_cross.rs` re-spells
   `geom_brep::Conic::of`'s Ellipse arm as a struct literal.
3. **Inside `carrier_speed.rs` (sure).** `speed_at` and `speed_hi`
   repeat the same Line and Circle arms, and the Circle arm duplicates
   `Conic::of`; `carrier_touch::speed_bound` now matches the carrier
   twice.
4. **Asymmetry (sure, harmless).** The turn term divides by e's
   (a + b) alone; swapping e and f moves the margin by up to 1.3e-2
   relative.
5. **Pins (sure).** The transverse ellipse × ellipse arm and the
   one-curve Positive branch have unit pins only.

## Fix

Fold the tilt into the one-curve margin so its Zero is one decision;
build the conic through `Conic::of`; one per-kind arm in
`carrier_speed.rs`; make the margin symmetric; add a body pin for the
transverse arm if a door reaches it.

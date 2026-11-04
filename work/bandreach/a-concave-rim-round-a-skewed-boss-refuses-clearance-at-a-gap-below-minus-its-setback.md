---
id: a-concave-rim-round-a-skewed-boss-refuses-clearance-at-a-gap-below-minus-its-setback
kind: issue
title: A concave rim round a skewed boss refuses face clearance at a gap of about minus the boss's width
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Found by `band/mixed-vertex-gate` (BAND, 2026-11-04), whose gate first lets
a concave rim round a boss reach this screen: before it, every corner of
that rim refused `UnsupportedCorner { MixedConvexity }` first.

**Witness.** A 6 × 4 × 0.5 plate with a four-sided boss standing on it to
z = 2 over the quadrilateral (1, 0.5), (1.5, 0.5 + s), (1.5, 3.5),
(1, 3.5) — `band_mixed_vertex::plate_boss`'s inset boss with its front
root edge skewed by `s` — and the four root edges at z = 0.5 requested,
chamfer or fillet at size 0.1. The walls are vertical and the plate's
top horizontal, so every requested dihedral is a right angle and each
corner is an isosceles turn; the bands fit with room to spare.

| s | result |
| --- | --- |
| 0 | builds, four mitres, `ΔV = +(d²/2·7 + 4·d³/3)` (`band_mixed_vertex::the_inset_root_rim_fills_the_mirror_wedge_at_each_corner`) |
| 0.1 | builds, four mitres, tier 3; the chamfer's `ΔV = 3.5896e-2` equals the mitred-offset closed form `Per·d²/2 + (d³/3)·Σ tan(θᵢ/2)` (θᵢ the outline's turning angles) |
| 0.3 | `FaceClearanceUncertified { face: 1v1, margin −0.1857, gap −0.0857 }` |
| −0.1 | `FaceClearanceUncertified { face: 1v1, margin −0.5903, gap −0.4903 }` |
| −0.3 | `FaceClearanceUncertified { face: 1v1, margin −0.9432, gap −0.8432 }` |

The gap is `margin + setback`, so it comes through
`battery::face_clearance_margin` from one of its two closed-form callers,
`reach::screened` (a support vertex's depth inside the strip its band
replaces) or `surgery::strip_clearance`; which one was not isolated. A
gap of about minus the boss's half-width to width is no distance the
fixture has: nothing of a support stands inside any band's strip. Both
verbs refuse alike, so the reading is the strip's, not a verb's.

The probe that measured it was `crates/sweep/tests/zz_probe_mixgate.rs`
on that branch (not committed); the fixture above rebuilds it in a few
lines from `band_mixed_vertex`'s.

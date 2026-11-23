---
id: a-meeting-on-an-arc-held-ellipse-away-from-its-anchor-is-certified-past-its-evaluators-rounding
kind: issue
title: curve-level doors (curve_pair::meet, the plane door, on_carrier, OnSurface::under) decide on spans past the session box that no door ever asked the box of
status: open
opened: 2026-11-23
priority: P2
cost: M
design: true
refs: [an-arc-held-ellipse-is-charged-a-centre-held-ones-resolution-and-hulls-its-phase-across-an-interval-seam-turn, an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre, a-section-conic-is-anchored-at-a-general-point-of-its-arc]
---

**The question.** The admission rule's ε covers the rounding of short chains on in-box coordinates (`geom_core::tolerance::ADMISSION_FLOOR`, the session box `SESSION_BOX_EXTENT` = 1e3 m). A model's coordinates are in the box. But the curve-level doors take spans, and no door asks whether a span's points lie in the box:

- `curve_pair::meet`;
- the plane door, `curve_face::pose::plane_conic`;
- `Curve3::on_carrier`;
- `OnSurface::under`.

So a door decides, unmetered, on a span reaching a megametre past the box, where a coordinate's spacing alone passes the band. Should such a span be refused at the door, metered, or read as it is? This is the design question. This row proposes no fix.

**The evidence.** The arc-held ladder of `oneround/conic-pair-readers` (PR 7543): `analysis/oneround/conic-pair-readers/ladder.rs`, judged by `oracle.py` at 60 digits against the curve the stored numbers state, and joined to the base pose by pose with `join.py`. Every pose carries whether both spans lie in the box (the harness's predicate: no coordinate past the box's extent at 4,097 points of each span, not the kernel's cover-based one) and whether the seam is at the anchor. On that unit's final head:

Fourteen readings are wrong, all at `f64` and all on a pose whose spans reach past the box. `Interval` reads none wrongly. In the box, with the seam at the anchor, the head reads none of the 864 per ε wrongly; the acceptance table is `analysis/oneround/conic-pair-readers/ladder/join-base-head.txt`. Joined to the base (PR 7510's head `0645d2b68` merged with the same main):

| ε | a | pose | in box | true gap | head | base |
|---|---|---|---|---|---|---|
| 1e-9 | 1e+08 fat | line, θ = 0.5, γ = 5° | no | 1.3e-09 (between) | a meeting 1.5e-08 off | refused (`curve_pair_walk_pair_gap`) |
| 1e-9 | 1e+09 fat | circle, θ = 0.5, γ = 90° | no | 9.1e-09 (between) | a meeting 3.1e-08 off | right |
| 1e-9 | 1e+09 fat | circle, θ = 0.5, γ = 30° | no | 4.8e-09 (between) | a meeting 3.5e-08 off | right |
| 1e-9 | 1e+08 thin | line, θ = 2, γ = 90° | no | 2.8e-09 (between) | a meeting 1.5e-08 off | wrong |
| 1e-9 | 1e+09 thin | circle, θ = 2, γ = 5° | no | 4.7e-10 (meets) | `Ok`, no meeting | wrong |
| 1e-12 | 1e+05 fat | circle, θ = 2, γ = 5° | no | 2.2e-13 (meets) | `Ok`, no meeting | refused (`curve_pair_walk_pair_gap`) |
| 1e-12 | 1e+06 fat | circle, θ = 2, γ = 90°, shift 20ε | no | 9.8e-13 (meets) | `Ok`, no meeting | wrong |
| 1e-12 | 1e+06 fat | circle, θ = 0.5, γ = 30° | no | 5.2e-12 (between) | a meeting 1.4e-11 off | right |
| 1e-12 | 1e+06 fat | circle, θ = 2, γ = 30° | no | 7.6e-14 (meets) | `Ok`, no meeting | wrong |
| 1e-12 | 1e+05 thin | line, θ = 2, γ = 5° | no | 4.2e-13 (meets) | a meeting 8.7e-11 off | wrong |
| 1e-12 | 1e+05 thin | circle, θ = 2, γ = 5° | no | 1.1e-12 (between) | a meeting 1.1e-11 off | wrong |
| 1e-12 | 1e+06 thin | line, θ = 0.5, γ = 30° | no | 1e-11 (between) | a meeting 1.1e-11 off | refused (`curve_pair_pose_closest`) |
| 1e-12 | 1e+08 thin | circle, θ = 0.0537, γ = 90° | no | 9.4e-13 (meets) | `Ok`, no meeting | wrong |
| 1e-12 | 1e+09 thin | line, θ = 0.017, γ = 90° | no | 2.3e-12 (between) | a meeting 1.7e-11 off | right |

Of the fourteen, the base read four rightly, refused three and read seven wrongly. Every one of them reads apart (`Ok`, no meeting) or a meeting placed too far off, for curves that meet or stand within the band. **A meeting reported for curves that are apart**, the other direction, is on the delta read's wall ladder (`analysis/oneround/conic-pair-readers/review2/ladder_wall.rs` on `oneround/conic-pair-readers-review`). It is past the box: ε 1e-12, thin `a` = 1e9, the line at 5°, θ = 0.0255, true gap 1.11e-11 (1.1 Kε), no twin shift. The base read it apart. The review counted nineteen on PR 7543's first head, whose twins stood `10ε` apart, on the escalation itself. The twins now stand `20ε` apart, and these fourteen are what is left. The review's own list, with each pose's evaluator error, is in its report (`analysis/oneround/conic-pair-readers/review-report.md` on `oneround/conic-pair-readers-review`).

**Three causes**, each a reading formed far from the point the conic stores:

1. **The evaluator's own rounding away from the anchor.** An arc-held ellipse evaluates `A + u·a·(cos θ − 1) + v·b·sin θ` (`Conic::point_at`, `crates/geom/src/curves/conic.rs`). Its point rounds at about `u·a·(1 − cos θ)`: exact beside the anchor, at the semi-axis's scale a radian away. Where that passes the band, `meet` reads the evaluator's point as the curve.
2. **`height_of` or `level_at` of a FOREIGN point formed far from the stored point.** A point formed 1.4e5 m from the anchor carries its coordinates' rounding (their spacing there is 2.9e-11 m), and the door reads it exactly. The review measured this on the ladder's `f64` line poses (ε 1e-9, `a` ≥ 1e8). `height_of` is off its own exact value by more than the zero level in 48 of 72 far poses, by up to 2.7e-8 m. Within 1 km of the anchor it is off by at most 7e-14 m. `on_carrier` shows the same at a second door: a definite `Off` inside the band's gap, far from the anchor.
3. **The stored seam out of square with the axis.** The doors take the frame as square (`conic.rs`, "A unit frame, by its type"). A frame minted square to its rounding (6.8e-17) departs from its plane by `a·|u·axis|·(1 − cos θ)` at θ from the anchor, which passes the band far from it (`work/onecopy/a-conics-harmonic-takes-its-stored-seam-as-square-to-its-axis.md`).

None of the three is a reader of where the conic is; each is the box's question asked of nothing. A meter of the evaluator's running bound (`Conic::running_point`, `Height::running_at`) would answer the first cause only. It takes the frame as exact and meters no foreign point. Whatever this row decides, it decides for the four doors at once.

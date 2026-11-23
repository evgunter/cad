---
id: an-interval-meeting-of-two-thin-arc-held-ellipses-spends-a-minute-to-refuse
kind: issue
title: curve_pair::meet at Interval spends about 70 s refusing (curve_pair_pose_basin) on two thin arc-held ellipses across planes at a = 1e9, where the centre-held twin refuses in 0.2 s
status: open
opened: 2026-11-23
priority: P3
cost: M
---

Found by `oneround/conic-pair-readers` (the K10 rows, 2026-11-23). `crates/geom-brep/src/curve_pair/arc_held_rows.rs`, `two_arc_held` (about line 452): two arc-held ellipses of the ladder's thin section are read against each other. The section has `a = 1e9` and `b = 1.04·√a ≈ 3.29e4`, its tip's radius of curvature is about 1 m, and the anchor is near `(1.3, −0.7, 0.4)`. The second ellipse is the first turned 0.7 rad about the arc's in-plane normal at a point 0.3 m or 5 m of arc from the anchor, then moved `30ε` along the pair's common normal.

At ε 1e-6, `curve_pair::meet` takes:

- about 2.8 s at `f64`, refusing `Unsettled(Meter { row: "curve_pair_pose_basin" })`;
- about 68 s at `Interval`, refusing on the same row.

The same pair, with the first ellipse replaced by the same locus held at its centre (`Ellipse::new` on its derived centre), refuses on the same row in 2.8 s at `f64` and 0.2 s at `Interval`. Every answer is a refusal, never a wrong one, so this is a cost only. The arc-held `Interval` reading spends some three hundred times the centre-held one's time before reaching the same refusal. Not traced: where the time goes (presumably the subdivision of the window, or the basin's polish loop, on the arc-held evaluator's wider enclosures), and whether the base, before this unit, spends it too.

**Reproduce:** `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p geom-brep --run-ignored only -E 'test(=curve_pair::arc_held_rows::two_thin_far_arc_held_ellipses_across_planes_meet)'`, about 140 s, green. The default run's `two_arc_held_ellipses_through_one_point_meet` leaves this one pose out for its cost.


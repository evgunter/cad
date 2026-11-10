---
id: every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12
kind: issue
title: Every mixed rim's section escalates its touch reading at Interval at eps 1e-12
status: open
opened: 2026-11-10
priority: P2
cost: M
refs: [the-shared-row-contact-class-is-levered-by-the-edges-length]
---

Filed by the band-mixchain lane (2026-11-10), from R1's re-review of PR 6520.

At the certified scalar at the tightest eps row (`CAD_TOLERANCE_EPS=1e-12`), an ordinary rounded plate refuses to fillet: the rectangle `[−1, 1] × [−0.6, 0.6]` with corner radius 0.3, at r = 0.05. So do the off-axis outlines, the barrel and the lattice-drawn plate and mouth.

The refusal is `Op { "surgery contact edge", Certification(Escalated { Transversality, shared_row_touch }) }`, raised at each junction's section. The reading is `geom_brep::row_contact`'s touch bound, the sine of the two bands' normals levered at the edge's extent. It encloses about `[0, 4e-12]`, inside that row's band, so it decides neither Touching nor Crossing.

The same rows build at Interval at eps 1e-9 and 1e-6, and at `f64` at every eps. The geometry is exact: both bands contain the section and share its normals.

The lever is the one `work/select/the-shared-row-contact-class-is-levered-by-the-edges-length.md` asks about. A touch bound levered on a length that does not shrink the enclosure with the band would decide here.

Pinned both ways by `crates/sweep/tests/band_closed_mixed_chain_shapes.rs::tightest_interval` / `assert_touch_escalation`: at Interval at 1e-12 every mixed-rim row must refuse with exactly this escalation, and everywhere else it must build. Closing this row flips that pin.

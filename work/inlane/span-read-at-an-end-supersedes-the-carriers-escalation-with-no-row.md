---
id: span-read-at-an-end-supersedes-the-carriers-escalation-with-no-row
kind: issue
title: Span::read supersedes the carrier's escalation at an end, and no row pins that choice
status: open
opened: 2026-10-18
priority: P3
cost: E
refs: [under-the-guided-lift-boss-unions-curve-pair-span-branch-flips-from-an-exact-zero-under-the-in-step-move]
---

PR 5257's review, mutant C. `geom_brep::curve_pair::Span::read` (`crates/geom-brep/src/curve_pair/span.rs`, the `splice_superseded` branch after `place_at`) reads the carrier first under `k_stats::detached`. Where the place is `End`, it splices that recording as superseded, which drops the carrier's escalation from the frame: the end answer does not rest on it. Swapping it for `splice` (the escalation kept live) leaves geom-brep's lib with `probe` (656 rows) and topo's containment rows green. The choice is right by the door's contract (`k_stats::splice_superseded`), but nothing tests it.

**The pose that would test it:** a reading that is `End` while `foot.on_carrier` escalates.
- **What it needs.** The end chord must decide Zero while the gap at the foot sits in the band's gap. On a line or a circle the foot is the nearest point, so the gap is at most the chord and reads Zero too. The pose therefore needs a carrier whose polished foot is not the nearest point.
- **Where.** An ellipse or spline foot on another stationary branch within the band's gap of the carrier, with the query point within the zero band of a span end. A hairpin spline whose two arms pass within the band of each other near an end is one candidate (`span_rows.rs`'s `hairpin()` is a start).
- **What the row asserts.** Read under a `k_stats::Bracket`, the answer is `End`, and `recorded.escalations` holds no `curve_carrier_*` escalation, while `recorded.verdicts` still holds the carrier's reading.

---
id: the-far-arc-end-reading-witness-stands-past-the-session-box
kind: issue
title: The census's far-arc end-reading witness stands past the session box, so its row no longer reaches the census
status: open
opened: 2026-11-21
priority: P3
cost: E
---

Filed by ONEROUND's `oneround/box-certify-ask` (certification asks the session box of an edge before any check).

`crates/topo/src/census.rs`'s row `a_far_arc_at_k_near_one_reads_its_end_where_certification_pinned_it` (the review's witness) built a cap on a circle of radius ~98.8 km at ε = 1e-9, where the conic's own frame and `Curve3::eval` part by rounding past `(K − 1)·zero` at `K = 1.001`, and pinned that a boundary pass reads the end certification pinned (`eval`'s), not the frame's. The arc reaches ~113 km past the session box, so its rim's certification now refuses it there, and the row is re-pinned as `a_far_arc_at_k_near_one_stands_past_the_session_box` (the box's refusal of that arc).

**What is left unpinned.** The census's end reading at `K` near one has no witness that reaches the census. The parting needs a radius near 1e5 m at ε = 1e-9 (scaled with ε), so an in-box witness may exist only at a finer ε, or not at all. Wanted: an in-box witness at some ε row, or a ruling that the defect is ruled out by the box and the admission rule, recorded at the census's end reading.



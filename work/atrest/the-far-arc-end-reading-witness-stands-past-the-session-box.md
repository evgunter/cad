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

`crates/topo/src/census.rs`'s row `a_far_arc_at_k_near_one_reads_its_end_where_certification_pinned_it` (the review's witness) builds a cap on a circle of radius ~98.8 km at ε = 1e-9, scaled with ε (~99 m at 1e-12, ~99 000 km at 1e-6), where the conic's own frame and `Curve3::eval` part by rounding past `(K − 1)·zero` at `K = 1.001`, and pins that a boundary pass reads the end certification pinned (`eval`'s), not the frame's. At 1e-9 and 1e-6 the arc reaches past the session box, so its rim's certification now refuses it, and the row pins that refusal and stands down there; at 1e-12 it stands in the box and reads its subject as before.

**What is left.** At the shipped ε (1e-9) and at 1e-6 the census's end reading at `K` near one has no witness that reaches the census. Wanted: an in-box witness at those rows (a smaller radius where the two spellings still part), or a ruling that the defect is ruled out there by the box and the admission rule, recorded at the census's end reading.

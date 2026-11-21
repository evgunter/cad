---
id: the-far-placement-mechanism-is-reached-only-past-the-session-box
kind: issue
title: The far-placement theorem-vs-numeric mechanism is reached only past the session box, which now refuses it first
status: open
opened: 2026-11-21
priority: P3
cost: E
---

Filed by ONEROUND's `oneround/box-certify-ask` (certification asks the session box of an edge before any check).

`crates/sweep/tests/sym11_far_placement_rows.rs` measured the first of SYM-11's two theorem-vs-numeric mechanisms: an ordinary body at `(d, d, d)`, `d ∈ {1e6, 3.7e7, 1e9}`, where the point channel's rounding exceeds the band on a form the tier proves zero. Every one of those placements is past the 1 km session box. Each far body's first edge now refuses at its certification, by the box, at every lane (bare `f64`, `Sym<f64>`, `Sym<Probe>`, `Sym<Interval>`) and every ε row, before any theorem is disputed, so the table reads `OutsideSessionBox` in every cell off the origin and the dispute column is zero there.

**What is left.** The file now pins only that the box refuses the far bodies; the mechanism it was written to measure (the four disputing cells, the `Sym<f64>` rescue at `(1e-9, 3.7e7)`) is reached through no door. Wanted: a ruling that the far-placement mechanism is ruled out by the box at a shipped run (the file's own prose already says the box and the admission rule rule it out together), in which case the file can shrink to that pin; or an in-box placement that still separates the channels, if one exists at 1e-12.



---
id: the-planar-strip-meter-reads-a-piece-across-and-along-its-rectangle-separately
kind: issue
title: blend: the planar strip meter reads a boundary piece's range across a requested edge's strip rectangle and its range along it separately, so a piece passing the rectangle's end reads straddling it
status: open
opened: 2026-11-12
priority: P3
cost: M
---


Filed by the band-meter lane (2026-11-12), from the sweep in the PR
that clips the support meter's pieces to a mixed link's window
(`a-mixed-ring-inside-a-few-wide-arcs-refuses-at-the-support-meter`).

`surgery::strip_clearance` (`crates/sweep/src/blend/surgery.rs`, arm (d)
of `ring_clearance_pass`) meters each boundary edge of a planar support
against the rectangle `{0 ≤ (p − o)·m ≤ across} ∩ {low ≤ (p − o)·d ≤ high}`
that bounds a requested edge's strip. Its margin is
`(lo_m − across).max(−hi_m).max(lo_d − high_d).max(low_d − hi_d)`: the
piece's range across the strip and its range along it, each read whole.
A piece that passes the rectangle's END — a hole's circle beside a
cut-off, or a line crossing the end at a slant — reads neither range
clear, since its sweep across runs through the strip's level and its
sweep along runs through the window, though it misses the rectangle.
Lines are not clipped here either, unlike the support meter.

The same reader already exists: the rectangle is a `StripLevel` with
`direction = Some(m)`, `at_trim = 0`, `rim_at = across`, `rim_above`,
and the one wedge `[(o + d·low, d), (o + d·high, −d)]`, and
`StripLevel::clearance` reproduces this margin as its whole-piece read
and then clips a line or circle to the window. What stops a drop-in
swap: the site decides `FaceClearance` (`face_clearance_margin`, with a
`gap`), while the clip's touch escalates `RingClearance`, and the
rectangle is itself a bound on the strip's quadrilateral, so the
clipped read is tighter but not exact. No witness is known. A round
hole does not reach this arm: it meters only the outer cycle and the
rings that carry a requested edge, and arm (a) takes every other ring
against the unbounded trimline. A witness needs an edge of one of
those cycles, not touching the requested edge's stations, passing the
rectangle's end: an arc of the outline (a notch, or a rounded tab)
beside a straight link's cut-off end, its range across through the
strip and its range along through the rectangle. Build one before the
fix.

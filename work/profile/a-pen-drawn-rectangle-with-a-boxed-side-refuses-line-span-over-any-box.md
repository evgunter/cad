---
id: a-pen-drawn-rectangle-with-a-boxed-side-refuses-line-span-over-any-box
kind: issue
title: A pen-drawn rectangle with a boxed side refuses the profile's line-span gate over any parameter box, on main
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Found by BOXFINISH's M3 measurement (2026-11-14, main `4c7404c2`; probe
row `m3_wide_slab` in `crates/editor-core/tests/boxfinish_m3_probe.rs`
on `boxfinish/probe-switch-off`, commit `bba45062`, never merged).

A rectangle drawn with the pen (heading 0, `Line(w)`, quarter turns,
closed with `LineTo(Start)`), its width `w` = 1 ± 0.05, replayed at
`Sym<Interval>` by the box driver: the profile node refuses before any
body is built,
`Profile(Escalated { site: SegmentPair(0, 3), gate: LineSpan, "line_span"
[-0.0150, 0.0248] })`, and the drive certifies 0 of 32 leaves. It
refuses the same way with the box-scalar switch on and off, and over
every box tried. The same rectangle given as four formula corners
builds.

Not traced: why the first and the closing segment's span reads in band
(inferred: the closing `LineTo(Start)` returns along a direction
recomputed from points that carry the box). BOXFINISH's
`the-profile-drops-the-unit-directions-its-verbs-made` records each
segment's direction at validate and may reach it; this row is the
measurement that says whether it does.

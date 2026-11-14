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

## Evidence (2026-11-14, BOXFINISH's profile-directions lane)

Recording each segment's direction at validate does not change this
row, by construction: `line_span` and the pair readings read the
segments' chords, never a recorded direction. Seen independently at
`Sym<Interval>`: a boxed rectangle and a pen-drawn one refuse
`LineSpan` at `SegmentPair(0, 1)` over a ±1e-6 box and pass at ±1e-11.
So "over any box" above means any box wider than about 1e-11 here.

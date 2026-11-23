---
id: a-pen-drawn-rectangle-with-a-boxed-side-refuses-line-span-over-any-box
kind: issue
title: A pen-drawn rectangle with a boxed side refuses the profile's line-span gate over any parameter box, on main
status: closed
opened: 2026-11-14
priority: P1
cost: M
closed: 2026-11-23
pr: 7264
branch: profile/pen-rectangle-over-a-box
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

## Traced (2026-11-20, PROFILE's pen-rectangle lane)

Two causes, measured unguided at `Sym<Interval>` through
`ConstructedProfile::validate`, `w = 1 ± h`.

**1. The pair pass re-reads an adjacent pair's shared vertex.** Both
spellings refuse `SegmentPair(0, 1)`, `LineSpan`, the margin about ±6h:
`[-6.0e-9, 6.0e-9]` at 1e-9, `[-6.0e-6, 6.0e-6]` at 1e-6, `[-0.3327,
0.2723]` at 0.05 (the corners pass only at 1e-11). `seg::line_line`
read `chord_side` of the shared vertex on the other segment's carrier,
then `touches_from` → `line_span` of it, `min(t, L − t)` with
`t = (q − a)·û/|û|`, `û = chord/len`. At the shared vertex `q = b`, so
`L − t` is zero by construction, but it is recomputed from the chord it
ends, with the box on `len`, on `û` and on `|û|`, and through radicals
the symbolic tier does not cancel. The `(0, 3)` refusal BOXFINISH
measured through the driver is the same reading at the closing pair's
shared vertex. Over plain `Interval`, with no symbolic tier behind it,
the side readings straddle too, and so does `judge_pair`'s
`contact_at_shared_vertex`, which measured `|v − v|`.

**2. The pen's fourth vertex cannot be ordered against the start**,
filed as `a-pen-loop-turned-by-a-rounded-quarter-cannot-order-its-start-over-an-interval`:
with cause 1 repaired, the pen turned by `fl(π/2)` refuses
`CanonicalOrderX` at `Loop`, over every box and a point box. Turned by
`Real::pi() / 2` (a document's `turn/4`) it validates.

Repaired (cause 1): `line_line` takes the pair's shared ends
(`seg::SharedEnds`, by index from `judge_pair`): the shared vertex's two
sides are Zero and its contact is `CKind::SharedVertex`, none read.

## Closed (2026-11-23, PR 7264)

Two adjacent lines share their vertex by construction, and the pair
pass recomputed it as a root of the two carriers, which it could not
order over a box. The pass now reads the shared end as a contact
(`SharedEnds`, `CKind::SharedVertex` in `line_line`), so the rectangle
validates over its box. The curved pairs still recompute their shared
vertex: `an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root`,
which closes with PLANAR's unit that moves profile onto the pair
reader. The second cause the trace found is
`a-pen-loop-turned-by-a-rounded-quarter-cannot-order-its-start-over-an-interval`.

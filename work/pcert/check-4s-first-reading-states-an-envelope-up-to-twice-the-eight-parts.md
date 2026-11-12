---
id: check-4s-first-reading-states-an-envelope-up-to-twice-the-eight-parts
kind: issue
title: Check 4's first reading states a sound envelope up to 2.1x the eight parts', which volume enclosures carry
status: open
opened: 2026-11-12
priority: P4
cost: E
---

Filed by the lane that closed
`check-4s-projected-door-reads-every-span-of-a-chord-in-eight-parts`
(PR 6864, its review's MINOR-3).

## What

Check 4 now reads a curved chart's net first at `Parts::ByTurn`
(`projected::run_projected_checks`), and a first reading that passes
states its own envelope. That envelope is sound but coarser.

Census: release, five crates' suites with the slow set, fuzz searches
aside, 6,663 reads. Against the same read cut 64 times over the edge:

| reading | median | p99 | max |
|---|---|---|---|
| eight parts a span | 1.00× | 1.01× | 1.18× |
| first reading | 1.60× | 2.08× | 2.09× |

The certificate's envelope feeds the edge's residual bound
(`certify::stated_bound`), and from there the volume enclosures.
`sweep/tests/cylinder_sphere_frame.rs`'s seven non-rim poses read pads
1.54–1.94× wider. That was measured on the same release binary with
check 4 forced to eight parts, and it moved their `MEASURED_PAD`
entries. Those pads are 2.4e-11 to 8.0e-10, against ceilings near
4.6e-6, so no consumer sits near a limit.

## What a fix could be

Floor the first reading at two parts a span: `ByTurn` with a minimum
count. A Bernstein hull's excess falls as the part's square, so this
roughly quarters the excess. It costs about twice the first reading's
composites, which is still about a quarter of main's eight. Take it if a
consumer needs the tighter envelope. Otherwise the cost win stands as
it is.

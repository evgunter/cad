---
id: self-touch-cause-arms-no-row-reaches
kind: issue
title: Ten self-touch refusal-cause arms no row reaches: their mutants survive
status: open
opened: 2026-11-12
priority: P3
cost: E
---


Found by PR 6854's mutant pass. Priority P3: each arm only labels a
refusal's cause (`InBand` against `Unreadable` or `NoTangentPlane`).
Swapping one never lets a point pass, but the recourse a user is told
would be wrong, and no row would notice.

## What

Each mutant below, run against
`topo --lib -E 'test(/link::|census::self_touch|census::tests::a_vertex_on_edge_record_off_a_circle/)'`
(f64 and `Interval`, default ε), survives:

| mutant | site |
|---|---|
| T6: the next horn read in band → `Unreadable` | `second_order::thorns`' `next` (`crates/topo/src/link/second_order.rs:122`) |
| T7: two rings in band of one ray → `Unreadable` | `thorns`' rays-apart check (`second_order.rs`, after the cycle loop) |
| A4: a nappe's rulings in band of each other → `NoTangentPlane` | `rest::whole_nappes` (`crates/topo/src/link.rs`). Red only at 1e-12, on `Interval`, 1 km off |
| K1: a pinch's `b` in band → `Unreadable` | `second_order::kind` |
| K4: a cap's facing in band → `Unreadable` | `second_order::kind` |
| P2: a sheet's side in band → `Unreadable` | `second_order::parts` |
| P3: a sheet's place in band → `Unreadable` | `second_order::parts` |
| P4: a bound's ray in band → `Unreadable` | `second_order::parts`, the `Place::Bound` arm (`second_order.rs:370`). See also `a-lens-candidate-dropped-in-band-refuses-unreadable.md` |
| P5: `misses`' undecided lean read as false | `second_order::misses` |
| A5: a nappe's axis read on a sector (`Material::On`/`Boundary`) → `Crossing` | `rest::at_apex`. Every plane of the rest is read decidedly off the axis point before this, so no row reaches it; the arm refuses by enumeration rather than by a wildcard |

## Two more, from the valid-bodies lane (branch `restread/self-touch-valid-bodies`)

Run against the same filter. These two arms refuse where their mutant
would read on, so they are guards and not labels; each was looked at
for a way to pass, and none was found:

| mutant | site |
|---|---|
| R14: an outer nappe's axis the rest leaves undecided (`material_round` gives `None`) is passed over | `rest::at_apex`. Every plane of the rest is read decidedly off the axis point, its tie's slack charged, before this, and the rest has read one material, so it has a reference that reads: no row reaches the arm |
| L14: a sector whose plane is in band of a line end's ray is taken as off it | `second_order::along_a_line`. The row that reaches the arm (`lens_rows`' plane in band of the tip's ray) refuses `InBand` under the mutant too: the remainder holds the plane cut along that ray, and reads the same sector in band of its bound |

## The fix

Give each arm a row that reaches it, in the style of
`link::second_order_rows` and `link::lens_rows`: f64 and `Interval`, at
the origin and 1 km off, with a reading set between the band's zero and
its escalation. Each row is red under its mutant.

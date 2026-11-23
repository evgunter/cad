---
id: a-station-far-from-the-origin-reads-indeterminate-on-its-own-carrier-and-panics
kind: issue
title: At eps 1e-12 and Interval a station of a face's own chart 30 or more off the origin reads indeterminate against its carrier, and the region door panics where the embedding ask should refuse
status: review
opened: 2026-11-22
priority: P1
branch: charttail/held-station
pr: 7626
---


## Finding

From `embed/first-finding` (PR 7319, 2026-11-22), on main `b4fd8ccf4`.
`topo::clearance::embedded` asked at `Interval`, `CAD_TOLERANCE_EPS=1e-12`,
of a plate whose void touches its top, built `(100, 100, 100)` off
the origin (`void_touching_plate_at([100.0; 3], false, 0)`,
`crates/sweep/tests/self_intersection_proves_injectivity.rs`, its
faces in their own order) panics in `cell_region::place`
(`crates/topo/src/cell_region.rs`):

```
a point held on its face's carrier (constructed as a station of the
face's own stored chart) reads Err(Indeterminate { margin:
Enclosure { lo: -2.637e-12, hi: 2.638e-12 }, band: Band { zero: 1e-12,
escalate: 1e-11 }, predicate: Some("held_on_carrier") }) against it
```

The station is the face's own chart evaluated at `Interval`; its
enclosure there is about 2.6e-12 wide either side, wider than ε at
that row, so `Held::constructed(..).check` cannot decide it on the
carrier and fails loud.

Measured, by this lane and by PR 7319's reviewer, the same plate built
at each offset and asked the embedding question:

| offset | `f64`, ε = 1e-12 | `Interval`, ε = 1e-12 | both scalars, ε = 1e-9 and 1e-6 |
|---|---|---|---|
| origin, `(0.1, 0, 0)` | reads | reads | reads |
| `(1,1,1)`, `(3,3,3)`, `(10,10,10)`, `(10,0,0)` | reads | reads | reads at 1e-6 |
| `(30,30,30)` | reads | **panics** in `cell_region::place` (`Held::check`) | reads at 1e-6 |
| `(100,0,0)`, `(100,100,100)` | reads | **panics**, the same site | reads |
| `(900,−900,900)` | reads | does not build: `newell_plane` escalates in the fixture | reads |

Which face's station it is was not measured.

## Band

P1, and a blocker of `embed/gate`'s landing (the reviewer's reading,
which this lane shares). Today no ordinary verb asks the embedding
question at `Interval`, so the panic is reached only by a test that
asks it directly. Once the at-rest gate lands every finish asks it,
and a body 30 units off the origin at the 1e-12 row would panic
inside the gate instead of being refused.

## What is owed

Whether the body should have finished at all at that ε (the brick and
the subtract both built) is not measured either. A width the scalar
cannot hold is a refusal of the question (`[tolerance.recourse]`),
typed, not a panic in the region door. The row
`a_body_that_does_not_pass_is_told_one_finding_or_another_and_never_that_it_holds`
does not ask, at `Interval` below ε = 1e-9, the two placements of its
list measured unread there (100 off: the panic; 900 off: the fixture
does not build), printing that it did not; `f64` asks every placement
at every ε.

## Measured and cured (`charttail/held-station`, 2026-11-23)

**Which station.** An env-gated probe at `Held::check` (not for merge)
over the plate at `Interval`, ε = 1e-12, `from = 0`: 306 checks per
placement, all in `cell_region::place`. The six brick faces' residuals
are exact (width 0) at every offset. The three slanted walls of the void
(Newell planes; axis `x` ≈ 0.860, −0.591, −0.135) carry frames that are
enclosures (about 1.8e-12 wide on a normal component at 100 off), and a
station of their chart reads:

| offset | worst half-width | worst centre | checks undecided |
|---|---|---|---|
| origin, `(0.1, 0, 0)` | 4.6e-14, 6.0e-14 | 1.9e-16 | 0 |
| `(1,1,1)` | 1.0e-13 | 3.7e-16 | 0 |
| `(3,3,3)` | 1.9e-13 | 6.4e-16 | 0 |
| `(10,0,0)`, `(10,10,10)` | 2.8e-13, 5.3e-13 | 1.3e-15 | 0 |
| `(30,30,30)` | 1.06e-12 | 3.8e-15 | 1 |
| `(100,0,0)` | 1.08e-12 | 6.7e-15 | 2 |
| `(100,100,100)` | 4.96e-12 | 8.4e-15 | 66 of 94 wall checks |

The width grows about linearly with the offset (≈3e-14 per metre of
`|o|`), and every enclosure is centred on zero to under 1e-14, about a
thousandth of its width: the station is on its carrier, and only the
enclosure is wide. It is the residual's dependency width: station and
carrier are one frame enclosure carried two ways, and the residual's
evaluation forgets that they move together. With the check suppressed,
every placement to 150 off reads `Refused(Budget { Depth, 1v1, 7v1 })`,
the finding it reads at the origin. From 200 off the fixture itself does
not finish at `Interval` (filed:
`work/oneround/a-planar-body-two-hundred-off-the-origin-does-not-finish-at-interval-at-1e-12.md`).

**Cure: (a), sharpened.** `topo::held::Held::check` now reads a
definite sign as the defect at every premise, and asks a constructed or
decided point for `Zero` only where its read is a point margin (`f64`,
whose rounding the session box bounds below the zero band). An
undecided enclosure is consistent with the construction: it holds the
true offset, zero. No `Held` premise, constructor or ONECOPY decision
moves: held points stay trusted and debug-checked. The ONECOPY ruling's
flip clause ("any trusting site reads a point non-`Zero` … i.e. a
construction stops holding its point on the carrier") is met here in
its letter and not its substance; the check now panics on the substance.

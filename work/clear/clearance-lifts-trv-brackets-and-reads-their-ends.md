---
id: clearance-lifts-trv-brackets-and-reads-their-ends
kind: issue
title: The clearance engine lifts brackets through from_certified and never refuses a Trv one, so 51 injective.rs and 17 clearance.rs endpoint reads decide on brackets that may not certify
status: open
opened: 2026-10-29
priority: P1
cost: M
---



Found by SPLICE's census lane (`splice/census-reds`) while dispositioning
`geom-core` `certified_endpoint_census` (`crates/geom-core/tests/certified_endpoint_census.rs`,
`ROSTER`), which counts `crates/topo/src/injective.rs` at (52, 1) and
`crates/topo/src/clearance.rs` at (23, 6) on main `0011f58a`. Those two entries
are pinned at main's counts with a note naming this row. The census
checks counts, not soundness. Closing this row means making each read
below sound, re-pinning the counts, and replacing the two notes with
dispositions.

## The shared cause: an ungated lift

`clearance::boundary_of`, `clearance::window_of` and `RegionMemo::ask` lift
vertices, span ends, pcurve points, iso values, harmonic coefficients and
carriers through `Interval::from_certified`. At `T = Interval` that keeps
a refused input's ordinary endpoints under a `Trv` decoration (its own
doc, `crates/geom-core/src/interval.rs` `from_certified`), and nothing
downstream refuses it. `T = Interval` is a production path (`lane.min_separation`
in `editor-core`'s `wire.rs`, the `m10_6_min_clearance_interval` row).
The same code asks the question in some places and not others:
`extent_of` asks `is_certified()` of the lifted pcurve ends, and
`iso_bounded_domain` asks it of knots and joins. The `Use.image`,
`Iso.at`, `Harmonic` values, `Rim.arc` span and `vertices` are never
asked.

**R1, the minimal fix:** refuse at the lift. In `boundary_of` and
`window_of`, return `None` / `Unsupported` for any lifted value that does
not certify (`certified_bracket()`, or `from_certified` then
`is_certified()`).

## clearance.rs (17 reads outside an asking function)

- `min_separation` (3) and `Bracket::pair` (3): the poison check is
  `sep.lo().is_finite() && sep.hi().is_finite()`, not `is_certified()`,
  so a `Trv` separation reaches the reported bracket's `floor`/`lo`/`hi`.
  Add `|| !sep.is_certified()` → `PoisonEnclosure`, on top of R1.
- `Bracket::witness` (1): `stations.hi()` becomes the incumbent upper bound
  unasked. Skip a station separation that does not certify.
- `Bracket::overflow` (1): an abandoned task's `sep` is never checked, and
  `floor.min(NaN)` drops a NaI lower end, so a poisoned pair falls out of
  the lower bound. Poison, or floor at 0, on a refused `sep`.
- `window_of` (2): the `along` superset reads a lifted origin and
  direction (R1). The cone `slant` read is covered by its own
  finiteness refusal once R1 holds.
- `boundary_of` (1): the lifted span's ends feed `carrier.eval` and
  `Aabb::from_points`, which reads `Bounds` and refuses nothing (R1).
- `split_axis` (3), `coincidence` (1), `meeting_stations` (2): these are
  sound. They choose an axis or seed a search whose result is verified.

## injective.rs (51 reads outside an asking function)

Sound once R1 holds:
- `overlap`, `covered`, `vacuous`, `present_curved`, `graph`'s `rb`,
  `runs`, `Ends::before`/`after`, `apart`, `edge_piece`;
- `present_chart`'s touch/reach, rectangle and apex reads.

Need a local ask besides R1. Each of these rebuilds an interval from
`lo()`/`hi()` with `from_bounds`, which mints a certified one from a
refused one:
- `forward`: `delta.lo() > 0` picks the face side (`delta.is_certified() &&`).
- `along`: `from_bounds(min lo, max hi)` feeds `covered`'s coverage claim.
- `present_chart`'s merge hull.
- `Trig::psi`'s `over`: `from_bounds(-m.hi(), m.hi())` goes to `is_zero`.
- `Curved::psi`'s `bound = (...).hi()`.

Unsound whatever the lift:
- `present_plane`'s `[e]` arm returns `Vacuous` on `y.hi() <= 0.0` before
  the SPAN funnel. `y = (d·bv)/len2` with `len2` possibly reaching zero
  is a quotient that may not certify. Fix: `y.is_certified() && …`.
- `present_chart`'s sliver: `at(lever.norm().hi())` mints a certified
  width from a lever `rect_jet` may have refused, then decides
  `SLIVER == Zero` → `Vacuous`. Fix: keep `lever.norm()` as the interval.
- Also: the merge folds use `f64::max`/`min`, which skip a NaN clip line
  at `T = f64` instead of refusing it.

Already sound:
- `mid`, `graph`'s `inside` and `lever`: seeds and selections.
- `present_plane`'s reads after the SPAN and CORNER funnels.
- `Pieces::of`'s `wind`, behind `exactly`.
- `monotone_run`, `narrowed` and `Curved::psi`'s shift reads: `from_bounds`
  of finite parameters.

The per-read reading is the lane's own, at the read sites. Check each
claim before fixing; none has a failing test yet.

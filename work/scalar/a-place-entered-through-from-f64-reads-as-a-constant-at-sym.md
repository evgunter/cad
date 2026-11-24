---
id: a-place-entered-through-from-f64-reads-as-a-constant-at-sym
kind: issue
title: A place entered through from_f64 reads as a constant at Sym, and the gate files it as one
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Found by `a-place-is-an-f64-argument-answered-on-the-value-channel`'s fix pass (both reviews of PR 7250). A place, DL5.2 (`crates/geom-core/README.md`), is an `f64` the code chose to evaluate at; it is handed to an evaluator as an `f64` (`geom`'s `eval_at`, `deriv_at`, `eval_in_span_at`), whose one step to the scalar is opaque at `Sym`, so a re-valuation at other values of the unknowns refuses rather than reading at a point chosen for the nominal ones. The sites below enter such a point through `from_f64` instead, an exact literal at `Sym`, and `scripts/gates/from-f64-sites.tsv` files each as a `constant`, since the gate has no class for a place. Its notes say what each is ("a point the partition chose", "midpoint", "a point the chain chose").

`from-f64-sites.tsv` rows, by file and argument:

- `geom-brep/src/pcurve_cache.rs` `l`: a line's critical parameter, "a place in the span to read the distance at".
- `geom-brep/src/pcurve_cache/closed_bound.rs` `mid`, `t` (the span's midpoint; a grid point of the span's outer ends); `h` is a half-width, a bound, not a place.
- `geom-brep/src/pcurve_cache/projected.rs` `a`, `b`, `s0`, `s1`, `f64::from(k) / f64::from(cuts)`: partition breaks, "a point the partition chose". `v` is already filed as `launders`.
- `geom-brep/src/ssi/certify.rs` `geom::mid_param(a, b)` (a refined span's midpoint, where the chain box's axis is read and then stored in the box: a choice of direction, so this one may be data rather than a place), `mid` (a chain box's midpoint), `tb` (an interior point limb 2's subdivision chose).
- `topo/src/splitting/spline_arc.rs` `m`: a piece's middle parameter, `self.curve.eval(T::from_f64(m))` under `T: Decide` (admits a dual), stored in `PieceBall.on`: trace whether the ball is kept before moving it.

Moved by that fix pass, so off the list: `ssi/certify.rs` `w.mid` (`probe_tube_chart`'s window midpoint, now `deriv_at`).

Not done there, and why: giving the gate a `place` class would bless the spelling a place should not have. The fix is each site through a place-taking evaluator, which takes the row off the gate; where no such evaluator exists for the type (a ruled cell, a projected partition piece), the row says which is needed.

## The gate's entries, 2026-11-24

`from-f64-is-split-by-where-the-number-comes-from` retired the gate's `constant` class and read every listed site against DL5.2's four sources. The places are now listed `owed` to this row in `scripts/gates/from-f64-sites.tsv`, each with its function. Beside the sites above, the reading found these, which the old list also filed as constants:

- `geom-brep/src/certify.rs` `held_station`: an interior station `schedule_param` computes at `f64` from the domain's ends.
- `geom-brep/src/edge_wedge.rs` `WedgeWalk::around_circle` (`phase`): a station on the rim, kept in a `Station`.
- `geom-brep/src/pcurve_cache/stated.rs` `iso_net` (`t`): an end of the domain an iso line is read at to build its net.
- `geom/src/cover/readings.rs` `cover_over` (`cut`): a spline window's interior cut.
- `geom/src/curves/nurbs.rs` `derivative_at` (`at`) and `sweep/src/skin.rs` `read_tangent` (`at`): the caller's `f64` parameter entered to seed a poison value and to evaluate the path.
- `topo/src/replace_face.rs` `mid_domain_normal` (two): the domain's centre a chart normal is read at.

Off this row: `projected.rs` `f64::from(k) / f64::from(cuts)` is `Real::count_ratio(k, cuts)` now. It is a dyadic fraction of a count, the same at every valuation, where `a`, `b`, `s0`, `s1` beside it are breaks a partition chose from values.

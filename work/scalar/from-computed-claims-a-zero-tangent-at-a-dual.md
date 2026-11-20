---
id: from-computed-claims-a-zero-tangent-at-a-dual
kind: issue
title: Real::from_computed returns tangent zero at a dual, so a re-entered value that moves with the inputs yields a plausible wrong sensitivity; and certification interiors admit a dual by type
status: parked
opened: 2026-11-19
priority: P1
cost: H
blocked_on: [7155]
refs: [dl5-has-no-arm-for-a-selection-whose-result-is-the-payload, census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection]
---


Filed by the SCALAR orchestrator, 2026-11-20. The build of DL5.2
(`crates/geom-core/README.md`, "Reading a bracket where nothing
certifies"), parked on Ev's answer to PR 7155.

## What

`Dual::from_computed` is `Self::constant(T::from_computed(x))`:
tangent zero, the claim that the value does not move with the inputs.
`Sym::from_computed` is an opaque unknown. `Real::from_computed`'s own
doc says a scalar that defaulted it to `from_f64` "would re-enter
every computed value as a constant", and at a dual that is what it
does.

A survey of the 157 calls outside tests (read, not executed; counts
and lists in `work/scalar/log.md`, 2026-11-20): 57 a dual cannot
reach, about 60 only decided on or reported, 9 constants, about 10
flat to first order by a comment's argument, 10 wrong, 10
undetermined. The wrong one on the path production runs is ATREST's
`link-lowest-on-freezes-its-candidates-so-a-dual-sensitivity-through-a-tie-drops-a-term`.

## What closing it takes

Two designers' count (rounds 6 to 10 of the fork; read, 22 place sites
traced to consumers): 114 lines, 128 calls, are in a dual's reach by
the enclosing function's own bound. About 42 lift a bound to decide
against, about 55 a place to evaluate at (with what were once called
free choices), 9 a solved value the result is built on, 13 data built
at `f64`, 3 a constant. Every unit re-sorts the sites it touches.
The rule is DL5.2 and DL5.3 of `crates/geom-core/README.md`.

- **Bounds stop being `T`**: a pure `f64` bound is decided at `f64`
  (`pcurve_cache/projected.rs` `hull_sector` lifts `x_lo` only to call
  `holds`); a bound subtracted from a `T` quantity is an `f64`
  allowance term of the margin. `Margin::<f64>::lift` is itself a
  re-entry and `Margin::value` is a public `T` exit.
- **Places**: `Real` gains an associated `Value` (the scalar with its
  tangent removed) and a read down to it; evaluators take an `f64`
  place and answer on the value channel. `SplineWall` already keeps an
  `f64` copy of its chart; `spline_trim`'s `Patch<T>` ends only in
  `Side`/`Reading` and runs whole at `T::Value`.
- **The solve door**: the `f64` answer and the residual at `T` in, the
  same value out with the implicit-function tangent at a dual;
  identity at `f64` and `Interval`, the degraded tangent at
  `Dual<Interval>` and at a singular slope. About 9 sites
  (`curve_face/composite.rs` and `spline_face.rs` `settle`,
  `replace_face.rs` `corner_of_root`, both polishes, the projection
  callers that go round `project`'s fence). `topo::Link::lowest_on`
  needs none: it keeps the `T` roots it already computes.
- **`f64`-built data** is built at `T` or lifted behind
  `CertifiedEnclosure` (the loft's enclosure arm, already owed as
  `launders` in `from-f64-sites.tsv`; `on_carrier_domain`; `skin.rs`).
- **`from_computed` leaves the surface dual-reachable code calls**,
  last: a remaining call fails to compile at a dual, which is the
  signal.
- **`from_f64` is split by source**: a compile-time-constant door for
  literals, an integer door for counts, the tolerance type for bands,
  the document's parameter door for authored values. The list
  (`from-f64-sites.tsv`, 202 rows, 257 sites) holds a computed lever
  scale as a `constant` (`coincide/replay.rs`).
- **DL5.3**: the `Certifies` marker bound; lane structs declared over
  it (`NurbsLane`, `FittedLane`, `SplineFoot`, `SectionLane`,
  `OffsetFitLane` are `T: Real` today, `QuadLane` `T: Decide`);
  `AtRestPolicy`'s lanes an associated type with no values at a dual
  in place of `Option`; a gate over listed certification files; one
  clause in the `Bounds` scope rule.
- A finite-difference row on the stackup's measurements, which would
  have caught `lowest_on`.

## Check before building (nobody has)

- Whether the roughly 9 untraced place sites fit the value channel,
  and whether reading a chart down to it allocates at a certifying
  scalar.
- Whether every certification-interior item can take `Certifies`
  without a construction caller being split off it, and whether any
  reads a bracket through a bound it inherits.
- Whether a compile-time-constant door covers every `constant` row;
  it has not been compiled.
- Whether `Sym` needs a computed place to be opaque inside the
  evaluator.
- Whether a pure-`f64` bound decided at `f64` gives the verdict it
  gave decided at `T` at every scalar.
- `props.rs` `volume_lo`/`volume_hi`/`surface_area` hold bracket ends
  in `T`-typed fields of a reported `VolumeEnclosure<T>`: whether the
  stackup reads them.
- `Interval` division of an exact zero by a slope enclosure, for the
  solve door's bit identity.
- Re-entry that avoids the door: `Interval::point`, the `launders`
  rows of `from-f64-sites.tsv`, and a `value_channel` map followed by
  evaluation at `T` (`SplineWall::outward`, which also goes round
  `project`'s `CertifiedBounds` door).

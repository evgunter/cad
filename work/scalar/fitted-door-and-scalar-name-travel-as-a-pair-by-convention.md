---
id: fitted-door-and-scalar-name-travel-as-a-pair-by-convention
kind: issue
title: The fitted door and the scalar's name travel as two parameters tied by convention (recertify, certify_general, map_approx)
status: closed
branch: scalar/scalar-name
pr: 3461
opened: 2026-09-25
closed: 2026-09-29
priority: P4
cost: D
---


## What

Three geom-brep/topo doors take a scalar's door and that scalar's name
as two separate parameters, and nothing ties the name to the scalar
the door (or its absence) belongs to:

- `geom_brep::PcurveCache::recertify(.., lane: Option<FittedLane<T>>,
  scalar: &'static str)` (`crates/geom-brep/src/pcurve_cache.rs`);
- `geom_brep::PcurveCache::certify_general(.., lane, scalar)`, which
  took the same pair in LANE-4's fix pass, because the mint reaches it
  at every scalar and an absent door must refuse at check 4;
- `topo::transform`'s private `map_approx(.., offset_fit:
  Option<OffsetFitLane<T>>, scalar: &'static str)`
  (`crates/topo/src/transform.rs`).

Every production caller passes `T::fitted_lane()`/`T::offset_fit_lane()`
beside `T::scalar_name()` from `topo::AtRestPolicy`, so the pair agrees
by convention. A public call like `recertify(.., None, "f64")` on an
`f64` cache compiles and refuses "the f64 scalar … may not certify".
`recertify` also asks every caller for the fitted pair on its
closed-form arms, which ignore it (e.g. `crates/sweep/tests/revert_plane_charts.rs`
passes `<f64 as AtRestPolicy>::fitted_lane()` for a plane `IsoLine`
row).

Raised by both LANE-4 reviewers (R1 N4 and S2, R2 S3).

## Why it is not fixed where it was found

geom-brep cannot name topo's policy, so the name has to travel in
from topo; the fix is a bundling shape (a value carrying "the door, or
the name of the scalar that holds none" — `run_fitted_checks` already
takes `Result<FittedLane<T>, &'static str>` privately), which changes
three public or crate signatures and every caller. That is its own
unit, not a fix-pass item.

## Decided (SCALAR-NAME)

The bundled door-or-name value is rejected: its public constructor
would let any caller write the absent arm's name, so the convention
would only move. Instead the name lives on the scalar:

- `geom_core::Real` declares `const NAME: &'static str`, with no
  default, beside `WITNESS` in each impl (`"f64"`, `"interval"`,
  `"telemetry probe"`, `"symbolic"`, `"dual"`);
- `topo::AtRestPolicy::scalar_name()` and `editor_core::lane::Lane::NAME`
  are deleted, and every refusal that names a scalar reads `T::NAME`
  off its own type parameter;
- the `scalar` parameter leaves `recertify`, `certify_general` and
  `map_approx`; the door stays an `Option` (H5 ruling 3), `recertify`
  keeps one signature taking it on every arm, and an absent door still
  refuses at check 4;
- the fitted and offset-fit refusals' text is true whether `None` means
  the scalar lacks the right or a caller at a certifying scalar
  withheld the door, and the three offset-fit refusals name the scalar
  in a `scalar` field.

## Cost

D: one `Real` const in five impls, three signatures and their callers
in topo/sweep tests and the `r2_p2_consumer` example, two name homes
deleted, and three offset-fit refusal shapes.

## Closed (2026-09-29) — PR 3461 (SCALAR-NAME)

One name source: `Real::NAME`, read as `T::NAME` by every refusal that names a scalar; `AtRestPolicy::scalar_name` and `editor_core::lane::Lane` are deleted, and the replay lists are built from `Real::NAME` with their membership pinned against the policy.

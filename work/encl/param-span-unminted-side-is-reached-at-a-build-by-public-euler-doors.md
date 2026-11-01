---
id: param-span-unminted-side-is-reached-at-a-build-by-public-euler-doors
kind: issue
title: geom-brep/topo: ParamSpan's unminted refused side is reached at a build from public input (Body::mev's caller span, mev_line's coincident points), so its build panic cannot land
status: closed
pr: 5365
opened: 2026-10-19
branch: encl/param-span-doors
closed: 2026-10-19
---


(Filed by the ENCL implementer of `a-proven-zero-is-definite-yet-ends-as-no-size-at-rest`.)

## What

`[tolerance.recourse]` (iv), as ratified on PR 5134, says a side no kernel construction produces panics at a build, and names `CertCheck::ParamSpan` unminted. The implementation could not land that panic: public input reaches a zero or reversed span at a build, and the build door renders it (`EulerOpError`'s `Display` reads `Reading::Build`, `crates/topo/src/euler.rs`).

- `topo::Body::mev` (`crates/topo/src/euler.rs`) is a public door that takes the caller's `EdgeCurveSpec`, whose `param_start`/`param_end` are public fields (`crates/geom-brep/src/certify.rs`, `EdgeCurveSpec`). A caller passing `param_end <= param_start` reaches `CertifyError::IntervalNotForward` at the span check (`certify.rs`, Check 2, `forward`).
- `topo::Body::mev_line` and `mef_chord` build the chord through `EdgeCurveSpec::line_between`, which mints `0 … |p1 − p0|`; coincident points give a span of exactly zero. `seqgen.rs` (`SPLIT_FRACTION`'s doc) records `mef_chord` refusing a zero-length chord this way.
- `topo::replace_face` (`ReplaceFaceError::ReanchorCollapse`'s doc) reaches a non-forward span from the user's offset distance; it re-maps lines and splines to its own refusal, but a periodic carrier's stays the attach door's `Op` and renders through `CertCheck::ParamSpan` at a build.
- A circle or ellipse carrier with a non-positive radius or minor semi-axis meters the span non-positive (`certify.rs`, `span_at_floor`'s comment), also through `mev`.

So `RefusedSide::Unminted` ends at a build in the lever and its note, as a stored contradiction does (`crates/geom-brep/src/recourse.rs`, `SizedDecision::words`). Its note still says "an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect worth reporting", which is false of `mev`'s caller and of `mev_line`'s coincident points.

## Repair shape

This is the ratified clause applied, not an open design question. `[failure]` and D2 refuse input typed at its door, so only a kernel bug reaches a check that runs after the door; (iv) then has a side no kernel construction produces panic at a build. The repair is therefore:

- the public Euler doors refuse a non-forward or degenerate caller span typed, before certification:
  - `Body::mev` with a caller's `EdgeCurveSpec` whose `param_end <= param_start`;
  - `mev_line` and `mef_chord` through `EdgeCurveSpec::line_between` with coincident points;
  - `replace_face`'s periodic carrier left non-forward;
  - a circle or ellipse carrier with a non-positive radius or minor semi-axis;
- with the doors in place, only a kernel bug reaches `CertCheck::ParamSpan`'s span check at a build, and its build panic lands as (iv) says. The build note's "no kernel construction mints" is then true.

`ParamWinding` falls under the same door check: a span past a full turn is refused at the door (`mev` with a circle spec past a full turn reaches `WindingExceeded` at a build today). Once that lands, `ParamWinding` becomes `RefusedSide::Unminted`, not `StoredContradiction`.

Rejected: making `ParamSpan`'s build side the caller's geometry (a stored contradiction, the lever at a build, the note dropping "no kernel construction mints"). It changes the ratified row assignment.

## What this row separates

`RefusedSide::StoredContradiction` and `RefusedSide::Unminted` currently behave identically at every door (`SizedDecision::words` in `crates/geom-brep/src/recourse.rs` gives them the same arms). This row is what separates them: once the doors above refuse typed, `Unminted` panics at a build and `StoredContradiction` keeps the lever there. (Reviewer's note on PR 5349.)

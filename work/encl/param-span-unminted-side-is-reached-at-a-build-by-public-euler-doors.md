---
id: param-span-unminted-side-is-reached-at-a-build-by-public-euler-doors
kind: issue
title: geom-brep/topo: ParamSpan's unminted refused side is reached at a build from public input (Body::mev's caller span, mev_line's coincident points), so its build panic cannot land
status: open
opened: 2026-10-19
---


(Filed by the ENCL implementer of `a-proven-zero-is-definite-yet-ends-as-no-size-at-rest`.)

## What

`[tolerance.recourse]` (iv), as ratified on PR 5134, says a side no kernel construction produces panics at a build, and names `CertCheck::ParamSpan` unminted. The implementation could not land that panic: public input reaches a zero or reversed span at a build, and the build door renders it (`EulerOpError`'s `Display` reads `Reading::Build`, `crates/topo/src/euler.rs`).

- `topo::Body::mev` (`crates/topo/src/euler.rs`) is a public door that takes the caller's `EdgeCurveSpec`, whose `param_start`/`param_end` are public fields (`crates/geom-brep/src/certify.rs`, `EdgeCurveSpec`). A caller passing `param_end <= param_start` reaches `CertifyError::IntervalNotForward` at the span check (`certify.rs`, Check 2, `forward`).
- `topo::Body::mev_line` and `mef_chord` build the chord through `EdgeCurveSpec::line_between`, which mints `0 … |p1 − p0|`; coincident points give a span of exactly zero. `seqgen.rs` (`SPLIT_FRACTION`'s doc) records `mef_chord` refusing a zero-length chord this way.
- `topo::replace_face` (`ReplaceFaceError::ReanchorCollapse`'s doc) reaches a non-forward span from the user's offset distance; it re-maps lines and splines to its own refusal, but a periodic carrier's stays the attach door's `Op` and renders through `CertCheck::ParamSpan` at a build.
- A circle or ellipse carrier with a non-positive radius or minor semi-axis meters the span non-positive (`certify.rs`, `span_at_floor`'s comment), also through `mev`.

So `RefusedSide::Unminted` ends at a build in the lever and its note, as a stored contradiction does (`crates/geom-brep/src/recourse.rs`, `SizedDecision::words`). Its note still says "an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect worth reporting", which is false of `mev`'s caller and of `mev_line`'s coincident points.

## Question (design)

Either:

1. the public Euler doors refuse a non-forward caller span typed before certification (and `line_between` refuses coincident endpoints), so only a kernel bug reaches the span check at a build, and the build panic lands as ratified; or
2. `ParamSpan`'s build side is the caller's geometry, so it is a stored contradiction (lever at a build), and the note drops "no kernel construction mints" at a build.

`ParamWinding` was ruled the same way for the same reason: `mev` with a circle spec past a full turn reaches `WindingExceeded` at a build.

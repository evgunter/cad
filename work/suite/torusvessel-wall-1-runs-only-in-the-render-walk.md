---
id: torusvessel-wall-1-runs-only-in-the-render-walk
kind: issue
title: torusvessel's wall 1 runs only in the render walk — D403's class, second instance
status: open
opened: 2026-09-15
---


## What

`demos/tour/src/torusvessel.rs` runs one `crate::walls::wall` probe —
wall 1, the SECTIONED vessel's rim, pinning
`ShellError::Face { .. TogetherAxialEdge .. }` — from inside
`pub fn stops(tol)`, whose only caller is the render walk in
`demos/tour/src/main.rs`. The file has no `#[cfg(test)]` module at all,
so `cd demos/tour && cargo test --release` — the command CI's
*"demos tour suite (the #99 ε pin + the tour's own probes)"* row runs —
never attempts it. The probe's whole point is fail-loud: `walls::wall`
panics both when the refusal CHANGES and when it goes away. Neither can
be observed by the suite that names it.

Found by D403's sweep. The pattern was `crate::walls::wall(` over
`demos/tour/src/*.rs`: four scenes call it — `lily` and `klein` through
their own `wall_probes`, both driven by in-bin tests since `#1434`;
`teapot`, closed by D403; and this one, the only remaining hit.

## Fix

Cheaper than D403's was. The probe's operands (`quarter`, `sectioned`)
are built immediately above it and are read by nothing after it — the
stop list is assembled from `sealed` and `cup_after` — so unlike the
teapot's pair this wall IS separable from `stops`. Lift it into a
`wall_probes(tol)` of the shape `lily::wall_probes` and
`klein::wall_probes` carry (it can build its own quarter revolve), call
it from `stops`, and add the in-bin `#[cfg(test)]` test that drives it.
It must be in-bin: `demo-tour` is bin-only, so nothing under `tests/`
can name the scene's items.

## Ground

`demos/tour/` is in no open program's `paths` — Track X left the
tracker on 2026-09-11 and SUITE carries D403, this row's sibling, so the
finding is filed beside it. A lane taking it announces to SHELL, which
holds the tour's scenes by courtesy.

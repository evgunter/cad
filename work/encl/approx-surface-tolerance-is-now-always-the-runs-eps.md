---
id: approx-surface-tolerance-is-now-always-the-runs-eps
kind: issue
title: SurfaceSpec.tolerance is now always the run's eps on every production path; map_approx reads it as if it were the surface's own
status: open
opened: 2026-09-05
---

**Owner: whoever owns `crates/geom/src/surfaces/approx.rs` and
`crates/topo/src/transform.rs` — `geom` / TRANSFORM.** Filed here rather
than on a program's slate because the two files sit under different
owners and neither is SEAT's; SEAT-9 (PR 1995) is the change that made
the field redundant and disclosed it.

## What moved

Before SEAT-9 the shell's offset chain let a caller choose the fit
target: `topo::shell` took a `tolerance: f64` beside `tol: Tol` and
threaded it to the mint. It does not any more — every kernel door on
that chain takes the `Tol` witness and the value is read once, in
`geom_brep::offset_fit::precision_target`. The one production mint of an
`ApproxSurface` is `topo::props`'s
`PropsQuadLane::approx_offset_surface` (`crates/topo/src/props.rs`), so
**every `ApproxSurface` a run mints now stores `tolerance == tol.eps()`**
— the field is a copy of the ambient global rather than a per-surface
fact.

## Why it was kept, and what the question is

`geom::SurfaceSpec::tolerance` / `ApproxSurface::tolerance()`
(`crates/geom/src/surfaces/approx.rs:198`, the accessor below it) has one
production reader: `topo::transform.rs`'s `map_approx` re-derives a
mapped face's certificate against `old.tolerance`, and the paragraph
above that function argues why — a rigid map genuinely moves `hull_sup`
(measured at 7.4e-9 on a bowed base fitted at 1e-6), so what the map
preserves is the CLAIM, "the mapped pair certifies at the same
tolerance", and reading the number off the surface is what makes that
claim self-describing. That argument was written when the mint's target
was the caller's. It now reads a number that is always ε.

**The question, for the owner**: does the field retire, with `map_approx`
classifying at the run's ε like tier 3 already does — or does it stay,
on the ground that a stored claim should not become ambient even when
its value is? Either answer is defensible; what is not is leaving a
reader to infer a per-surface bound from a field that is a global.

The cost of retiring it, measured while deciding not to do it in
SEAT-9: `geom::SurfaceSpec` loses a field, `ApproxSurface::certify`'s
certifier closure loses its fourth argument,
`geom_brep::PcurveFittedLane::remap_certificate` loses its `tolerance`
parameter (`crates/geom-brep/src/pcurve_cache.rs`), `map_approx` needs a
`Tol` it does not currently hold, and roughly a dozen test sites that
read `.tolerance()` move. It also retires the one production caller of
`geom-brep`'s numeric-target `_at` routines — the exception
`crates/topo/tests/shell_tolerance_chain.rs`'s
`only_the_transform_lane_reaches_the_numeric_target_routines` names — so
that census would tighten to "no production caller at all".

## Reproduction

Nothing is broken, so there is no failing row to cite. The claim is a
reading: `crates/topo/src/props.rs`'s `PropsQuadLane for f64` is the
only production `approx_offset_surface` caller in the tree, it now
passes the witness, and `geom_brep::approx_offset_surface` stores
`precision_target(tol)` into `SurfaceSpec::tolerance`
(`crates/geom-brep/src/offset_fit.rs`, the storage door).

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/props/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `crates/geom/src/surfaces/approx.rs` is PROPS' glob and the field's owner decides; `topo/src/transform.rs`'s `map_approx` is SHELL's and is edited by announced seam.

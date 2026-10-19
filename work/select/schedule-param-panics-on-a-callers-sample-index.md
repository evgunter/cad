---
id: schedule-param-panics-on-a-callers-sample-index
kind: issue
title: geom_brep::schedule_param is pub and asserts samples >= 2 and i < samples on its caller's arguments, which D2 makes input
status: open
opened: 2026-10-19
priority: P3
cost: E
---



Found by the blind-spot pass of ENCL's total-render sweep
(`work/encl/a-hand-built-unminted-refusal-panics-through-pub-render.md`):
the same shape as a panicking `pub` render, outside the render class.

`geom_brep::schedule_param` (`crates/geom-brep/src/certify.rs`,
re-exported from `crates/geom-brep/src/lib.rs`) is `pub` and
`assert!`s `samples >= 2 && i < samples` (its `# Panics`: "every
caller passes a fixed count"). `[failure]` (D2) makes a `pub` call's
arguments input, so `geom_brep::schedule_param(0.0, 1.0, 5, 3)`
panics on input.

Repair: a typed refusal or a type that cannot hold the bad index (a
sample index minted by its schedule), or `pub(crate)` if no consumer
outside the crate needs it; keep the assertion only where the count is
the kernel's own.

## More doors (ENCL review of PR 5393)

`geom_brep::sample_param(t0, t1, i)` (`crates/geom-brep/src/certify.rs`, `sample_param`, ~2561, re-exported) and `EdgeCurve::sample_param(&self, i)` (~2354) are also `pub`. Both take the caller's `i` and reach the same `assert!` when `i ≥ CERT_SAMPLES`. `topo` calls `sample_param`, so making `schedule_param` `pub(crate)` does not cover them. The repair has to make the index typed (a sample index constructible only below `CERT_SAMPLES`) or the call total.

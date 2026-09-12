---
id: gate-then-measure-class-remains-in-scene-modules
kind: issue
title: the gate-then-measure pair is still spelled in the tour's scene modules and an stl example after the side unit
status: open
opened: 2026-09-12
---


## The finding

Reported by the `gate-then-measure` side unit (PR 2440), outside its
fence: after `demos/tour/src/main.rs`'s `run_body` gates then measures
with one quadrature, the same pair is still spelled at
`demos/tour/src/torusvessel.rs:400/404, :428/478, :516/539`,
`skinned.rs:893/906`, `diefillet.rs:603/605`,
`impeller.rs:333/355/373`, and in `klein.rs`, `lily.rs`, `teapot.rs`;
and at `crates/stl/examples/export_acceptance.rs:26,28`. The tour's
~115 `mass_properties` calls outside `run_body` are these. Each site
is its own byte-identity question (its printed line), and the same fix
applies: take the certificate from the gate and continue it.

`demos/tour/src/probe.rs:69,72` (`validate_probe`) has the shape but
stays as it is: it is the tour's telemetry instrument at the `Probe`
scalar, and merging its doors would stop it sampling the reporting
walk's predicates that `tools/k-lint` reads (orchestrator ruling,
2026-09-12).

## What a fix is

One sweep unit: the tour's scene modules and the stl example take the
one-quadrature spelling, stdout diffed byte-identical at three ε per
site; `validate_probe` excluded by name. Demo code, no kernel change.

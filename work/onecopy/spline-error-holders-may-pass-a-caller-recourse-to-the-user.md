---
id: spline-error-holders-may-pass-a-caller-recourse-to-the-user
kind: issue
title: Twelve SplineError holders may render its caller-facing recourse where the kernel built the spline
status: open
opened: 2026-11-09
---


Filed by the ONECOPY `onecopy/unbuilt-recourse` lane (PR 6622). It is the
second pass of the knot-API sweep the orchestrator asked for. The first pass, over
`KnotAlgebraError` holders, filed
`work/carverest/skin-renders-a-knot-api-recourse-to-the-loft-user.md` and
`compose-elevation-refusal-renders-a-knot-api-recourse.md`.

## What

`geom_core::spline::SplineError`'s `Display` (`crates/geom-core/src/spline/knots.rs`)
states about 24 `Recourse:` endings, each addressed to whoever built the
spline. A holder whose spline the user supplied passes the right
recourse. A holder whose spline the kernel built passes a recourse the
user cannot act on. The holders are below, each still unclassified:

- `step-import/src/adopt.rs` `ChartRow(SplineError)`
- `sweep/src/loft.rs` (`source: SplineError`), `sweep/src/skin.rs` `Structure`
- `geom-core/src/spline/algebra.rs` `KnotAlgebraError::Structure`, `geom-core/src/spline/compose.rs` `Structure`
- `geom-brep/src/intersect/conic_arc.rs` `Carrier`, `geom-brep/src/offset_fit.rs` `Structure`,
  `geom-brep/src/nurbs_iso.rs` (`source`), `geom-brep/src/pcurve_cache.rs` (`source`)
- `geom/src/curves/compose.rs` `Knots` and `Structure`, `geom/src/curves/fit.rs` `Structure`

The pattern was `(SplineError)`, `: SplineError,` and `source: SplineError` outside
`tests/`. It cannot match a holder that stringifies the error or reaches
it through a wrapper enum.

## What closing it takes

For each holder, say who built the spline. Where the kernel built it, render
the holder's own ending instead of the payload's.

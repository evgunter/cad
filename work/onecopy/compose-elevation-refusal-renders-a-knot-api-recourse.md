---
id: compose-elevation-refusal-renders-a-knot-api-recourse
kind: issue
title: ComposeError::ElevationRefused renders KnotAlgebraError's caller-facing recourse for an elevation the kernel chose
status: open
opened: 2026-11-09
---


Filed by the ONECOPY `onecopy/unbuilt-recourse` lane (PR 6622) in the
sweep the orchestrator asked for after `topo::SectionUnbuilt::Seat` was
fixed. Seat had passed a knot-API recourse through to the user.

## What

`geom::curves::compose::ComposeError::ElevationRefused` writes "leg {leg}
refused degree elevation: {source}", with `source` a `KnotAlgebraError`.
Compose elevates each leg to the chain's common degree, so the kernel
chooses that elevation. `KnotAlgebraError`'s `Display` ends on a recourse
addressed to the knot API's caller ("Recourse: ask for …, or raise the
degree"), which here is compose itself.

`ComposeError`'s other arms state no recourse at all, so the right
ending may be decided for the enum as a whole.

## What closing it takes

Give `ElevationRefused` its own ending, decided on the payload, and stop
rendering the knot-API sentence through it.

`geom_core::spline::KnotAlgebraError::fact` renders the refusal without
its caller-facing recourse (PR 6622 added it for `SectionUnbuilt::Seat`),
so a holder can keep the diagnostics and end on its own recourse.

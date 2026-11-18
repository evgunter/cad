---
id: composite-form-takes-enclosure-valued-coefficients
kind: issue
title: The ring composite's implicit form takes enclosure-valued coefficients, so no operand is read at a midpoint
status: open
opened: 2026-11-17
priority: P3
cost: H
refs: [carrier-frames-the-types-and-the-lift]
---


`geom_brep::ssi::certify::composite_form` reads each analytic operand's
structural parameters as `f64` structure, through `exact`'s `lo == hi`
gate. Since `carrier-frames-the-types-and-the-lift`, a frame re-minted
at an enclosure scalar is read at its midpoint when it is rounding-wide
(`frame_direction`, under `FRAME_ROUNDING`). The turn its width allows is
charged over the carrier's control box (`FrameCharge::over`). Any
operand widened past that still refuses `UnsupportedCertificate` as "a
family of surfaces".

The doc comment above `exact` names the principled fix: enclosure-valued
coefficients in `geom_core`'s `ImplicitSurface` composite. With them no
operand is read at a midpoint, and a widened operand certifies as the
family it is. That is a `geom_core` change of its own (orchestrator ruling
(c), 2026-11-17).

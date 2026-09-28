---
id: adoption-certification-has-no-eps-in-lever
kind: issue
title: step-import: a band-decided certification refusal at adoption names no tolerance, because the ladder certifies at the kernel's ε and not at the file's ε_in
status: open
opened: 2026-09-28
---


(ENCL implementer, from the fix pass on PR 3351.)

## What

D4 ¶1 (i): "At adoption the lever is the import's own ε_in (D7),
phrased by the import door." PR 3351 gives STEP adoption its own
reading of a certification refusal
(`geom_brep::certify::Reading::Adopt`, rendered by
`StepImportError::Adoption` and `::Assembly` in
`crates/step-import/src/error.rs`). A band-decided arm of a sized
decision (span, winding, transversality, second order) — in band, at
Zero where Zero does not pass, or straddling — names its geometry
lever alone; a sign-certain arm, and every residual, ends in the
kernel-or-file defect ending.

It names no ε_in lever because none would be true. The ladder
certifies each candidate through `Body::set_edge_curve_nurbs_lane(…,
tol)` (`crates/step-import/src/adopt.rs`, `adopt_edges`) at the
kernel's ambient `Tol`. ε_in is spent elsewhere: surface recognition
(`recognize.rs`), curve recognition (`recognize_curve.rs`), rim and
wall normalization (`normalize.rs`), the imported chart
(`chart.rs`), and vertex resolution and the placement check
(`entities.rs`) — never by an edge certification. So no ε_in value
decides these margins, and the import door has no phrasing of ε_in as
a lever to reuse: its only ε_in sentences are the refusals that name
it as the budget ("recognition at ε_in is ambiguous", "the file's own
declared uncertainty", `MissingUncertainty`).

A second import door reads certification differently.
`StepImportError::TierInvalid` renders `topo::validate`'s at-rest
text for the adopted body, so an at-rest certification refusal there
can still name "tighten the tolerance below …" — the kernel's ε, at
an import door. Which reading that door should take is part of the
same question.

## Repair shape

Either the band-decided certification arms at adoption stay
lever-only (and D4 ¶1's ε_in sentence is read as covering the
recognition refusals only), or adoption certifies against a band
that ε_in widens, and then those arms name "a smaller declared
uncertainty" with the value; and `TierInvalid` either keeps the
at-rest reading or takes the adoption one. A design fork on D4 ¶1 /
D7.

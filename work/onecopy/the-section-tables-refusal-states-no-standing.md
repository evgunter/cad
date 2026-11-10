---
id: the-section-tables-refusal-states-no-standing
kind: issue
title: geom_brep::SectionError states no standing, so the join's Section cause stands as an abort
status: closed
closed: 2026-11-10
branch: onecopy/section-error-standing
opened: 2026-11-10
---


Found folding the review of `onecopy/touch-unread-names` (PR 6726). `BooleanError::ApexSection` and `BooleanError::Join` are read by the Boolean's vertex-on-face climb (`boolean/mod.rs`, `Rung::refused(e, BooleanError::refusal, …)`), and both now stand by cause (`SplitJoinError::refusal`, `ApexSectionReading::refusal`).

## What

`SplitJoinError::Section { source: geom_brep::SectionError }` stands as an abort. It does so as a stand-in, because `geom_brep::SectionError` (`crates/geom-brep/src/intersect.rs`, ≈15 variants) states no standing of its own, and most of its variants' docs state no class:
- `WrongLane`, `NoRegion`: a caller bug.
- `Escalated`, `RadiusEscalated`: an escalation.
- `RoutesToGeneralRung`, `NoMarch`, `BeyondOperandExtent`: not built.
- `DegenerateOperand`, `DegenerateTorus`: invalid input, the latter with `TorusRefusal`'s own standing.
- `Carrier`, `Spiric`: a constructor's verdict.
- `Pinched`, `Unordered`: decided readings.
- `Unaccounted`: mixed.

A ladder reads it through the apex section's table refusal (`ApexSectionReading::Refused`, from `chord_join::wall_section` on a cone, `plane_cone_section`). There, an abort stops the vertex-on-face climb, where a later rung might answer.

## What closing it takes

A per-variant `SectionError::refusal()` in geom-brep, each variant's class stated in its doc. Then `SplitJoinError::Section` delegates to it. Measure the vertex-on-face climb's outcomes before and after.

## Closed: each section refusal stands by its cause (#onecopy/section-error-standing)

`geom_brep::SectionError::refusal` (`crates/geom-brep/src/intersect.rs`) is exhaustive, and `SplitJoinError::Section` delegates to it. The standings are:

- **By margin:** escalations, and the carrier and spiric constructors' escalations.
- **In band:** `CoincidentSurfaces`. Also `Unordered`, now decided `Zero` only, because the torus arms carry an undecided side to `Escalated`. Also `ConeAtTheBand`, the cone's aperture and seam guards decided `Zero`, split from `DegenerateOperand`.
- **Limits:** an arm not built, a station beyond the extent, an ellipse past what the representation carries.
- **Aborts:** a caller's defect, a constructor disagreeing with its arm, and a mixed cause.

Two standings moved on the ladder's path, which reaches `SectionError` only through the plane-cone apex section:
- `Carrier(Escalated)` moved from abort to its margin.
- A cone's guard decided `Zero` moved from abort to in band.

A probe at default ε and at `1e-12` found no ladder reading a section refusal in-suite, so no reached outcome moved. Both standings are pinned through the apex section.

Two mixes remain, each with its own row:
- `Pinched`: `work/onecopy/section-pinched-mixes-decided-tangencies-with-undecided-ones.md`.
- `DegenerateOperand`'s radius, poison and unread partner: `work/onecopy/section-degenerate-operand-mixes-a-radius-at-the-band-with-invalid-operands.md`.

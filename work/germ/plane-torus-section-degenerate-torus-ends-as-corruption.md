---
id: plane-torus-section-degenerate-torus-ends-as-corruption
kind: issue
title: germ: the plane×torus section ends a definite non-ring torus as 'corrupt' and its in-band sibling as an escalation
status: open
opened: 2026-09-30
---


(TOPO, the §5 second pass of PR 3506: the other doors that read the
ring-torus convention, checked for D4 ¶1 (iv).)

## What

`geom_brep::intersect`'s plane×torus section (`crates/geom-brep/src/intersect.rs`,
the ring-convention guard near :1985) decides the convention's two
halves (`pt_tube_guard`, then `geom::ring_torus`):

- in band, it returns `SectionError::Escalated`, whose `Display` renders
  the whole `Indeterminate` ("the two surfaces' configuration is
  ill-conditioned at this tolerance: {diag}"), so it ends in
  `COINCIDENCE_RECOURSE`;
- decided `Zero | Negative`, it returns `SectionError::DegenerateTorus`,
  whose `Display` ends "a validated body has a ring torus, so this one
  is corrupt": a defect ending, no lever.

PR 3506 gave the convention one story at the pierce
(`geom_brep::TorusConvention`, `crates/geom-brep/src/torus_convention.rs`,
read by `BooleanError::DegenerateTorus`):
both arms name the half's lever ("make the tube radius clearly smaller
than the ring radius"), the band-decided arms with the tolerance the
margin gives; tier 3's `ValidationError::DegenerateTorus` reads the same
`SizedDecision`. The section's two arms tell two other stories, and
neither is the pierce's.

## Repair shape

Carry which half refused and its verdict (`geom::torus_tube`, which
keeps its reporting margin, in place of `pt_tube_guard`), as a
`geom_brep::TorusConvention` in this crate, and end both arms from its
`sized()` and `refused()` at the door that reads them (a build, or at rest through `join`'s frames and
`replace_face`), or say why the section reads only validated bodies and
end both arms as the defect.

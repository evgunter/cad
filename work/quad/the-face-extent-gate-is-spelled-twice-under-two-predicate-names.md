---
id: the-face-extent-gate-is-spelled-twice-under-two-predicate-names
kind: issue
title: props: the face-extent gate is spelled twice, curved::require_extent (props_face_extent) and quad::run::face_extent (props_quad_face_extent)
status: open
opened: 2026-10-27
priority: P4
---


Found in the STYLE review of PR 5815 (C3). It is left as is there,
because it predates that PR and spans two programs' ground.

The same gate, "a definitely positive extent, else `DegenerateFace`",
is spelled in two places:

- `crates/geom-brep/src/props/curved.rs`, `require_extent`. It decides
  `props_face_extent` through `curved`'s `classify`, on the margin its
  closed-form callers build. FLUX/FLUXTAIL ground.
- `crates/geom-brep/src/props/quad/run.rs`, `face_extent`. It decides
  `props_quad_face_extent` through `classify_len`, on
  `Margin::over_lever(lo_or_refuse(area), perimeter)`. Since C3 this
  is the one site every quadrature lane (and spiric) asks.

The two differ in predicate name, funnel wrapper and margin shape.
Whether they should be one door (one name, or two names through one
helper) is the question. Merging the names would move k-lint's
per-predicate baselines (`docs/K-REPORT.md`), so it is a re-baselining
decision, not a refactor. Related:
`work/twice/geom-brep-has-more-than-one-decide-wrapper.md` (the
funnels).

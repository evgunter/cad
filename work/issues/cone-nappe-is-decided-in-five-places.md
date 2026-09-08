---
id: cone-nappe-is-decided-in-five-places
kind: issue
title: which nappe of a cone a face or point lies on is decided by five predicates in four files — one fact, five spellings, none reconciled
status: open
opened: 2026-09-08
---


Found by both reviewers of SHELL-6 (PR #2178, 2026-09-08; R1 MINOR-2,
R2's Q1/Q4 class finding), placed here by the SHELL orchestrator
because the sites span three programs' territory.

SHELL-6 gave the OFFSET lane one home for the fact:
`topo::offset_nappe::face_nappe` (predicate `offset_nappe`), read by
both offset doors, the apex-window gate and
`geom_brep::ConeOffset::displacement`. The same fact — which nappe of
the double cone a face's (or a point's) material lies on — is decided
independently, by name, in:

- `crates/geom-brep/src/pcurve_cache.rs` (`pcurve_cone_chart_nappe`,
  two sites, per POINT from its own axial station — the `copysign`
  shape SHELL-6 deleted from `displacement`) — TRIM's file;
- `crates/geom-brep/src/props/curved.rs` (`props_cone_nappe`, a
  window straddle refusing `NappeSpanning` — the twin of
  `ReplaceFaceError::NappeStraddles`, decided from a window instead
  of the corners) — PROPS' file;
- `crates/topo/src/boolean/solid_contain.rs` (`bool_cone_trim_nappe`,
  `bool_cone_trim_side`, `bool_ray_cone_nappe`, from a face's slant
  window) — S-BOOL's / CURVED's file;
- and inside SHELL's own fence `offset_axial_side` (per corner) beside
  `face_nappe` (per face), which SHELL-6's fix pass reconciles.

Five spellings of one fact can disagree with each other exactly where
it matters — a face whose corners sit on both nappes, a point near the
apex — and no test pins that any two agree. What closing this needs
is one vocabulary (`geom_brep::Nappe` exists now) and one decision
site per GRANULARITY (face, point), with the three programs' readers
calling it; the per-point reads may be right to stay per-point, in
which case the row that pins them against the face's answer is the
deliverable. Owner undecided between TRIM, PROPS and S-BOOL, which is
why this sits in `work/issues/`. Signed (SHELL orchestrator).

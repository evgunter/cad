---
id: pcurve-mint-escalation-carries-no-decision
kind: issue
title: topo::pcurves: PcurveMintError::Escalated carries no closed decision, so its at-rest ending cannot name a lever or a tolerance
status: open
opened: 2026-09-29
---


(ENCL implementer, `work/encl/validate-own-close-levers-follow-the-d4-recourse-ruling.md`.)
The rule is D4 ¶1 (i) in `docs/DESIGN.md`: the recourse follows from the
decision and its verdict, and the decision is a closed type at its site.

## What

`PcurveMintError::Escalated { half_edge, cause }` (`crates/topo/src/pcurves.rs`)
is raised by several decisions (`pcurve_iso_side`,
`pcurve_iso_arc_direction`, `pcurve_loop_continuity`, and the others in
`pcurves.rs` that map an `Indeterminate` onto it) and keeps only the
`Indeterminate`, whose predicate name is the one trace of which it was.
`topo::validate::classify_pcurve` therefore cannot tell a sized decision
(a valued conditional tighten on a positive in-band margin) from a
residual (`pcurve_loop_continuity`: the last resort or the defect), and
ends it in the bare lever "Recourse: move the geometry", or the defect
ending on a poisoned margin (`own_close`).

The certifier's own escalation already carries its decision
(`PcurveCertifyError::Escalated { check: PcurveCheck, .. }`), and ends
through `PcurveCertifyError::ending` / `PcurveCheck::recourse`
(`crates/geom-brep/src/pcurve_cache.rs`).

## Repair shape

Give `Escalated` a closed check type, one variant per decision the mint
takes, with its ending table beside it (`geom_brep::recourse::SizedDecision`
for a sized one, `geom_brep::recourse::Unsized` for a residual or a form
selection), and have `classify_pcurve` read it at `Reading::AtRest`.

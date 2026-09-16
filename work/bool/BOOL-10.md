---
id: BOOL-10
kind: unit
title: the arc_continue retirement and the declared-joints arc form
status: closed
closed: 2026-09-16
opened: 2026-09-01
refs: [BOOL-9]
branch: bool/10-arc-continue-retirement
pr: 2135
---

Q1 second-round extension: remove the `arc_continue` verb, its program step,
eval arm, profile lift and refusal family; re-spell authored arc subdivision
as declared tangent joints on one circle (the sixth-round ruling's name for
it); narrow the sealed verb-module signatures to bare state values so
chain-state-consuming verbs are unwritable. With BOOL-13 landed there is no
schema bump — the wire vocabulary changes and the corpus regenerates.
Difficulty L. Spec: `docs/BOOL-10-SPEC.md`.

Runs beside BOOL-9's raw-door survey, after BOOL-12. From
`work/bool/log.md`, the BOOL-11 and BOOL-13 entries' slate lines.

## Closed

PR 2135 merged 2026-09-16 at the stripped landing head `af006c649` (run
35048280142 green). Landed: `arc_continue` removed (tag 28 retired), the
seal made real as an inventory plus a `DirectedPoint` `compile_fail`,
the lift re-target, the equator authored through
`.tangent().tangent_arc_to(p)`, no wire-shape change. The declared-split
arc form `arc_to(spec.split(n))` was built, dual-reviewed at ordinal
1108 and fix-passed, then DROPPED by Ev's ruling (in-chat, 2026-09-13):
the sixth round already admits adjacent same-carrier arcs as declared
tangent joints, so the form bought convenience and 1–2 ulp at the cost
of a format break and three types. **It is preserved in history at
`f79fa7081`** (review head `3f8163dd8`, fix-pass head `231b4db83`) in
case it is wanted later. A/B row BOOL10, sample #212. Residue filed:
`sweep-arclen-legs-fold-an-over-full-angle` (this slate);
`carriers-are-identical-reads-carrier-identity-at-the-lattice` and
`lift-comparator-misclasses-a-declared-joint-difference` (already open
here); `north-star-audit-verb-list-names-arc-continue` (LIB);
`work/m10/symbolic-tier-census.md:101/102/159` name the retired verb
(M10's — reported on the PR, not edited).

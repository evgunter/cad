---
id: a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band
kind: issue
title: A glued pair is re-read at its consuming site through C4's declared rung, which bridges an in-band displacement D10 says refuses
status: open
opened: 2026-10-29
priority: P0
cost: M
design: true
refs: [d10-exit-walk]
---


Found by the D10 exit walk (`work/intent/d10-exit-walk-report.md`, clause 1).

## What

Stage 4 F (#4641) removed every public declaration of a face pair, and
the glue door (#4496) decides each pair from its margins. The glue
table is then read at the consuming sites through the ladder rung that
was C4's declaration: `vtxfac.rs:978` and `recl.rs:140` build
`PlaneIdentity { declared: declared_one_carrier }` from the glue table
(`declared.class_of`), and `merge_faces.rs:2669` passes
`PlaneIdentity::DECLARED` for its `declared` argument.
`PlaneIdentity::declared` (`plane_eq.rs:116`) routes
`carrier_eq_reading` (`carrier_eq.rs:365`) and `plane_ladder`
(`plane_eq.rs:202`) to `carrier_eq::declared_reading`
(`carrier_eq.rs:700`). That reading bridges a displacement whose upper
bound is in band at every consumed point (`ContactVerdict::Bridged`,
`carrier_eq.rs:790`). The undeclared path refuses the same in-band
offset (`PlaneEqError::Undecided`, `plane_eq.rs:279`).

DESIGN `[coincidence]`: "a margin decided Zero glues them, a definite
one keeps them apart, and one in the sliver band between refuses." The
topo README's C4 is the clause that says a declaration bridges the
residue, and DESIGN's state-of-the-build list retires it.

## The question

Is the second read a decided coincidence being read again, which has
to agree with the door's decision? Or is it a fresh decision over a
different extent, which D10 says refuses in band? If it is the first,
the rung's names (`declared`, `DECLARED`, `declared_reading`, "F5") and
its docs (`plane_eq.rs:9-17`) are the only residue. If it is the
second, `Bridged` admits an in-band sliver at that site, and
`reachhold/a-settled-declared-coincidence-crosses-a-tight-volume-bound`
is one of its consequences.

Related: `the-at-rest-face-face-confirm-bridges-an-in-band-relative-curvature`
(the at-rest half), `boolean-refusals-for-door-glued-pairs-keep-declaration-names`
(the refusal names).

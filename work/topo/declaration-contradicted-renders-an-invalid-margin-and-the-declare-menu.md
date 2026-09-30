---
id: declaration-contradicted-renders-an-invalid-margin-and-the-declare-menu
kind: issue
title: topo: a contradicted declaration renders its definite verdict as an invalid margin followed by the declare menu, beside its own recourse
status: review
opened: 2026-09-29
priority: P2
cost: E
pr: 3493
branch: topo/route-refusal-subjects
---


(CHROME, disclosed in the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`
and filed here so the disclosure has a file. `work.py territory` gives
`crates/topo/src/merge_faces.rs` to TOPO and ZIP. It names no owner for
`crates/topo/src/boolean/mod.rs`, so this row takes both. The
contradicting verdicts themselves are minted in
`crates/topo/src/boolean/carrier_eq.rs`, which is TANG's ground, and
the repair's routing table reads that file's predicate names.)

## What

Two arms render the whole `Indeterminate` inside a parenthesis, then a
recourse of their own:

- `BooleanError::DeclarationContradicted`
  (`crates/topo/src/boolean/mod.rs:1750`, the variant at :871): "a
  declared coincidence contradicts the geometry ({diag}) — the declared
  pair's planes are definitely distinct; fix the declaration or the
  geometry, the op never glues a lie". It is raised at `reduce.rs:641`,
  `vtxfac.rs:331` and `recl.rs:148`. The `recl.rs` site takes a
  **non-planar** pair through `carrier_eq`. So the fixed words "the
  declared pair's planes are definitely distinct" (`boolean/mod.rs:1752`)
  are false there: the pair may be two cylinders, two spheres or two
  tori.
- `MergeCoplanarError::DeclarationContradicted`
  (`crates/topo/src/merge_faces.rs:560`, the variant at :417, raised at
  :1868): "merge_coplanar_faces: declared coincidence contradicts the
  geometry ({diag}) — fix the declaration or the geometry, …". The
  function-name label is the shape guard's `stage_prefixes` problem as
  well.

The `diag` is not an escalation. Every site builds it for a DEFINITE
verdict with `margin: MarginDiag::INVALID`, at four raise sites:

- `crates/topo/src/boolean/plane_eq.rs:304` (`bool_plane_parallel`);
- `plane_eq.rs:344` (`bool_plane_offset`);
- `crates/topo/src/boolean/carrier_eq.rs:336` (`carrier_kind`);
- `carrier_eq.rs:397–404`, in `data_rungs`: the cylinder, sphere and
  torus parameter predicates (`carrier_cyl_axis_parallel`,
  `carrier_cyl_axis_offset`, `carrier_cyl_radius`,
  `carrier_sphere_center`, `carrier_sphere_radius`,
  `carrier_torus_axis_parallel`, `carrier_torus_center`,
  `carrier_torus_major_radius`, `carrier_torus_minor_radius`). These
  reach the Boolean through `recl.rs:139–148`. So `{diag}` renders "margin is invalid (NaN or a
poisoned enclosure) against the ambiguity band (…) — check the
operation's inputs upstream, then declare the coincidence, move the
geometry, or lower the tolerance". That text is wrong three ways:

- the margin was decided, not poisoned;
- "check the inputs upstream" points at a defect that is not there;
- "declare the coincidence" advises adding the very declaration the
  geometry has just contradicted.

## What the shape guard counts

`test_utils::refusal::recourse_markers` would count **one**: the bare
`COINCIDENCE_RECOURSE` inside the parenthesis (one of
`BARE_RECOURSES`, in a sentence no `Recourse:` opens). It does not count
the arm's own "fix the declaration or the geometry", because that has no
`Recourse:` label. So the guard would pass a message whose one counted
recourse is the wrong one, and whose right one it cannot see.
`subjectless_escalations` would pass it too: the clause before the
parenthesis is a sentence. Neither arm is rendered by the shape guard's
samples today.

## Repair shape

**PR 3493 does not route by `diag.predicate`**, as the paragraph below
prescribes. D4 ¶1 (i), ratified in PR 3352, makes the decision a
closed type at its site, so its sentence is an exhaustive match, never
a lookup by predicate name. The rung that finds the pair distinct sets
a `boolean::Contradiction` (`plane_eq`'s declared rung, `carrier_eq`'s
kind arm and `data_rungs`), carried in `CarrierEqError::Contradicted`
and in both `DeclarationContradicted` arms in place of the
`Indeterminate`; `contact_verify::fit_steer` reads the same enum. It
ends in `contact::CONTRADICTION_RECOURSE`, the one recourse every
contradicted declaration states, rather than the label below.

Drop `{diag}` from both sentences, and do not substitute `payload()`:
for this `INVALID` margin it says "margin is invalid (NaN or a poisoned
enclosure)", which is equally false. Say which fact contradicted the
declaration, in words routed by `diag.predicate`. That takes one clause
per predicate above, not three: "the declared planes are not parallel"
(`bool_plane_parallel`), "…are parallel but apart"
(`bool_plane_offset`), "the declared faces are different kinds of
surface" (`carrier_kind`), "the cylinders' axes are not parallel",
"…their radii differ", "the spheres' centres differ", and so on for the
torus rungs. Replace the fixed "planes" clause at `boolean/mod.rs:1752`
with the routed one. `contact_verify.rs` :125–134 (`fit_steer`) already
names several of these predicates and tells an angular contradiction
from a separation; the table can start there. Then label the existing lever:
`Recourse: fix the declaration or move the geometry`. That gives
exactly one marker. If the measured margin is worth showing, the raise
sites have to carry it (`plane_eq` decides the sign, then throws the
margin away); `INVALID` cannot stand in for it.

---
id: escalations-forwarded-whole-are-untriaged-for-a-declarations-object
kind: issue
title: chrome: ~60 error variants forward Indeterminate whole, and which of them offer a declaration their door has no object for is untriaged
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of PR 3347; the review asked that
this residue be filed rather than left in the PR body.)

## What

`geom_core::Indeterminate`'s own `Display` ends in
`COINCIDENCE_RECOURSE`, unlabelled: "declare the coincidence, move the
geometry, or lower the tolerance". A field-type grep found about 60
error-enum fields that carry an `Indeterminate`, and many of them
render it whole:

    grep -rnE '^\s+(source|cause|diag|inner|escalation): (geom_core::)?(predicate::)?Indeterminate,?$' crates --include=*.rs

The hits, at 7ab8ead, are spread across `sweep` (revolve 7, extrude 4,
loft, tube, blend), `topo` (validate 6, splitting 5, contact 3,
boolean 5, merge_faces 2, chord_join 2 and others), `editor-core`
(eval 4, names 3), `geom-brep` (pcurve_cache 2, certify, props, offset,
nurbs_iso, newell, offset_meters), `profile` (validate, path) and
`geom` (curves/compose).

Each one that forwards the whole `Display` hands the viewer two
problems:

- an unlabelled repair, which `test_utils::refusal::recourse_markers`
  counts as zero (see item 8 of
  `the-refusal-shape-guard-has-blind-spots`);
- a "declare" lever, which is advice only where the door takes a
  declaration.

Nobody has checked the sites one by one.

## Already done or filed

- The routed blend arms render the payload view with their own
  recourse.
- `PathError`'s non-junction arms render the payload view with their
  own recourse.
- `MeterError::Escalated` does the same (PR 3347).
- Four geom-brep carriers that `topo::validate` routes through
  `own_close` are filed on their owners' slates:
  - `work/encl/certify-escalation-renders-the-coincidence-menu-unlabelled.md`
  - `work/iso/plane-nurbs-escalation-renders-the-coincidence-menu-unlabelled.md`
  - `work/pcert/pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled.md`
  - `work/props/props-escalation-renders-the-coincidence-menu-unlabelled.md`
- Extrude and revolve offer the menu on purpose, even on their definite
  arms, so for those two the question is their owners'.

## The triage

For each remaining hit, say which door raises it and whether that door
takes a declaration. Where it does not, the site wants the
`payload()` + routed `Recourse:` shape, and a row filed on the owner's
slate.

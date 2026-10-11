---
id: whole-escalation-renders-outside-the-too-close-family
kind: issue
title: geom-brep IsoFamilyRefusal::Undecided and sweep's sliver Lever arm render an Indeterminate whole outside the undecided shape
status: open
opened: 2026-10-11
priority: P3
cost: M
---


(Found by the `indeterminate-ending-drops-the-per-kind-levers-under-keeps`
sweep for whole-escalation renders in geom-core, geom-brep and sweep.)

## What

Two Displays render an `Indeterminate` whole through its bare `Display`
(`under(COINCIDENCE_RECOURSE)`), so they end in the coincidence menu
with `IndeterminateUnder`'s ` — {advice}; {ending}` joint, and neither
is on the `too-close-to-call-remainder` list:

- `geom_brep::IsoFamilyRefusal::Undecided` (`pcurve_cache.rs`, its
  `Display`): "whether the edge runs along a cylinder's ruling or around
  it is undecided: {diag}". Its one consumer, `topo::JoinRefusal`
  (`boolean/edge_join.rs`), renders the escalation itself under
  `JOIN_LEVER`, so this nested type's ending is its caller's: by the
  parent row's rule it is a payload-only Display, through
  `geom_core::undecided!`.
- `sweep::swept::sliver_text`'s `DihedralReading::Lever` arm: "{what} is
  neither a definite corner nor definitely smooth: {source}". Its
  `Bend` arm already takes the undecided shape; the `Lever` arm wants a
  subject in plain words and a recourse picked from the verbs that reach
  it (extrude, revolve), none of which takes a declaration.

The other whole renders the sweep met are owned: `intersect.rs`'s
`SectionError::Escalated` ("ill-conditioned at this tolerance") by
`work/germ/plane-torus-section-degenerate-torus-ends-as-corruption.md`,
`PcurveCertifyError::Escalated` by
`work/pctail/pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled.md`,
and sweep's "too close to call" sites by `too-close-to-call-remainder`.

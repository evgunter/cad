---
id: whole-escalation-renders-outside-the-too-close-family
kind: issue
title: geom-brep IsoFamilyRefusal::Undecided renders an Indeterminate whole outside the undecided shape
status: closed
opened: 2026-10-11
priority: P3
cost: E
branch: encl/iso-family-undecided
pr: 4756
closed: 2026-10-12
---


(Found by the `indeterminate-ending-drops-the-per-kind-levers-under-keeps`
sweep for whole-escalation renders in geom-core, geom-brep and sweep.)

## What

`geom_brep::IsoFamilyRefusal::Undecided` (`pcurve_cache.rs`, its
`Display`) renders an `Indeterminate` whole through its bare `Display`
(`under(COINCIDENCE_RECOURSE)`), so it ends in the coincidence menu
with `IndeterminateUnder`'s ` — {reading}; {ending}` joint, and it is
not on the `too-close-to-call-remainder` list: "whether the edge runs
along a cylinder's ruling or around it is undecided: {diag}". Its one
consumer, `topo::JoinRefusal` (`boolean/edge_join.rs`), renders the
escalation itself under `JOIN_LEVER`, so this nested type's ending is
its caller's: by the parent row's rule it is a payload-only Display,
through `geom_core::undecided!`.

The other whole renders the sweep met are owned:

- `sweep::swept::sliver_text`'s `DihedralReading::Lever` arm ("{what}
  is neither a definite corner nor definitely smooth: {source}"), and
  its `Bend` arm's `COINCIDENCE_RECOURSE` at extrude and revolve
  (`SliverJoin`, `SliverRim`), by
  `work/carvetail/carve-refusals-short-of-the-shape-guard.md`
  ("Escalations that offer a declaration the door cannot take");
- `intersect.rs`'s `SectionError::Escalated` ("ill-conditioned at this
  tolerance") by
  `work/germ/plane-torus-section-degenerate-torus-ends-as-corruption.md`;
- `PcurveCertifyError::Escalated` by
  `work/pctail/pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled.md`;
- sweep's "too close to call" sites by `too-close-to-call-remainder`.

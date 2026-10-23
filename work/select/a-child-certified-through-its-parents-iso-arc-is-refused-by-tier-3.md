---
id: a-child-certified-through-its-parents-iso-arc-is-refused-by-tier-3
kind: issue
title: A child edge certified through its parent's stated IsoArc is refused by tier 3's builder path (MapResidual), so the op admits an edge tier 3 refuses
status: closed
opened: 2026-10-15
priority: P2
cost: M
refs: [an-iso-arc-certificate-never-checks-the-edge-span-lies-in-its-arc, onpair-has-no-incidence-lane-on-a-spline-adjacent-pair]
closed: 2026-10-23
---


Raised in the post-merge re-review of #5198 (m1, re-probed on head
0831a7e467, the merge of main).

A child edge spanning `[0, π/4]` of a plane × spline arc rim certifies
`Ok` when it is certified through its parent's stated `IsoArc`
(`EdgeCurve::certify_via_stated`, `crates/geom-brep/src/certify.rs:1580`).
Tier 3's builder path re-derives the same child's image instead, and
refuses it: `CertifyError::OnPairArcRim { iso: Some(MapResidual), limb:
ImageMiss, bound: 3.07e-5 }` (`crates/geom-brep/src/certify.rs:577`,
read at `crates/topo/src/validate.rs:2790`). So the op can admit an edge
that tier 3 then refuses: the two paths disagree on one edge.

Latent today: the plane × spline doors still state `Chart(plane)`
(`onpair-has-no-incidence-lane-on-a-spline-adjacent-pair`), so no build
mints such a child through the stated image yet. It turns live the day
the doors switch to the pair, and has to be settled before then: either
tier 3 reads the stated image the op certified against, or the stated
path refuses what tier 3 would (the sibling item
`an-iso-arc-certificate-never-checks-the-edge-span-lies-in-its-arc` is
the span half of the same gap; fix them together if one change closes
both).

Witness: the #5198 review lane's probe (local, not committed): split the
π/2 arc rim's edge at π/4, keep the parent's stated `IsoArc` on the
child, certify via the stated image, then run tier 3 on the result.

## Closed

`iso/arc-rim-sub-arc`, as a matter of course. The two paths disagreed
because the pick (`geom_brep::pick_arc_rim`) offered a child only
images mapping its own interval onto the whole row. It now follows that
closed form with the row over the arc the row measures
(`arc_rim_measured`), which for a split child is the parent's row and
arc, and the certificate decides the child's span lies in that arc (the
sibling item, closed by the same PR). On the witness, the `[0, π/4]`
child of the quarter rim, the derived path (`EdgeCurve::certify_via`,
nothing stated) now certifies through the same row the stated path
does, to the same residual: `geom-brep`'s
`onpair_plane_nurbs::a_split_child_recertifies_through_its_parents_stored_image`.
Tier 3 already reads the stored image (`recertify_via_stated`), so rest
and op read one image either way.

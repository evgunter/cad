---
id: set-edge-curve-keeps-a-certified-edges-rows-across-a-reparameterization
kind: issue
title: set_edge_curve keeps a certified edge's rows across a re-parameterization, so a later site mint keeps an image over an interval its edge does not span
status: closed
opened: 2026-10-06
closed: 2026-10-06
priority: P3
refs: [kev-describing-leaves-a-re-described-certified-members-far-face-rows-stale]
cost: E
---


Found by the lane that closed
`kev-describing-leaves-a-re-described-certified-members-far-face-rows-stale`,
checking whether the `debug_assert!` PR 4039 considered (a kept
image's interval is its edge's) now holds everywhere. It does not.

`Body::set_edge_curve` (`crates/topo/src/attach.rs`) keeps a certified
edge's rows (the `Completes` posture,
`pcurves::staleness_posture`; `Remints::FirstDescription` in
`Body::description_rows`). A re-description that certifies against the
same endpoints but parameterizes the carrier differently changes the
edge's interval, and the kept rows then span the old one. Tier 3 reads
that at rest (`RowInterval` and `Certify`/`MapResidual` on both
halves). Since PR 4039 the site mint keeps every image the door does
not create (`pcurves::site_rows`), so a later `mev`/`mef` on the face
walks the stale image as found, and its old interval enters the joints
that operator decides beside it.

## Recipe

In `crates/topo/tests/euler_site_pcurve_rows.rs`' terms: `wall()`
(bottom rim split at `UM = 0.8`); `set_edge_curve` on the rim piece
`(0.2, 0.8)` with the same circle under `u_ref` rotated by `+0.1` rad,
`arc_of_circle(carrier, 0.1, 0.7)`; `validate_pcurves` reports
`RowInterval` + `Certify { MapResidual }` on both halves. Then
`strut(&mut body, face, m)`: with the assertion below in `site_rows`'s
kept-image arm, the strut's site mint panics on the rim's wall half
(reproduced, 2026-10-06).

## The assertion this blocks

```rust
debug_assert!(
    !matches!(
        geom_core::k_stats::detached(|| {
            let (carrier, ..) = traversal(SiteHalf::Existing(he));
            row_interval(body, he, cache, &carrier, band)
        })
        .0,
        Err(PcurveMintError::RowInterval { .. })
    ),
    "site_rows: the image kept for {he:?} spans an interval its edge does not"
);
```

It reads tier 3's own `row_interval` (metres, at the band; no bit
comparison), detached so a debug-only check records no decision. With
it, the `topo`/`sweep` suites and every `join*` battery pass; only this
recipe fires it.

## Closing it

`set_edge_curve` re-mints a certified edge's faces where its write
moves the carrier or interval (`Remints::Every`, as `kev_describing`
now does), which changes `Completes`' declaration for
`set_edge_curve`; then the assertion lands. Or the posture stays, in
which case the assertion cannot.

## Closed

Closed by the first option, applied to every certified description.
`Body::set_edge_curve` (`crates/topo/src/attach.rs`) plans every
description through `Body::description_rows`, the plan `kev_describing`
uses, through the one predicate `Body::description_remints`. A certified
edge's face on a spline chart is left as found. With both doors
re-minting every described edge's faces, `Remints` had one case left and
is retired.

A measured rule (re-mint only where sampled points moved) was tried
first and dropped. It sampled five points, and a spline carrier on the
same locus and interval can move between them: the rim witness below
moves 2 mm between its quarters and kept its stale rows. The measured
rule was adopted because re-minting every description refused a sound
boolean (`sweep/tests/carved_sphere_operand.rs`, both rows,
`Pcurve { Unminted }`). That refusal did not come from the closing mint
being unable to re-image the sphere's general circle; it can. It came
from the boolean's two containment-fallback finishes
(`boolean/ops.rs` `fallback`'s assembly/voided arm and `finish_fallback`),
which re-describe and go to the gate with no closing mint. Both now
close with `mint_pcurves`, as the boolean's main output stage does.

`replace_faces_offset` (`crates/topo/src/replace_face.rs`) now drops the
rows of every edge that ends at a vertex it moves, before its re-anchors'
site mints. The closing `mint_pcurves` re-derives them. The doubt about a
vertex the move leaves in place is filed as
`work/shell/replace-faces-offset-drops-rows-at-a-vertex-the-move-leaves-in-place`.

The `debug_assert!` above is in `pcurves::site_rows`' kept-image arm.
The witnesses are
`euler_site_pcurve_rows::a_re_parameterized_certified_edge_re_mints_its_faces`,
which is this row's recipe, and
`reach_split_gate_per_face::a_spline_rim_moved_between_its_quarters_keeps_no_row`.
The sibling door `set_face_surfaces_describing` is filed as
`set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart`.

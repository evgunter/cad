---
id: set-edge-curve-keeps-a-certified-edges-rows-across-a-reparameterization
kind: issue
title: set_edge_curve keeps a certified edge's rows across a re-parameterization, so a later site mint keeps an image over an interval its edge does not span
status: open
opened: 2026-10-06
priority: P3
refs: [kev-describing-leaves-a-re-described-certified-members-far-face-rows-stale]
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

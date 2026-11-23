---
id: a-certified-tube-and-the-idealized-step-are-sized-by-a-levers-length
kind: issue
title: A certified tube's radius ladder and the idealized step are sized by a lever's length
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [sizes-are-read-back-out-of-extent-levers, diameter-levers-of-a-datum-read-at-a-point]
---


A lever read back as a length is a margin's business (`geom_core::Lever`'s doc: `Lever::length` is "the exit, not an entrance"). Two SSI quantities that size a domain are read from one:

- **The certified tube's radius ladder.** `geom_brep::edge_nurbs` hands `extent.length()` (and `edge_extent_inf(..).length()`) to `certify_branch` at three sites (near lines 1200, 1865 and 2152). There, `let widest = Bounds::hi(extent)` (`ssi/certify.rs`, near 2589) feeds `tube_ladder(widest, band)`, whose rung `k` is the radius `SSI_TUBE_RADIUS_MAX · extent / 2^k` (`tube_ladder`, near 172). So the edge's extent lever is the scale of the tube the certificate proves.
- **The idealized step.** `SsiDomain::extent`, the "lever arm of last resort" field, also sizes the march's idealized step on the chart lane: `(SSI_IDEALIZED_STEP * ctx.extent) / speed` (`ssi/march.rs`, near 1320). `topo::offset_derive`'s plane × wall and pair sections fill it with `extent.length()`.

Every rung is proved, so nothing is unsound. These are "a domain's size" in the words of `work/oneround/sizes-are-read-back-out-of-extent-levers.md`, whose unit (PR 7628) listed them as a margin's business and corrected that on review.

**Wanted:** the tube's scale and the step read from the geometry they size: the edge's own region over its span (`edge_cover`, `Reach::of_span`), or the chart window. The lever stays for the certificate's margins.

The ONEROUND unit `reach-from-the-read-point` (`work/oneround/diameter-levers-of-a-datum-read-at-a-point.md`) leaves the same `edge_nurbs` and `ssi/certify.rs` sites for a later unit on the spline certificate's per-sample levers. That unit and this row are about the same calls, so whichever lands first should say so on the other.

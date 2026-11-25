---
id: a-one-band-balls-seam-certificate-is-not-the-fresh-runs
kind: issue
title: A one-band ball's seam meridian carries a certificate a fresh re-certification does not reproduce
status: closed
opened: 2026-11-23
priority: P1
cost: E
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-23
pr: 7414
---


Found by the LATHE one-site lane (2026-11-23), measured on its branch
`lathe/one-site-wrap-edges` (stacked on `lathe/full-revolve-one-band`).

## What

The bare one-band ball `revolved_about_y([(0,-1),1.0),((0,1),0.0)], Full)`
(`sweep`'s snowman `ball(1, 0)`): re-certify its seam meridian (edge
`1v1`) against its own ends and surfaces through
`sweep/tests/common/certificates.rs` `assert_certificates_fresh` at
`Tol::witness()`: it stores `Certificate { samples: 9, max_residual:
1.5700924586837752e-16 }`, a fresh run mints `1.2246467991473532e-16`.
So `snowman::the_fallback_assembly_carries_the_kept_operands_certificates`
fails its "slab ∪ ball" row (`fresh: true`) once the lens it builds first
no longer refuses.

Same shape as
`work/cleavetail/a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce.md`,
now on a revolve's own seam rather than a boolean's.

## Ground

`crates/sweep/src/revolve/full.rs`, the one-band build's meridian mint,
or the test's `fresh` flag for the ball if the build's certificate is
the right one to keep.

## Cause

The revolve restated the meridian once, as `EdgeDescriptionSpec::wrap(wall)`
with no image, so the door derived the chart image and certified over
the derived one: `1.5700924586837752e-16`. The edge stores that image,
and a run over it stated (`recertify`, or `set_edge_curve` with the
stored edge's `restated_spec`) mints `1.2246467991473532e-16`. Same
carrier, interval, ends and surface; the endpoints do not move after
the restatement (checked at `upgrade_meridian_wrap` and after the
revolve's finish). So a run over a derived image and a run over the same
image stated differ in the last bits. That is a geom-brep certify
property, not the revolve's, and it may be the cleavetail row's cause
too (`work/cleavetail/a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce.md`,
"Cause: not established"); not checked there.

## Closed

2026-11-23, on PR 7414 (`lathe/full-revolve-one-band`).

`crates/sweep/src/revolve/upgrade.rs` `upgrade_meridian_wrap` restates
the meridian once more from what it stores, through
`restate_stored_edge` (the census door for a stored edge), so the
stored certificate is the run a fresh re-certification mints.
`snowman::the_fallback_assembly_carries_the_kept_operands_certificates`
passes on `lathe/full-revolve-one-band`.

- 2026-11-23 — cause traced and fixed on PR 7414's branch (LATHE one-band lane).
